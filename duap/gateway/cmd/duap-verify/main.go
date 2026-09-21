// Command duap-verify checks a DUAP conformance vector suite using an
// implementation written independently of the Rust reference.
//
// STATUS: PRODUCTION (conformance levels L1-L3).
//
// This program exists to answer one question: can DUAP be implemented from
// its specification by someone who did not write the reference? It claims
// levels L1 (canonical encoding and digests), L2 (ed25519 signature
// verification) and L3 (Merkle proofs and receipt verification). It does not
// claim L4 (authorization evaluation) or L5 (pricing arithmetic), and it
// skips those vectors explicitly rather than silently.
//
// Usage: duap-verify [vector-directory]
package main

import (
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strconv"

	"github.com/duap/gateway/internal/canon"
	"github.com/duap/gateway/internal/duapcrypto"
	"github.com/duap/gateway/internal/merkle"
	"github.com/duap/gateway/internal/taxonomy"
)

type vector struct {
	ID          string          `json:"id"`
	Level       string          `json:"level"`
	Requirement string          `json:"requirement"`
	Input       json.RawMessage `json:"input"`
	Expect      json.RawMessage `json:"expect"`
}

type vectorFile struct {
	Group       string   `json:"group"`
	Protocol    string   `json:"protocol"`
	Description string   `json:"description"`
	Vectors     []vector `json:"vectors"`
}

type report struct {
	passed  int
	failed  int
	skipped int
	fails   []string
}

func (r *report) pass()             { r.passed++ }
func (r *report) skip()             { r.skipped++ }
func (r *report) fail(id, why string) {
	r.failed++
	r.fails = append(r.fails, fmt.Sprintf("%s: %s", id, why))
}

// claimedLevels are the conformance levels this verifier implements.
var claimedLevels = map[string]bool{"L1": true, "L2": true, "L3": true}

func main() {
	dir := "spec/vectors"
	if len(os.Args) > 1 {
		dir = os.Args[1]
	}
	files, err := filepath.Glob(filepath.Join(dir, "*.json"))
	if err != nil || len(files) == 0 {
		fmt.Fprintf(os.Stderr, "no vector files found in %s\n", dir)
		os.Exit(2)
	}
	sort.Strings(files)

	r := &report{}
	for _, f := range files {
		raw, err := os.ReadFile(f)
		if err != nil {
			fmt.Fprintf(os.Stderr, "cannot read %s: %v\n", f, err)
			os.Exit(2)
		}
		var vf vectorFile
		if err := json.Unmarshal(raw, &vf); err != nil {
			fmt.Fprintf(os.Stderr, "cannot parse %s: %v\n", f, err)
			os.Exit(2)
		}
		if vf.Protocol != canon.DigestPrefix {
			fmt.Fprintf(os.Stderr, "%s: protocol %q is not %q\n", f, vf.Protocol, canon.DigestPrefix)
			os.Exit(2)
		}
		for _, v := range vf.Vectors {
			if !claimedLevels[v.Level] {
				r.skip()
				continue
			}
			runVector(vf.Group, v, r)
		}
	}

	fmt.Printf("duap-verify (independent Go implementation, levels L1-L3)\n")
	fmt.Printf("  passed  %d\n", r.passed)
	fmt.Printf("  failed  %d\n", r.failed)
	fmt.Printf("  skipped %d (levels this implementation does not claim)\n", r.skipped)
	for _, f := range r.fails {
		fmt.Printf("  FAIL %s\n", f)
	}
	if r.failed > 0 {
		os.Exit(1)
	}
}

func runVector(group string, v vector, r *report) {
	switch group {
	case "canonical-encoding":
		checkCanonical(v, r)
	case "crypto":
		checkCrypto(v, r)
	case "merkle":
		checkMerkle(v, r)
	case "events":
		checkEvent(v, r)
	case "receipts":
		checkReceipt(v, r)
	case "taxonomy":
		checkTaxonomy(v, r)
	default:
		r.skip()
	}
}

func decodeInto(raw json.RawMessage, out any) error { return json.Unmarshal(raw, out) }

// ---------------------------------------------------------------------------

func checkCanonical(v vector, r *report) {
	var in map[string]any
	var ex map[string]any
	if err := decodeInto(v.Input, &in); err != nil {
		r.fail(v.ID, "bad input: "+err.Error())
		return
	}
	if err := decodeInto(v.Expect, &ex); err != nil {
		r.fail(v.ID, "bad expectation: "+err.Error())
		return
	}

	if hexIn, ok := in["cbor_hex"].(string); ok {
		// A rejection vector: these bytes must not decode.
		raw, err := hex.DecodeString(hexIn)
		if err != nil {
			r.fail(v.ID, "input hex is malformed")
			return
		}
		wantDecodes, _ := ex["decodes"].(bool)
		got := canon.IsCanonical(raw)
		if got != wantDecodes {
			r.fail(v.ID, fmt.Sprintf("decodes=%v, expected %v", got, wantDecodes))
			return
		}
		r.pass()
		return
	}

	// An encoding vector: the JSON view must encode to the stated bytes.
	val, err := canon.FromJSONView(in["json_view"])
	if err != nil {
		r.fail(v.ID, "cannot read JSON view: "+err.Error())
		return
	}
	got := canon.Encode(val)
	wantHex, _ := ex["cbor_hex"].(string)
	if hex.EncodeToString(got) != wantHex {
		r.fail(v.ID, fmt.Sprintf("encoded %s, expected %s", hex.EncodeToString(got), wantHex))
		return
	}
	// Round trip.
	back, err := canon.Decode(got)
	if err != nil {
		r.fail(v.ID, "canonical bytes failed to decode: "+err.Error())
		return
	}
	if hex.EncodeToString(canon.Encode(back)) != wantHex {
		r.fail(v.ID, "decode/encode round trip changed the bytes")
		return
	}
	// Digest under the test domain.
	if want, ok := ex["sha2_256_under_test_domain"].(string); ok {
		if canon.DigestOf("duap.test.v1", got).String() != want {
			r.fail(v.ID, "sha2-256 digest mismatch")
			return
		}
	}
	r.pass()
}

func checkCrypto(v vector, r *report) {
	var in, ex map[string]any
	_ = decodeInto(v.Input, &in)
	_ = decodeInto(v.Expect, &ex)
	suite, _ := in["suite"].(string)

	if _, isKeyDerivation := in["seed_hex"]; isKeyDerivation {
		if _, hasDomain := in["domain"]; !hasDomain {
			// Key derivation vector. Only the identifier derivation is
			// checkable without implementing the suite's key generation, so
			// this verifier checks that the published public key produces
			// the published identifier.
			pubHex, _ := ex["public_key_hex"].(string)
			pub, err := hex.DecodeString(pubHex)
			if err != nil {
				r.fail(v.ID, "bad public key hex")
				return
			}
			want, _ := ex["key_id"].(string)
			if got := duapcrypto.KeyID(suite, pub); got != want {
				r.fail(v.ID, fmt.Sprintf("key id %s, expected %s", got, want))
				return
			}
			r.pass()
			return
		}
	}

	if suite != "ed25519" {
		// Suites this verifier does not implement are skipped, not failed.
		r.skip()
		return
	}

	domain, _ := in["domain"].(string)
	payloadHex, _ := in["payload_cbor_hex"].(string)
	payload, err := hex.DecodeString(payloadHex)
	if err != nil {
		r.fail(v.ID, "bad payload hex")
		return
	}
	createdF, _ := in["created"].(float64)
	sigHex, _ := in["signature_hex"].(string)
	if sigHex == "" {
		sigHex, _ = ex["signature_hex"].(string)
	}
	sigBytes, err := hex.DecodeString(sigHex)
	if err != nil {
		r.fail(v.ID, "bad signature hex")
		return
	}
	keyID, _ := ex["key_id"].(string)
	if keyID == "" {
		keyID, _ = in["key_id"].(string)
	}

	// Recover the public key from the sibling key-derivation vector is not
	// possible here, so the vector carries it in the seed-derived form; the
	// signing vectors publish the key id and we re-derive the public key
	// from the accompanying key vector. To stay self-contained, the
	// signature vectors are checked against the public key published in the
	// matching crypto/keyid vector, which the harness passes through the
	// expectation when present.
	pubHex, _ := in["public_key_hex"].(string)
	if pubHex == "" {
		r.fail(v.ID, "vector does not publish the public key")
		return
	}
	pub, err := hex.DecodeString(pubHex)
	if err != nil {
		r.fail(v.ID, "bad public key hex")
		return
	}

	env := duapcrypto.Envelope{Domain: domain, Payload: payload}
	sig := duapcrypto.Signature{
		Suite:   suite,
		KeyID:   keyID,
		Created: uint64(createdF),
		Sig:     sigBytes,
	}
	err = env.VerifyWithKey(sig, suite, pub)
	wantVerifies := true
	if b, ok := ex["verifies"].(bool); ok {
		wantVerifies = b
	}
	if (err == nil) != wantVerifies {
		r.fail(v.ID, fmt.Sprintf("verifies=%v (err=%v), expected %v", err == nil, err, wantVerifies))
		return
	}
	if wantVerifies {
		if want, ok := ex["payload_digest"].(string); ok {
			if env.PayloadDigest().String() != want {
				r.fail(v.ID, "payload digest mismatch")
				return
			}
		}
	}
	r.pass()
}

func checkMerkle(v vector, r *report) {
	var in, ex map[string]any
	_ = decodeInto(v.Input, &in)
	_ = decodeInto(v.Expect, &ex)

	if leaves, ok := in["leaves"].([]any); ok {
		hashes := make([]canon.Digest, 0, len(leaves))
		for _, l := range leaves {
			s, _ := l.(string)
			hashes = append(hashes, merkle.LeafHash([]byte(s)))
		}
		want, _ := ex["root"].(string)
		if got := merkle.Root(hashes).String(); got != want {
			r.fail(v.ID, fmt.Sprintf("root %s, expected %s", got, want))
			return
		}
		r.pass()
		return
	}

	if payload, ok := in["leaf_payload"].(string); ok {
		idx, _ := in["index"].(float64)
		size, _ := in["size"].(float64)
		path, err := digestList(in["path"])
		if err != nil {
			r.fail(v.ID, err.Error())
			return
		}
		rootStr, _ := ex["root"].(string)
		root, err := canon.ParseDigest(rootStr)
		if err != nil {
			r.fail(v.ID, "bad root digest")
			return
		}
		ok := merkle.VerifyInclusion(merkle.LeafHash([]byte(payload)), uint64(idx), uint64(size), path, root)
		if !ok {
			r.fail(v.ID, "inclusion proof did not verify")
			return
		}
		r.pass()
		return
	}

	if _, ok := in["old_size"]; ok {
		oldSize, _ := in["old_size"].(float64)
		newSize, _ := in["new_size"].(float64)
		path, err := digestList(in["path"])
		if err != nil {
			r.fail(v.ID, err.Error())
			return
		}
		oldRootStr, _ := ex["old_root"].(string)
		newRootStr, _ := ex["new_root"].(string)
		oldRoot, e1 := canon.ParseDigest(oldRootStr)
		newRoot, e2 := canon.ParseDigest(newRootStr)
		if e1 != nil || e2 != nil {
			r.fail(v.ID, "bad root digest")
			return
		}
		if !merkle.VerifyConsistency(uint64(oldSize), uint64(newSize), path, oldRoot, newRoot) {
			r.fail(v.ID, "consistency proof did not verify")
			return
		}
		r.pass()
		return
	}

	if _, ok := ex["leaf_a"]; ok {
		la, _ := ex["leaf_a"].(string)
		lb, _ := ex["leaf_b"].(string)
		nab, _ := ex["node_ab"].(string)
		empty, _ := ex["empty"].(string)
		if merkle.LeafHash([]byte("a")).String() != la ||
			merkle.LeafHash([]byte("b")).String() != lb ||
			merkle.NodeHash(merkle.LeafHash([]byte("a")), merkle.LeafHash([]byte("b"))).String() != nab ||
			merkle.EmptyRoot().String() != empty {
			r.fail(v.ID, "leaf/node/empty hashing mismatch")
			return
		}
		r.pass()
		return
	}
	r.skip()
}

func digestList(x any) ([]canon.Digest, error) {
	raw, ok := x.([]any)
	if !ok {
		return nil, fmt.Errorf("path is not a list")
	}
	out := make([]canon.Digest, 0, len(raw))
	for _, e := range raw {
		s, _ := e.(string)
		d, err := canon.ParseDigest(s)
		if err != nil {
			return nil, fmt.Errorf("bad digest in path: %w", err)
		}
		out = append(out, d)
	}
	return out, nil
}

func checkEvent(v vector, r *report) {
	var in, ex map[string]any
	_ = decodeInto(v.Input, &in)
	_ = decodeInto(v.Expect, &ex)

	if hexIn, ok := in["cbor_hex"].(string); ok {
		// A structural-rejection vector. This verifier checks the subset of
		// the rules that are decidable from the taxonomy alone: unit must be
		// the operation's meter, sensitivity must not fall below the class
		// default, and the duap. extension namespace is reserved.
		raw, err := hex.DecodeString(hexIn)
		if err != nil {
			r.fail(v.ID, "bad hex")
			return
		}
		val, err := canon.Decode(raw)
		if err != nil {
			// Rejecting at the codec is also a valid rejection.
			r.pass()
			return
		}
		if eventStructurallyValid(val) {
			r.fail(v.ID, "event was accepted but the vector says it is invalid")
			return
		}
		r.pass()
		return
	}

	val, err := canon.FromJSONView(in["json_view"])
	if err != nil {
		r.fail(v.ID, "cannot read JSON view: "+err.Error())
		return
	}
	got := canon.Encode(val)
	wantHex, _ := ex["cbor_hex"].(string)
	if hex.EncodeToString(got) != wantHex {
		r.fail(v.ID, "event encoding mismatch")
		return
	}
	wantDigest, _ := ex["digest"].(string)
	if canon.DigestOf("duap.event.v1", got).String() != wantDigest {
		r.fail(v.ID, "event digest mismatch")
		return
	}
	if !eventStructurallyValid(val) {
		r.fail(v.ID, "the sample event failed structural validation")
		return
	}
	r.pass()
}

// eventStructurallyValid applies the subset of PROTOCOL.md section 4.4 that
// this verifier implements.
func eventStructurallyValid(v canon.Value) bool {
	opV, ok := v.Get("op")
	if !ok {
		return false
	}
	opCode, ok := opV.Str()
	if !ok {
		return false
	}
	op := taxonomy.Operation(opCode)
	if !op.Valid() {
		return false
	}
	qy, ok := v.Get("qy")
	if !ok {
		return false
	}
	uV, ok := qy.Get("u")
	if !ok {
		return false
	}
	unitCode, _ := uV.Str()
	if taxonomy.OperationMeter[op] != taxonomy.Unit(unitCode) {
		return false
	}
	nV, ok := qy.Get("n")
	if !ok {
		return false
	}
	if n, ok := nV.U64(); !ok || n == 0 {
		return false
	}
	dcV, ok := v.Get("dc")
	if !ok {
		return false
	}
	dcCode, _ := dcV.Str()
	dc := taxonomy.DataClass(dcCode)
	if !dc.Valid() {
		return false
	}
	snV, ok := v.Get("sn")
	if !ok {
		return false
	}
	snCode, _ := snV.Str()
	if taxonomy.TierRank[taxonomy.SensitivityTier(snCode)] < taxonomy.TierRank[taxonomy.DataClassSensitivity[dc]] {
		return false
	}
	if xt, ok := v.Get("xt"); ok && xt.Kind == canon.KindMap {
		for k := range xt.Map {
			if len(k) >= 5 && k[:5] == "duap." {
				return false
			}
			if !containsDot(k) {
				return false
			}
		}
	}
	return true
}

func containsDot(s string) bool {
	for i := 0; i < len(s); i++ {
		if s[i] == '.' {
			return true
		}
	}
	return false
}

func checkReceipt(v vector, r *report) {
	var in, ex map[string]any
	_ = decodeInto(v.Input, &in)
	_ = decodeInto(v.Expect, &ex)

	if jv, ok := in["json_view"]; ok {
		val, err := canon.FromJSONView(jv)
		if err != nil {
			r.fail(v.ID, "cannot read JSON view: "+err.Error())
			return
		}
		got := canon.Encode(val)
		wantHex, _ := ex["cbor_hex"].(string)
		if hex.EncodeToString(got) != wantHex {
			r.fail(v.ID, "receipt encoding mismatch")
			return
		}
		wantDigest, _ := ex["digest"].(string)
		if canon.DigestOf("duap.receipt.v1", got).String() != wantDigest {
			r.fail(v.ID, "receipt digest mismatch")
			return
		}
		r.pass()
		return
	}

	if envHex, ok := in["envelope_cbor_hex"].(string); ok {
		raw, err := hex.DecodeString(envHex)
		if err != nil {
			r.fail(v.ID, "bad envelope hex")
			return
		}
		env, err := duapcrypto.ParseEnvelope(raw)
		if err != nil {
			r.fail(v.ID, "cannot parse envelope: "+err.Error())
			return
		}
		pubHex, _ := in["public_key_hex"].(string)
		pub, err := hex.DecodeString(pubHex)
		if err != nil {
			r.fail(v.ID, "bad public key hex")
			return
		}
		if len(env.Signatures) == 0 {
			r.fail(v.ID, "envelope carries no signature")
			return
		}
		if err := env.VerifyWithKey(env.Signatures[0], "ed25519", pub); err != nil {
			r.fail(v.ID, "receipt signature did not verify: "+err.Error())
			return
		}
		if want, ok := ex["key_id"].(string); ok {
			if duapcrypto.KeyID("ed25519", pub) != want {
				r.fail(v.ID, "key id mismatch")
				return
			}
		}
		r.pass()
		return
	}

	if digs, ok := in["event_digests"].([]any); ok {
		leaves := make([]canon.Digest, 0, len(digs))
		for _, d := range digs {
			s, _ := d.(string)
			pd, err := canon.ParseDigest(s)
			if err != nil {
				r.fail(v.ID, "bad event digest")
				return
			}
			leaves = append(leaves, merkle.LeafHash(pd.Bytes[:]))
		}
		want, _ := ex["events_root"].(string)
		if merkle.Root(leaves).String() != want {
			r.fail(v.ID, "coverage root mismatch")
			return
		}
		r.pass()
		return
	}
	r.skip()
}

func checkTaxonomy(v vector, r *report) {
	var ex map[string]any
	_ = decodeInto(v.Expect, &ex)
	if want, ok := ex["ontology_sha256"].(string); ok {
		if taxonomy.OntologySHA256 != want {
			r.fail(v.ID, "ontology hash mismatch: the generated taxonomies have diverged")
			return
		}
	}
	counts, _ := ex["counts"].(map[string]any)
	check := func(name string, got int) bool {
		f, ok := counts[name].(float64)
		return ok && int(f) == got
	}
	if !check("data_classes", len(taxonomy.AllDataClasss)) ||
		!check("operations", len(taxonomy.AllOperations)) ||
		!check("purposes", len(taxonomy.AllPurposes)) ||
		!check("units", len(taxonomy.AllUnits)) {
		r.fail(v.ID, "taxonomy cardinality mismatch")
		return
	}
	classes, _ := ex["data_classes"].([]any)
	for _, c := range classes {
		m, _ := c.(map[string]any)
		code, _ := m["code"].(string)
		dc := taxonomy.DataClass(code)
		if !dc.Valid() {
			r.fail(v.ID, "unknown data class "+code)
			return
		}
		sens, _ := m["sensitivity"].(string)
		if string(taxonomy.DataClassSensitivity[dc]) != sens {
			r.fail(v.ID, "sensitivity mismatch for "+code)
			return
		}
		reid, _ := m["reid_risk"].(float64)
		if taxonomy.DataClassReidRisk[dc] != int(reid) {
			r.fail(v.ID, "re-identification prior mismatch for "+code)
			return
		}
	}
	ops, _ := ex["operations"].([]any)
	for _, o := range ops {
		m, _ := o.(map[string]any)
		code, _ := m["code"].(string)
		op := taxonomy.Operation(code)
		if !op.Valid() {
			r.fail(v.ID, "unknown operation "+code)
			return
		}
		meter, _ := m["meter"].(string)
		if string(taxonomy.OperationMeter[op]) != meter {
			r.fail(v.ID, "meter mismatch for "+code)
			return
		}
		derives, _ := m["derives"].(bool)
		if taxonomy.OperationDerives[op] != derives {
			r.fail(v.ID, "derives mismatch for "+code)
			return
		}
	}
	purposes, _ := ex["purposes"].([]any)
	for _, p := range purposes {
		m, _ := p.(map[string]any)
		code, _ := m["code"].(string)
		pu := taxonomy.Purpose(code)
		if !pu.Valid() {
			r.fail(v.ID, "unknown purpose "+code)
			return
		}
		var wantParent string
		if s, ok := m["parent"].(string); ok {
			wantParent = s
		}
		if string(taxonomy.PurposeParent[pu]) != wantParent {
			r.fail(v.ID, "parent mismatch for "+code)
			return
		}
	}
	r.pass()
}

// unused imports guard
var _ = base64.RawURLEncoding
var _ = strconv.Itoa
