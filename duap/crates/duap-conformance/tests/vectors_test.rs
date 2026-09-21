//! The committed vectors, checked by `cargo test`.
//!
//! STATUS: REFERENCE.
//!
//! Until this file existed the 93 vectors in `spec/vectors/` were only
//! checked by running the `duap-conformance` binary by hand. A change that
//! altered a vector left `cargo test --workspace` green, which is precisely
//! backwards: a changed vector is a wire-format change and is the single
//! thing that most needs to fail loudly.

use duap_conformance::{Level, VectorFile, vectors};
use std::path::PathBuf;

fn vector_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../spec/vectors")
}

fn committed() -> Vec<(String, VectorFile)> {
    let dir = vector_dir();
    vectors::all()
        .into_iter()
        .map(|(name, _)| {
            let path = dir.join(name);
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
            let file: VectorFile = serde_json::from_str(&text)
                .unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()));
            (name.to_owned(), file)
        })
        .collect()
}

#[test]
fn every_committed_vector_passes_the_self_check() {
    let summary = vectors::self_check(&committed());
    for failure in &summary.failures {
        eprintln!("FAIL {}: {}", failure.id, failure.detail);
    }
    assert!(summary.ok(), "{} vector(s) failed", summary.failed);
    assert_eq!(summary.failed, 0);
    assert!(
        summary.passed >= 93,
        "vector count fell to {}; vectors are only ever added, and a removal \
         is a wire-format change that needs an ADR",
        summary.passed
    );
}

#[test]
fn the_committed_files_are_byte_identical_to_the_generator() {
    let dir = vector_dir();
    for (name, generated) in vectors::all() {
        let path = dir.join(name);
        let on_disk = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        assert_eq!(
            on_disk,
            generated.to_json(),
            "{name} on disk differs from what the generator produces. If this \
             is deliberate it is a wire-format change: write the ADR, then \
             regenerate with `cargo run -p duap-conformance -- generate spec/vectors`."
        );
    }
}

#[test]
fn every_level_is_covered() {
    let files = committed();
    for level in [Level::L1, Level::L2, Level::L3, Level::L4, Level::L5] {
        let n = files
            .iter()
            .flat_map(|(_, f)| f.vectors.iter())
            .filter(|v| v.level == level)
            .count();
        assert!(n > 0, "no vectors at {level:?}; a claimed level with no vectors is unverifiable");
    }
}
