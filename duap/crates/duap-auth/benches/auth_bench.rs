use criterion::{Criterion, criterion_group, criterion_main};
use duap_auth::prelude::*;
use duap_model::prelude::*;
use std::hint::black_box;

fn setup(n_terms: u32) -> (Grant, DataUsageEvent) {
    let controller: OrgId = "org:duap/acme".parse().unwrap();
    let subject = SubjectRef([1u8; 16]);
    let key = duap_crypto::SecretKey::from_seed(duap_crypto::SuiteId::Ed25519, [2u8; 32]);
    let mut b = GrantBuilder::new(
        GrantId([3u8; 16]),
        subject,
        key.key_id(),
        controller.clone(),
        Timestamp::from_secs(1_700_000_000),
        Currency::EUR,
    );
    for i in 0..n_terms {
        b = b.term(
            Term::permit(
                i,
                Matcher::any().classes(ClassSelector::In {
                    values: vec![DataClass::ALL[(i as usize) % DataClass::ALL.len()]],
                }),
            )
            .with_pricing(PricingRule::Free),
        );
    }
    let grant = b.build().unwrap();
    let ev = EventBuilder::new(
        EventId([4u8; 16]),
        AgentRef::new("bench", "0"),
        controller,
        SubjectScope::Subject { subject },
        Jurisdiction::new("DE").unwrap(),
        DataClass::LocationCoarse,
        Operation::AccessQuery,
        Purpose::ServiceCore,
        grant.reference().unwrap(),
        1,
        Timestamp::from_secs(1_700_000_100),
    )
    .build()
    .unwrap();
    (grant, ev)
}

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("auth");
    for n in [1u32, 8, 64, 256] {
        let (grant, ev) = setup(n);
        let ctx = EvalContext::verified();
        g.bench_function(format!("evaluate/{n}_terms"), |b| {
            b.iter(|| evaluate(black_box(&grant), &[], black_box(&ev), &ctx))
        });
    }
    let (grant, _) = setup(16);
    g.bench_function("grant_digest", |b| b.iter(|| black_box(&grant).digest().unwrap()));
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
