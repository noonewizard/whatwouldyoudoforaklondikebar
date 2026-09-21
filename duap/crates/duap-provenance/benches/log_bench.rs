use criterion::{Criterion, criterion_group, criterion_main};
use duap_canon::HashAlg;
use duap_provenance::MerkleLog;
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("merkle");
    for n in [1_000usize, 100_000] {
        let mut log = MerkleLog::new(HashAlg::Sha2_256);
        for i in 0..n {
            log.append(&(i as u64).to_be_bytes());
        }
        let size = log.len();
        g.bench_function(format!("root/{n}"), |b| b.iter(|| black_box(&log).root()));
        g.bench_function(format!("inclusion_proof/{n}"), |b| {
            b.iter(|| black_box(&log).inclusion_proof(size / 3, size).unwrap())
        });
        let proof = log.inclusion_proof(size / 3, size).unwrap();
        let leaf = log.leaf(size / 3).unwrap();
        let root = log.root();
        g.bench_function(format!("inclusion_verify/{n}"), |b| {
            b.iter(|| assert!(black_box(&proof).verify(leaf, root)))
        });
        g.bench_function(format!("consistency_proof/{n}"), |b| {
            b.iter(|| black_box(&log).consistency_proof(size / 2, size).unwrap())
        });
    }
    let mut log = MerkleLog::new(HashAlg::Sha2_256);
    g.bench_function("append", |b| {
        let mut i = 0u64;
        b.iter(|| {
            i += 1;
            log.append(black_box(&i.to_be_bytes()))
        })
    });
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
