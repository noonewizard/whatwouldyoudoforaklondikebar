// Package merkle implements the DUAP transparency-log tree: RFC 6962 with
// DUAP domain separation.
//
// STATUS: PRODUCTION (independent implementation, conformance level L3).
package merkle

import (
	"github.com/duap/gateway/internal/canon"
)

const (
	leafDomain  = "duap.log.leaf.v1"
	nodeDomain  = "duap.log.node.v1"
	emptyDomain = "duap.log.empty.v1"
)

// EmptyRoot is the head of a tree with no entries.
func EmptyRoot() canon.Digest { return canon.DigestOf(emptyDomain, nil) }

// LeafHash hashes a leaf payload with the 0x00 prefix.
func LeafHash(data []byte) canon.Digest {
	buf := make([]byte, 0, 1+len(data))
	buf = append(buf, 0x00)
	buf = append(buf, data...)
	return canon.DigestOf(leafDomain, buf)
}

// NodeHash hashes an interior node with the 0x01 prefix.
func NodeHash(l, r canon.Digest) canon.Digest {
	buf := make([]byte, 0, 65)
	buf = append(buf, 0x01)
	buf = append(buf, l.Bytes[:]...)
	buf = append(buf, r.Bytes[:]...)
	return canon.DigestOf(nodeDomain, buf)
}

func splitPoint(n int) int {
	k := 1
	for k<<1 < n {
		k <<= 1
	}
	return k
}

// Root computes the tree head over already-hashed leaves.
func Root(leaves []canon.Digest) canon.Digest {
	switch len(leaves) {
	case 0:
		return EmptyRoot()
	case 1:
		return leaves[0]
	default:
		k := splitPoint(len(leaves))
		return NodeHash(Root(leaves[:k]), Root(leaves[k:]))
	}
}

// VerifyInclusion checks an audit path for leaf index within a tree of size.
func VerifyInclusion(leaf canon.Digest, index, size uint64, path []canon.Digest, root canon.Digest) bool {
	if index >= size {
		return false
	}
	if size == 1 {
		return index == 0 && len(path) == 0 && leaf == root
	}
	fn, sn := index, size-1
	r := leaf
	for _, p := range path {
		if sn == 0 {
			return false
		}
		if fn%2 == 1 || fn == sn {
			r = NodeHash(p, r)
			for fn%2 == 0 && fn != 0 {
				fn /= 2
				sn /= 2
			}
		} else {
			r = NodeHash(r, p)
		}
		fn /= 2
		sn /= 2
	}
	return sn == 0 && r == root
}

// VerifyConsistency checks that the tree of size old is a prefix of the tree
// of size newSize (RFC 6962 section 2.1.2).
func VerifyConsistency(old, newSize uint64, path []canon.Digest, oldRoot, newRoot canon.Digest) bool {
	if old > newSize {
		return false
	}
	if old == newSize {
		return len(path) == 0 && oldRoot == newRoot
	}
	if old == 0 {
		return len(path) == 0
	}
	fn, sn := old-1, newSize-1
	for fn&1 == 1 {
		fn >>= 1
		sn >>= 1
	}
	i := 0
	var fr, sr canon.Digest
	if fn != 0 {
		if len(path) == 0 {
			return false
		}
		fr, sr = path[0], path[0]
		i = 1
	} else {
		fr, sr = oldRoot, oldRoot
	}
	for ; i < len(path); i++ {
		c := path[i]
		if sn == 0 {
			return false
		}
		if fn&1 == 1 || fn == sn {
			fr = NodeHash(c, fr)
			sr = NodeHash(c, sr)
			for fn != 0 && fn&1 == 0 {
				fn >>= 1
				sn >>= 1
			}
		} else {
			sr = NodeHash(sr, c)
		}
		fn >>= 1
		sn >>= 1
	}
	return sn == 0 && fr == oldRoot && sr == newRoot
}
