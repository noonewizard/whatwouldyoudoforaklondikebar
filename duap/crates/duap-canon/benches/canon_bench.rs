use criterion::{Criterion, criterion_group, criterion_main};
use duap_canon::{
    Value, codec,
    digest::{Digest, HashAlg},
};
use std::hint::black_box;

fn sample_event() -> Value {
    Value::map([
        ("v", Value::Uint(1)),
        ("eid", Value::Bytes(vec![7u8; 16])),
        ("ts", Value::Uint(1_758_412_800_000_000)),
        ("org", Value::text("org:acme")),
        ("sub", Value::Bytes(vec![3u8; 32])),
        ("cls", Value::text("loc.coarse")),
        ("op", Value::text("process.profiling")),
        ("pur", Value::text("personalization")),
        ("jur", Value::text("EU-DE")),
        ("auth", Value::Bytes(vec![9u8; 32])),
        (
            "q",
            Value::map([("unit", Value::text("record")), ("n", Value::Uint(1))]),
        ),
        (
            "prov",
            Value::Array(vec![
                Value::Bytes(vec![1u8; 32]),
                Value::Bytes(vec![2u8; 32]),
            ]),
        ),
    ])
}

fn bench(c: &mut Criterion) {
    let v = sample_event();
    let bytes = codec::encode(&v);
    let mut g = c.benchmark_group("canon");
    g.throughput(criterion::Throughput::Bytes(bytes.len() as u64));
    g.bench_function("encode_event", |b| b.iter(|| codec::encode(black_box(&v))));
    g.bench_function("decode_event", |b| {
        b.iter(|| codec::decode(black_box(&bytes)).unwrap())
    });
    g.bench_function("digest_sha256", |b| {
        b.iter(|| Digest::of(HashAlg::Sha2_256, "duap.event.v1", black_box(&bytes)))
    });
    g.bench_function("digest_blake3", |b| {
        b.iter(|| Digest::of(HashAlg::Blake3_256, "duap.event.v1", black_box(&bytes)))
    });
    g.bench_function("json_view_roundtrip", |b| {
        b.iter(|| {
            let s = duap_canon::json::to_json_string(black_box(&v));
            duap_canon::json::from_json_str(&s).unwrap()
        })
    });
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
