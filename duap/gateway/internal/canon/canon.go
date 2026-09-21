// Package canon implements the DUAP canonical data model and deterministic
// CBOR codec.
//
// STATUS: PRODUCTION (independent implementation).
//
// This package was written against PROTOCOL.md and specs/protocol-v0.1.md,
// not by translating the Rust reference implementation. That is deliberate:
// the point of the exercise is to find out whether the specification is
// sufficient on its own. Places where the specification had to be consulted
// twice, or where the Rust source had to be checked to resolve an ambiguity,
// are recorded in docs/reviews/vertical-slice-review.md.
package canon

import (
	"bytes"
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"errors"
	"fmt"
	"sort"
	"strconv"
	"strings"
	"unicode/utf8"
)

// Kind enumerates the DUAP canonical data model.
type Kind int

const (
	KindNull Kind = iota
	KindBool
	KindUint
	KindNint // value is -1 - N
	KindBytes
	KindText
	KindArray
	KindMap
)

// Value is a value in the DUAP canonical data model.
//
// The model deliberately excludes floats, tags, indefinite lengths,
// non-text map keys and duplicate keys; see PROTOCOL.md section 3.
type Value struct {
	Kind  Kind
	B     bool
	U     uint64 // Uint payload, or Nint payload N
	Bytes []byte
	Text  string
	Array []Value
	Map   map[string]Value
}

func Null() Value          { return Value{Kind: KindNull} }
func Bool(b bool) Value    { return Value{Kind: KindBool, B: b} }
func Uint(u uint64) Value  { return Value{Kind: KindUint, U: u} }
func Nint(n uint64) Value  { return Value{Kind: KindNint, U: n} }
func Bstr(b []byte) Value  { return Value{Kind: KindBytes, Bytes: b} }
func Tstr(s string) Value  { return Value{Kind: KindText, Text: s} }
func Arr(v ...Value) Value { return Value{Kind: KindArray, Array: v} }
func Mp(m map[string]Value) Value {
	return Value{Kind: KindMap, Map: m}
}

// Int builds an integer value from a signed integer.
func Int(i int64) Value {
	if i >= 0 {
		return Uint(uint64(i))
	}
	return Nint(uint64(-1 - i))
}

// ---------------------------------------------------------------------------
// Encoding
// ---------------------------------------------------------------------------

const (
	mtUint   = 0
	mtNint   = 1
	mtBytes  = 2
	mtText   = 3
	mtArray  = 4
	mtMap    = 5
	mtSimple = 7
)

func head(buf *bytes.Buffer, mt byte, arg uint64) {
	m := mt << 5
	switch {
	case arg <= 23:
		buf.WriteByte(m | byte(arg))
	case arg <= 0xff:
		buf.WriteByte(m | 24)
		buf.WriteByte(byte(arg))
	case arg <= 0xffff:
		buf.WriteByte(m | 25)
		buf.Write([]byte{byte(arg >> 8), byte(arg)})
	case arg <= 0xffffffff:
		buf.WriteByte(m | 26)
		buf.Write([]byte{byte(arg >> 24), byte(arg >> 16), byte(arg >> 8), byte(arg)})
	default:
		buf.WriteByte(m | 27)
		for i := 7; i >= 0; i-- {
			buf.WriteByte(byte(arg >> (8 * uint(i))))
		}
	}
}

// Encode produces the deterministic CBOR encoding of v.
func Encode(v Value) []byte {
	var buf bytes.Buffer
	encodeInto(&buf, v)
	return buf.Bytes()
}

func encodeInto(buf *bytes.Buffer, v Value) {
	switch v.Kind {
	case KindNull:
		buf.WriteByte(0xf6)
	case KindBool:
		if v.B {
			buf.WriteByte(0xf5)
		} else {
			buf.WriteByte(0xf4)
		}
	case KindUint:
		head(buf, mtUint, v.U)
	case KindNint:
		head(buf, mtNint, v.U)
	case KindBytes:
		head(buf, mtBytes, uint64(len(v.Bytes)))
		buf.Write(v.Bytes)
	case KindText:
		head(buf, mtText, uint64(len(v.Text)))
		buf.WriteString(v.Text)
	case KindArray:
		head(buf, mtArray, uint64(len(v.Array)))
		for _, e := range v.Array {
			encodeInto(buf, e)
		}
	case KindMap:
		head(buf, mtMap, uint64(len(v.Map)))
		keys := make([]string, 0, len(v.Map))
		for k := range v.Map {
			keys = append(keys, k)
		}
		// RFC 8949 4.2.1: sort by encoded key bytes, which for text keys is
		// length first, then bytewise.
		sort.Slice(keys, func(i, j int) bool {
			if len(keys[i]) != len(keys[j]) {
				return len(keys[i]) < len(keys[j])
			}
			return keys[i] < keys[j]
		})
		for _, k := range keys {
			head(buf, mtText, uint64(len(k)))
			buf.WriteString(k)
			encodeInto(buf, v.Map[k])
		}
	}
}

// ---------------------------------------------------------------------------
// Decoding
// ---------------------------------------------------------------------------

const (
	maxDepth         = 64
	maxCollectionLen = 1 << 20
	// maxNintPayload bounds negative integers at i64::MIN.
	maxNintPayload = uint64(1)<<63 - 1
)

type decoder struct {
	b   []byte
	pos int
}

// Decode parses deterministic CBOR, rejecting anything Encode would not emit.
func Decode(b []byte) (Value, error) {
	d := &decoder{b: b}
	v, err := d.value(0)
	if err != nil {
		return Value{}, err
	}
	if d.pos != len(b) {
		return Value{}, fmt.Errorf("trailing data: consumed %d of %d bytes", d.pos, len(b))
	}
	return v, nil
}

// IsCanonical reports whether b is exactly the canonical encoding of a value.
func IsCanonical(b []byte) bool {
	_, err := Decode(b)
	return err == nil
}

func (d *decoder) take(n int) ([]byte, error) {
	if d.pos+n > len(d.b) {
		return nil, fmt.Errorf("unexpected end of input at %d, wanted %d bytes", d.pos, n)
	}
	s := d.b[d.pos : d.pos+n]
	d.pos += n
	return s, nil
}

func (d *decoder) head() (byte, uint64, error) {
	if d.pos >= len(d.b) {
		return 0, 0, errors.New("unexpected end of input")
	}
	ib := d.b[d.pos]
	d.pos++
	mt := ib >> 5
	ai := ib & 0x1f
	switch {
	case ai <= 23:
		return mt, uint64(ai), nil
	case ai == 24:
		s, err := d.take(1)
		if err != nil {
			return 0, 0, err
		}
		if s[0] < 24 {
			return 0, 0, errors.New("argument not in shortest form")
		}
		return mt, uint64(s[0]), nil
	case ai == 25:
		s, err := d.take(2)
		if err != nil {
			return 0, 0, err
		}
		val := uint64(s[0])<<8 | uint64(s[1])
		if val <= 0xff {
			return 0, 0, errors.New("argument not in shortest form")
		}
		return mt, val, nil
	case ai == 26:
		s, err := d.take(4)
		if err != nil {
			return 0, 0, err
		}
		var val uint64
		for _, c := range s {
			val = val<<8 | uint64(c)
		}
		if val <= 0xffff {
			return 0, 0, errors.New("argument not in shortest form")
		}
		return mt, val, nil
	case ai == 27:
		s, err := d.take(8)
		if err != nil {
			return 0, 0, err
		}
		var val uint64
		for _, c := range s {
			val = val<<8 | uint64(c)
		}
		if val <= 0xffffffff {
			return 0, 0, errors.New("argument not in shortest form")
		}
		return mt, val, nil
	case ai <= 30:
		return 0, 0, errors.New("reserved additional information")
	default:
		return 0, 0, errors.New("indefinite length or break stop code")
	}
}

func (d *decoder) value(depth int) (Value, error) {
	if depth >= maxDepth {
		return Value{}, errors.New("nesting too deep")
	}
	mt, arg, err := d.head()
	if err != nil {
		return Value{}, err
	}
	switch mt {
	case mtUint:
		return Uint(arg), nil
	case mtNint:
		if arg > maxNintPayload {
			return Value{}, errors.New("negative integer below i64::MIN is outside the data model")
		}
		return Nint(arg), nil
	case mtBytes:
		n, err := d.collectionLen(arg, true)
		if err != nil {
			return Value{}, err
		}
		s, err := d.take(n)
		if err != nil {
			return Value{}, err
		}
		return Bstr(append([]byte(nil), s...)), nil
	case mtText:
		n, err := d.collectionLen(arg, true)
		if err != nil {
			return Value{}, err
		}
		s, err := d.take(n)
		if err != nil {
			return Value{}, err
		}
		if !utf8Valid(s) {
			return Value{}, errors.New("text string is not valid UTF-8")
		}
		return Tstr(string(s)), nil
	case mtArray:
		n, err := d.collectionLen(arg, false)
		if err != nil {
			return Value{}, err
		}
		items := make([]Value, 0, min(n, 1024))
		for i := 0; i < n; i++ {
			e, err := d.value(depth + 1)
			if err != nil {
				return Value{}, err
			}
			items = append(items, e)
		}
		return Arr(items...), nil
	case mtMap:
		n, err := d.collectionLen(arg, false)
		if err != nil {
			return Value{}, err
		}
		m := make(map[string]Value, n)
		prevLen := -1
		prevKey := ""
		for i := 0; i < n; i++ {
			kmt, karg, err := d.head()
			if err != nil {
				return Value{}, err
			}
			if kmt != mtText {
				return Value{}, errors.New("map key is not a text string")
			}
			kn, err := d.collectionLen(karg, true)
			if err != nil {
				return Value{}, err
			}
			ks, err := d.take(kn)
			if err != nil {
				return Value{}, err
			}
			if !utf8Valid(ks) {
				return Value{}, errors.New("map key is not valid UTF-8")
			}
			key := string(ks)
			if prevLen >= 0 {
				if len(key) < prevLen || (len(key) == prevLen && key < prevKey) {
					return Value{}, errors.New("map keys are not in canonical order")
				}
				if len(key) == prevLen && key == prevKey {
					return Value{}, errors.New("duplicate map key")
				}
			}
			prevLen, prevKey = len(key), key
			val, err := d.value(depth + 1)
			if err != nil {
				return Value{}, err
			}
			if _, dup := m[key]; dup {
				return Value{}, errors.New("duplicate map key")
			}
			m[key] = val
		}
		return Mp(m), nil
	case 6:
		return Value{}, errors.New("CBOR tags are outside the data model")
	case mtSimple:
		switch arg {
		case 20:
			return Bool(false), nil
		case 21:
			return Bool(true), nil
		case 22:
			return Null(), nil
		case 23:
			return Value{}, errors.New("undefined is outside the data model")
		case 25, 26, 27:
			return Value{}, errors.New("floating point is outside the data model")
		default:
			return Value{}, errors.New("unassigned simple value")
		}
	}
	return Value{}, errors.New("unreachable major type")
}

func (d *decoder) collectionLen(arg uint64, isBytes bool) (int, error) {
	if arg > uint64(maxCollectionLen) && !isBytes {
		return 0, errors.New("collection too large")
	}
	n := int(arg)
	if n < 0 || uint64(n) != arg {
		return 0, errors.New("length out of range")
	}
	if n > len(d.b)-d.pos {
		return 0, errors.New("length exceeds remaining input")
	}
	return n, nil
}

func utf8Valid(b []byte) bool { return utf8.Valid(b) }

func stringsValidUTF8(s string) bool {
	for _, r := range s {
		if r == 0xFFFD && !strings.Contains(s, "�") {
			return false
		}
	}
	return true
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// ---------------------------------------------------------------------------
// Digests
// ---------------------------------------------------------------------------

// DigestPrefix binds every digest to the protocol and major version.
const DigestPrefix = "DUAP/1"

// Digest is a self-describing content digest.
type Digest struct {
	Alg   string
	Bytes [32]byte
}

func (d Digest) String() string {
	return d.Alg + ":" + hex.EncodeToString(d.Bytes[:])
}

// ParseDigest reads the "alg:hex" form.
func ParseDigest(s string) (Digest, error) {
	i := strings.IndexByte(s, ':')
	if i < 0 {
		return Digest{}, fmt.Errorf("missing ':' in digest %q", s)
	}
	alg := s[:i]
	if alg != "sha2-256" && alg != "blake3-256" {
		return Digest{}, fmt.Errorf("unknown hash algorithm %q", alg)
	}
	raw, err := hex.DecodeString(s[i+1:])
	if err != nil || len(raw) != 32 {
		return Digest{}, fmt.Errorf("bad digest hex in %q", s)
	}
	var d Digest
	d.Alg = alg
	copy(d.Bytes[:], raw)
	return d, nil
}

// DigestOf computes H(prefix || 0 || domain || 0 || payload).
//
// Only sha2-256 is implemented here; blake3 is optional in the protocol and
// this verifier does not claim it.
func DigestOf(domain string, payload []byte) Digest {
	h := sha256.New()
	h.Write([]byte(DigestPrefix))
	h.Write([]byte{0})
	h.Write([]byte(domain))
	h.Write([]byte{0})
	h.Write(payload)
	var d Digest
	d.Alg = "sha2-256"
	copy(d.Bytes[:], h.Sum(nil))
	return d
}

// ---------------------------------------------------------------------------
// JSON view
// ---------------------------------------------------------------------------

const jsonSafeInt = uint64(1)<<53 - 1

// FromJSONView converts a decoded JSON document (as produced by
// encoding/json into any) into a canonical value.
func FromJSONView(x any) (Value, error) {
	switch t := x.(type) {
	case nil:
		return Null(), nil
	case bool:
		return Bool(t), nil
	case float64:
		// encoding/json gives numbers as float64; the model permits only
		// integers, and only inside the JSON-safe range.
		if t != float64(int64(t)) {
			return Value{}, fmt.Errorf("non-integer number %v is outside the data model", t)
		}
		return Int(int64(t)), nil
	case string:
		return Tstr(t), nil
	case []any:
		out := make([]Value, 0, len(t))
		for _, e := range t {
			v, err := FromJSONView(e)
			if err != nil {
				return Value{}, err
			}
			out = append(out, v)
		}
		return Arr(out...), nil
	case map[string]any:
		if len(t) == 1 {
			for k, raw := range t {
				s, isStr := raw.(string)
				switch k {
				case "$b64":
					if !isStr {
						return Value{}, errors.New("$b64 must carry a string")
					}
					b, err := base64.RawURLEncoding.DecodeString(s)
					if err != nil {
						return Value{}, fmt.Errorf("bad $b64: %w", err)
					}
					if base64.RawURLEncoding.EncodeToString(b) != s {
						return Value{}, errors.New("base64url is not in canonical unpadded form")
					}
					return Bstr(b), nil
				case "$u64":
					if !isStr {
						return Value{}, errors.New("$u64 must carry a string")
					}
					u, err := strconv.ParseUint(s, 10, 64)
					if err != nil {
						return Value{}, fmt.Errorf("bad $u64: %w", err)
					}
					if u <= jsonSafeInt {
						return Value{}, errors.New("$u64 must not be used inside the JSON-safe range")
					}
					return Uint(u), nil
				case "$n64":
					if !isStr {
						return Value{}, errors.New("$n64 must carry a string")
					}
					// value <= -(2^53); N = -1 - value, which can reach 2^64-1.
					if !strings.HasPrefix(s, "-") {
						return Value{}, errors.New("$n64 must be negative")
					}
					mag, err := strconv.ParseUint(s[1:], 10, 64)
					if err != nil {
						return Value{}, fmt.Errorf("bad $n64: %w", err)
					}
					if mag <= jsonSafeInt {
						return Value{}, errors.New("$n64 must not be used inside the JSON-safe range")
					}
					// The model bounds negative values at i64::MIN, so the
					// magnitude is at most 2^63 and the payload fits u64.
					if mag > 1<<63 {
						return Value{}, errors.New("$n64 is below i64::MIN, outside the data model")
					}
					return Nint(mag - 1), nil
				}
			}
		}
		m := make(map[string]Value, len(t))
		for k, raw := range t {
			key := k
			if strings.HasPrefix(k, "$$") {
				key = k[1:]
			} else if strings.HasPrefix(k, "$") {
				return Value{}, fmt.Errorf("object key %q uses the reserved $ prefix", k)
			}
			v, err := FromJSONView(raw)
			if err != nil {
				return Value{}, err
			}
			if _, dup := m[key]; dup {
				return Value{}, fmt.Errorf("duplicate key %q", key)
			}
			m[key] = v
		}
		return Mp(m), nil
	}
	return Value{}, fmt.Errorf("unsupported JSON node %T", x)
}

// Get looks up a map field.
func (v Value) Get(key string) (Value, bool) {
	if v.Kind != KindMap {
		return Value{}, false
	}
	x, ok := v.Map[key]
	return x, ok
}

// Str returns the text of a text value.
func (v Value) Str() (string, bool) {
	if v.Kind != KindText {
		return "", false
	}
	return v.Text, true
}

// U64 returns the value of an unsigned integer value.
func (v Value) U64() (uint64, bool) {
	if v.Kind != KindUint {
		return 0, false
	}
	return v.U, true
}

// Bin returns the bytes of a byte string.
func (v Value) Bin() ([]byte, bool) {
	if v.Kind != KindBytes {
		return nil, false
	}
	return v.Bytes, true
}
