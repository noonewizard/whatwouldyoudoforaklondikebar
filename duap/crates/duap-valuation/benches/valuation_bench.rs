use criterion::{Criterion, criterion_group, criterion_main};
use duap_meter::{ScopeTag, UsageCounter, UsageKey};
use duap_model::prelude::*;
use duap_valuation::*;
use std::collections::BTreeMap;
use std::hint::black_box;

fn key() -> UsageKey {
    UsageKey {
        controller: "org:duap/acme".parse().unwrap(),
        processor: None,
        subject: Some(SubjectRef([1u8; 16])),
        scope_tag: ScopeTag::Subject,
        data_class: DataClass::LocationPrecise,
        operation: Operation::AccessQuery,
        purpose: Purpose::MarketingAdvertisingBehavioral,
        country: "DE".into(),
        unit: Unit::Query,
        window_start: Timestamp::from_secs(1_750_000_000),
    }
}

fn counter(n: u64) -> UsageCounter {
    UsageCounter {
        quantity: n,
        event_count: n,
        first: Timestamp::from_secs(1_750_000_000),
        last: Timestamp::from_secs(1_750_003_600),
        evidence_root: duap_canon::Digest::of(duap_canon::HashAlg::Sha2_256, "x", b"y"),
        evidence_size: n,
    }
}

fn bench(c: &mut Criterion) {
    let e = PriceEngine::new(Currency::EUR);
    let k = key();
    let ct = counter(1000);
    let rule = PricingRule::PerUnit {
        unit: Unit::Query,
        unit_price: Precise::new(Currency::EUR, 12_500),
    };
    let inputs = PricingInputs::default();
    let mut g = c.benchmark_group("valuation");
    g.bench_function("price_per_unit", |b| {
        b.iter(|| e.price(black_box(&k), &ct, &rule, &inputs).unwrap())
    });
    for n in [10usize, 1_000, 100_000] {
        let mut shares = BTreeMap::new();
        for i in 0..n {
            shares.insert(i as u64, Ratio::new(1, n as u64).unwrap());
        }
        g.bench_function(format!("distribute/{n}"), |b| {
            b.iter(|| distribute(Money::new(Currency::EUR, 1_000_000), black_box(&shares)).unwrap())
        });
    }
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
