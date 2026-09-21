use criterion::{Criterion, criterion_group, criterion_main};
use duap_crypto::{Envelope, SecretKey, SuiteId};
use std::hint::black_box;

fn payload() -> Vec<u8> {
    duap_canon::to_canonical_cbor(&serde_json::json!({
        "org": "org:acme", "class": "loc.coarse", "op": "process.profiling", "n": 1
    }))
    .unwrap()
}

fn bench(c: &mut Criterion) {
    let p = payload();
    let mut g = c.benchmark_group("crypto");
    for suite in [
        SuiteId::Ed25519,
        SuiteId::MlDsa44,
        SuiteId::MlDsa65,
        SuiteId::Ed25519MlDsa44,
    ] {
        let key = SecretKey::from_seed(suite, [17u8; 32]);
        let pk = key.public_key();
        let mut env = Envelope::from_payload_bytes("duap.event.v1", p.clone()).unwrap();
        env.sign(&key, 1_000_000, None).unwrap();
        let sig = env.signatures[0].clone();

        g.bench_function(format!("sign/{suite}"), |b| {
            b.iter(|| {
                let mut e = Envelope::from_payload_bytes("duap.event.v1", p.clone()).unwrap();
                e.sign(black_box(&key), 1_000_000, None).unwrap();
            })
        });
        g.bench_function(format!("verify/{suite}"), |b| {
            b.iter(|| env.verify_with_key(black_box(&sig), &pk).unwrap())
        });
        g.bench_function(format!("keygen/{suite}"), |b| {
            b.iter(|| SecretKey::from_seed(suite, black_box([3u8; 32])).public_key())
        });
    }
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
