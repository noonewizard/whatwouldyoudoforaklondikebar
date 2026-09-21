//! Rust mirrors of the invariants checked in `formal/Accounting.tla`.
//!
//! Each test bears the model invariant's name. As in the authorization
//! mirror, these establish agreement on specific configurations rather than
//! a refinement proof.

use duap_ledger::*;
use duap_model::prelude::*;

const T0: u64 = 1_700_000_000;

fn scale() -> i128 {
    NANO
}

fn ledger() -> Ledger {
    let mut l = Ledger::new();
    for (id, kind) in [
        (AccountId::new("acct:a"), AccountKind::Asset),
        (AccountId::new("acct:b"), AccountKind::Revenue),
        (AccountId::new("acct:subject"), AccountKind::Liability),
        (AccountId::new("acct:clearing"), AccountKind::Asset),
    ] {
        l.ensure_account(id, kind, Currency::EUR);
    }
    l
}

fn total(l: &Ledger) -> i128 {
    ["acct:a", "acct:b", "acct:subject", "acct:clearing"]
        .iter()
        .map(|a| l.balance(&AccountId::new(*a)))
        .sum()
}

/// `EntryBalances` in `formal/Accounting.tla` (INV-L1): no transition
/// changes the ledger total, and an unbalanced entry is not postable.
#[test]
fn entry_balances() {
    let mut l = ledger();
    let mut n = 0;
    for v in -3i128..=3 {
        n += 1;
        let e = JournalEntry::new(
            format!("je:{n}"),
            Timestamp::from_secs(T0 + n),
            "model post",
            vec![
                Posting::debit(AccountId::new("acct:a"), Money::new(Currency::EUR, v)),
                Posting::credit(AccountId::new("acct:b"), Money::new(Currency::EUR, v)),
            ],
        );
        let before = total(&l);
        l.post(e).expect("a balanced entry posts");
        assert_eq!(total(&l), before, "a posting changed the ledger total");
    }

    // An unbalanced entry is refused, and the refusal changes nothing.
    let before = total(&l);
    let bad = JournalEntry::new(
        "je:bad",
        Timestamp::from_secs(T0),
        "unbalanced",
        vec![
            Posting::debit(AccountId::new("acct:a"), Money::new(Currency::EUR, 5)),
            Posting::credit(AccountId::new("acct:b"), Money::new(Currency::EUR, 4)),
        ],
    );
    assert!(matches!(l.post(bad), Err(LedgerError::Unbalanced { .. })));
    assert_eq!(total(&l), before);
}

/// `TrialBalanceZero` in `formal/Accounting.tla` (INV-L2).
#[test]
fn trial_balance_zero() {
    let mut l = ledger();
    assert!(l.is_balanced());
    for n in 1..20i128 {
        let e = JournalEntry::new(
            format!("je:{n}"),
            Timestamp::from_secs(T0 + n as u64),
            "model post",
            vec![
                Posting::debit(AccountId::new("acct:a"), Money::new(Currency::EUR, n * 7)),
                Posting::credit(AccountId::new("acct:b"), Money::new(Currency::EUR, n * 7)),
            ],
        );
        l.post(e).expect("posts");
        assert!(l.is_balanced(), "trial balance broke after entry {n}");
        assert_eq!(total(&l), 0);
    }
}

/// `ValueConserved` in `formal/Accounting.tla` (INV-L3): everything accrued
/// is either settled or still carried; nothing is rounded to zero and
/// nothing is invented.
#[test]
fn value_conserved() {
    let mut acc: duap_valuation::PayoutAccumulator<u32> =
        duap_valuation::PayoutAccumulator::new(Currency::EUR, Money::new(Currency::EUR, 1));
    let mut accrued = 0i128;
    let mut settled = 0i128;

    // The model's Accrue/Settle interleaving, at the real 10^9 scale.
    for round in 1..=25i128 {
        let v = round * 37_000_000; // sub-unit amounts
        accrued += v;
        acc.accrue(1, Precise::new(Currency::EUR, v), Timestamp::from_secs(T0))
            .expect("accrues");
        for p in acc.release(Timestamp::from_secs(T0 + 1)).expect("releases") {
            settled += p.amount.minor;
        }
        let carried = acc.total_outstanding().expect("total").nmu;
        assert_eq!(
            settled * scale() + carried,
            accrued,
            "value was created or destroyed at round {round}"
        );
    }
    assert!(settled > 0, "the model must actually settle something");
}

/// The rounding remainder never goes negative and never reaches a whole
/// unit once settlement has run.
#[test]
fn remainder_bounded() {
    let mut acc: duap_valuation::PayoutAccumulator<u32> =
        duap_valuation::PayoutAccumulator::new(Currency::EUR, Money::new(Currency::EUR, 1));
    for v in [1i128, NANO - 1, NANO, NANO + 1, 5 * NANO + 7] {
        acc.accrue(1, Precise::new(Currency::EUR, v), Timestamp::from_secs(T0))
            .expect("accrues");
        acc.release(Timestamp::from_secs(T0 + 1)).expect("releases");
        let carried = acc.total_outstanding().expect("total").nmu;
        assert!(carried >= 0, "the carried remainder went negative");
        assert!(
            carried < NANO,
            "a whole settled unit was left carried instead of paid"
        );
    }
}
