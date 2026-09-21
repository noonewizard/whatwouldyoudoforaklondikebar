// Package duapcrypto implements the DUAP signature envelope for the suites
// this verifier claims: ed25519 only.
//
// STATUS: PRODUCTION (independent implementation, conformance level L2).
//
// ML-DSA is deliberately not implemented here. A verifier that claimed a
// post-quantum suite it cannot actually check would be worse than one that
// says plainly which suites it supports; PROTOCOL.md requires a verifier to
// reject a suite it does not implement rather than accept it.
package duapcrypto

import (
	"crypto/ed25519"
	"encoding/binary"
	"errors"
	"fmt"

	"github.com/duap/gateway/internal/canon"
)

// SigContext is the context string mixed into every DUAP signature.
var SigContext = []byte("DUAP/1 signature")

// SigInputDomain is the domain label of the signature input structure.
const SigInputDomain = "duap.sig-input.v1"

// KeyIDDomain is the domain label for key identifier derivation.
const KeyIDDomain = "duap.key-id.v1"

// KeyID derives the self-certifying identifier for a suite and public key.
func KeyID(suite string, pub []byte) string {
	buf := make([]byte, 0, len(suite)+1+len(pub))
	buf = append(buf, suite...)
	buf = append(buf, 0)
	buf = append(buf, pub...)
	d := canon.DigestOf(KeyIDDomain, buf)
	return "kid1:" + hexLower(d.Bytes[:16])
}

func hexLower(b []byte) string {
	const digits = "0123456789abcdef"
	out := make([]byte, len(b)*2)
	for i, c := range b {
		out[i*2] = digits[c>>4]
		out[i*2+1] = digits[c&0x0f]
	}
	return string(out)
}

// Signature is one signature over an envelope payload.
type Signature struct {
	Suite   string
	KeyID   string
	Created uint64
	Nonce   []byte
	HasNonce bool
	Sig     []byte
}

// Envelope is a payload plus the signatures over it.
type Envelope struct {
	Domain     string
	Payload    []byte
	Signatures []Signature
}

// ParseEnvelope decodes a canonical envelope.
func ParseEnvelope(b []byte) (Envelope, error) {
	v, err := canon.Decode(b)
	if err != nil {
		return Envelope{}, fmt.Errorf("envelope is not canonical: %w", err)
	}
	var e Envelope
	dv, ok := v.Get("domain")
	if !ok {
		return Envelope{}, errors.New("envelope has no domain")
	}
	if e.Domain, ok = dv.Str(); !ok {
		return Envelope{}, errors.New("envelope domain is not text")
	}
	pv, ok := v.Get("payload")
	if !ok {
		return Envelope{}, errors.New("envelope has no payload")
	}
	if e.Payload, ok = pv.Bin(); !ok {
		return Envelope{}, errors.New("envelope payload is not a byte string")
	}
	sv, ok := v.Get("signatures")
	if !ok || sv.Kind != canon.KindArray {
		return Envelope{}, errors.New("envelope has no signature array")
	}
	for _, s := range sv.Array {
		var sig Signature
		if x, ok := s.Get("suite"); ok {
			sig.Suite, _ = x.Str()
		}
		if x, ok := s.Get("kid"); ok {
			sig.KeyID, _ = x.Str()
		}
		if x, ok := s.Get("created"); ok {
			sig.Created, _ = x.U64()
		}
		if x, ok := s.Get("nonce"); ok && x.Kind == canon.KindBytes {
			sig.Nonce, sig.HasNonce = x.Bytes, true
		}
		if x, ok := s.Get("sig"); ok {
			sig.Sig, _ = x.Bin()
		}
		e.Signatures = append(e.Signatures, sig)
	}
	return e, nil
}

// PayloadDigest is the digest of the payload under the envelope's domain.
func (e Envelope) PayloadDigest() canon.Digest {
	return canon.DigestOf(e.Domain, e.Payload)
}

// SigningInput reconstructs the canonical bytes that were signed.
//
// The structure is documented in PROTOCOL.md section 5.2: a map with the
// fields c (context), s (suite), k (key id), d (payload domain), h (payload
// digest), t (created) and optionally n (nonce).
func SigningInput(domain string, payloadDigest canon.Digest, s Signature) []byte {
	m := map[string]canon.Value{
		"c": canon.Tstr("duap.sig.v1"),
		"s": canon.Tstr(s.Suite),
		"k": canon.Tstr(s.KeyID),
		"d": canon.Tstr(domain),
		"h": canon.Tstr(payloadDigest.String()),
		"t": canon.Uint(s.Created),
	}
	if s.HasNonce {
		m["n"] = canon.Bstr(s.Nonce)
	}
	return canon.Encode(canon.Mp(m))
}

// bindContext length-prefixes the context for suites with no native context
// parameter (PROTOCOL.md section 5.3).
func bindContext(ctx, msg []byte) []byte {
	out := make([]byte, 0, 8+len(ctx)+len(msg))
	var n [8]byte
	binary.BigEndian.PutUint64(n[:], uint64(len(ctx)))
	out = append(out, n[:]...)
	out = append(out, ctx...)
	out = append(out, msg...)
	return out
}

// VerifyWithKey checks one signature against an explicit public key.
func (e Envelope) VerifyWithKey(s Signature, suite string, pub []byte) error {
	if s.Suite != suite {
		return fmt.Errorf("signature suite %q does not match the key's %q", s.Suite, suite)
	}
	if suite != "ed25519" {
		return fmt.Errorf("suite %q is not implemented by this verifier", suite)
	}
	if len(pub) != ed25519.PublicKeySize {
		return fmt.Errorf("ed25519 public key must be %d bytes", ed25519.PublicKeySize)
	}
	if want := KeyID(suite, pub); want != s.KeyID {
		return fmt.Errorf("key id mismatch: signature names %s, key derives %s", s.KeyID, want)
	}
	if len(s.Sig) != ed25519.SignatureSize {
		return fmt.Errorf("ed25519 signature must be %d bytes", ed25519.SignatureSize)
	}
	input := SigningInput(e.Domain, e.PayloadDigest(), s)
	msg := bindContext(SigContext, input)
	if !ed25519.Verify(ed25519.PublicKey(pub), msg, s.Sig) {
		return errors.New("ed25519 verification failed")
	}
	return nil
}
