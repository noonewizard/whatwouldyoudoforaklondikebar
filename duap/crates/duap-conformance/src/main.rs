//! Generate or check the DUAP conformance vectors.
//!
//! ```text
//! duap-conformance generate <dir>   write the vectors
//! duap-conformance check <dir>      re-derive and compare
//! ```

use duap_conformance::{VectorFile, vectors};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("check");
    let dir = PathBuf::from(
        args.get(2)
            .cloned()
            .unwrap_or_else(|| "spec/vectors".to_owned()),
    );

    match cmd {
        "generate" => {
            std::fs::create_dir_all(&dir).expect("vector directory is writable");
            let mut total = 0;
            for (name, file) in vectors::all() {
                total += file.vectors.len();
                std::fs::write(dir.join(name), file.to_json()).expect("vector file is writable");
                println!(
                    "wrote {} ({} vectors)",
                    dir.join(name).display(),
                    file.vectors.len()
                );
            }
            println!("{total} vectors in {} files", vectors::all().len());
        }
        "check" => {
            let mut files = Vec::new();
            for (name, _) in vectors::all() {
                let path = dir.join(name);
                let text = match std::fs::read_to_string(&path) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("cannot read {}: {e}", path.display());
                        std::process::exit(2);
                    }
                };
                let f: VectorFile = match serde_json::from_str(&text) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("cannot parse {}: {e}", path.display());
                        std::process::exit(2);
                    }
                };
                files.push((name.to_owned(), f));
            }
            let s = vectors::self_check(&files);
            println!(
                "conformance self-check: {} passed, {} failed",
                s.passed, s.failed
            );
            for f in &s.failures {
                println!("  FAIL {}: {}", f.id, f.detail);
            }
            if !s.ok() {
                std::process::exit(1);
            }
        }
        other => {
            eprintln!("unknown command {other:?}; use generate or check");
            std::process::exit(2);
        }
    }
}
