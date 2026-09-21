use criterion::{Criterion, criterion_group, criterion_main};
use duap_meter::*;
use duap_model::prelude::*;
use std::hint::black_box;

fn mk(i: u64) -> DataUsageEvent {
    let controller: OrgId = "org:duap/acme".parse().unwrap();
    EventBuilder::new(
        EventId((i as u128).to_be_bytes()),
        AgentRef::new("bench", "0"),
        controller,
        SubjectScope::Subject { subject: SubjectRef(((i % 10_000) as u128).to_be_bytes()) },
        Jurisdiction::new("DE").unwrap(),
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
        AuthorizationRef {
            grant: GrantId([1u8; 16]),
            grant_digest: duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "g", b"g"),
            epoch: 1,
        },
        1,
        Timestamp::from_secs(1_750_000_000 + i % 3600),
    )
    .build()
    .unwrap()
}

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("meter");
    g.throughput(criterion::Throughput::Elements(1));
    g.bench_function("pipeline_offer", |b| {
        let mut p = MeterPipeline::new(
            WindowConfig::default(),
            WindowSize::Hour,
            ResolutionPolicy::ControllerWins,
        );
        let now = Timestamp::from_secs(1_750_003_600);
        let mut i = 0u64;
        b.iter(|| {
            i += 1;
            p.offer(black_box(&mk(i)), now).unwrap()
        })
    });
    g.bench_function("fingerprint", |b| {
        let ev = mk(7);
        b.iter(|| duap_meter::fingerprint(black_box(&ev), 1_000))
    });
    g.bench_function("event_digest", |b| {
        let ev = mk(7);
        b.iter(|| black_box(&ev).digest().unwrap())
    });
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
