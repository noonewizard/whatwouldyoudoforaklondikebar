use criterion::{Criterion, criterion_group, criterion_main};
use duap_ledger::*;
use duap_model::prelude::*;
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("ledger");
    g.bench_function("post_entry", |b| {
        let mut l = Ledger::new();
        l.ensure_account(AccountId::new("acct:a"), AccountKind::Asset, Currency::EUR);
        l.ensure_account(AccountId::new("acct:b"), AccountKind::Revenue, Currency::EUR);
        let mut i = 0u64;
        b.iter(|| {
            i += 1;
            let e = JournalEntry::new(
                format!("je:{i}"),
                Timestamp::from_secs(1_750_000_000),
                "bench",
                vec![
                    Posting::debit(AccountId::new("acct:a"), Money::new(Currency::EUR, 100)),
                    Posting::credit(AccountId::new("acct:b"), Money::new(Currency::EUR, 100)),
                ],
            );
            l.post(black_box(e)).unwrap()
        })
    });
    for n in [10usize, 1_000, 10_000] {
        let obligations: Vec<Obligation> = (0..n)
            .map(|i| Obligation {
                from: format!("p{}", i % 50),
                to: format!("p{}", (i * 7 + 3) % 50),
                amount: Money::new(Currency::EUR, (i as i128 % 997) + 1),
            })
            .collect();
        g.bench_function(format!("net/{n}"), |b| {
            b.iter(|| net(black_box(&obligations), Currency::EUR).unwrap())
        });
    }
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
