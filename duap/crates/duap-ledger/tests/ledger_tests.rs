//! Tests for the ledger, invoicing, disputes, netting and settlement.
//!
//! The central invariant, restated from `formal/Accounting.tla`:
//!
//!   INV-L1  Every posted entry balances, per currency.
//!   INV-L2  The ledger's trial balance is zero, always.
//!   INV-L3  Rounding never creates or destroys value.
//!   INV-L4  Netting conserves every participant's net position.
//!   INV-L5  A correction is a new entry, never an edit.

use duap_canon::{Digest, HashAlg};
use duap_ledger::*;
use duap_meter::{ScopeTag, UsageKey};
use duap_model::prelude::*;
use duap_valuation::{PriceBreakdown, PriceEngine, PricingInputs};
use proptest::prelude::*;

const T0: u64 = 1_750_000_000;

fn t(s: u64) -> Timestamp {
    Timestamp::from_secs(s)
}

fn org(s: &str) -> OrgId {
    s.parse().unwrap()
}

fn setup() -> Ledger {
    let mut l = Ledger::new();
    l.ensure_account(
        AccountId::receivable(&org("org:duap/acme")),
        AccountKind::Asset,
        Currency::EUR,
    );
    l.ensure_account(
        AccountId::clearing_fee_revenue(),
        AccountKind::Revenue,
        Currency::EUR,
    );
    l.ensure_account(
        AccountId::settlement_cash(Currency::EUR),
        AccountKind::Asset,
        Currency::EUR,
    );
    l.ensure_account(
        AccountId::rounding(Currency::EUR),
        AccountKind::Rounding,
        Currency::EUR,
    );
    l
}

// ---------------------------------------------------------------------------
// INV-L1 / INV-L2
// ---------------------------------------------------------------------------

#[test]
fn unbalanced_entries_are_refused() {
    let mut l = setup();
    let e = JournalEntry::new(
        "je:1",
        t(T0),
        "bad",
        vec![
            Posting::debit(
                AccountId::receivable(&org("org:duap/acme")),
                Money::new(Currency::EUR, 100),
            ),
            Posting::credit(
                AccountId::clearing_fee_revenue(),
                Money::new(Currency::EUR, 99),
            ),
        ],
    );
    assert!(matches!(l.post(e), Err(LedgerError::Unbalanced { .. })));
    assert!(l.is_balanced());
    assert_eq!(l.entry_count(), 0);
}

#[test]
fn balanced_entries_post_and_the_trial_balance_stays_zero() {
    let mut l = setup();
    for i in 0..20 {
        let e = JournalEntry::new(
            format!("je:{i}"),
            t(T0 + i),
            "fee",
            vec![
                Posting::debit(
                    AccountId::receivable(&org("org:duap/acme")),
                    Money::new(Currency::EUR, 137),
                ),
                Posting::credit(
                    AccountId::clearing_fee_revenue(),
                    Money::new(Currency::EUR, 137),
                ),
            ],
        );
        l.post(e).unwrap();
        assert!(l.is_balanced(), "trial balance broke at entry {i}");
    }
    assert_eq!(
        l.balance(&AccountId::receivable(&org("org:duap/acme"))),
        20 * 137
    );
    assert_eq!(l.balance(&AccountId::clearing_fee_revenue()), -20 * 137);
    let dc = l.debits_and_credits();
    let (d, c) = dc["EUR"];
    assert_eq!(d, c);
}

#[test]
fn duplicate_entry_ids_are_refused() {
    let mut l = setup();
    let mk = || {
        JournalEntry::new(
            "je:dup",
            t(T0),
            "x",
            vec![
                Posting::debit(
                    AccountId::receivable(&org("org:duap/acme")),
                    Money::new(Currency::EUR, 1),
                ),
                Posting::credit(
                    AccountId::clearing_fee_revenue(),
                    Money::new(Currency::EUR, 1),
                ),
            ],
        )
    };
    l.post(mk()).unwrap();
    assert!(matches!(l.post(mk()), Err(LedgerError::Duplicate(_))));
}

#[test]
fn postings_must_match_the_account_currency() {
    let mut l = setup();
    let e = JournalEntry::new(
        "je:fx",
        t(T0),
        "wrong currency",
        vec![
            Posting::debit(
                AccountId::receivable(&org("org:duap/acme")),
                Money::new(Currency::USD, 100),
            ),
            Posting::credit(
                AccountId::clearing_fee_revenue(),
                Money::new(Currency::USD, 100),
            ),
        ],
    );
    assert!(matches!(
        l.post(e),
        Err(LedgerError::CurrencyMismatch { .. })
    ));
}

#[test]
fn a_mixed_currency_entry_cannot_balance_by_accident() {
    let mut l = setup();
    l.ensure_account(
        AccountId::new("acct:usd"),
        AccountKind::Asset,
        Currency::USD,
    );
    let e = JournalEntry::new(
        "je:mixed",
        t(T0),
        "100 EUR against 100 USD",
        vec![
            Posting::debit(
                AccountId::receivable(&org("org:duap/acme")),
                Money::new(Currency::EUR, 100),
            ),
            Posting::credit(AccountId::new("acct:usd"), Money::new(Currency::USD, 100)),
        ],
    );
    assert!(
        matches!(l.post(e), Err(LedgerError::Unbalanced { .. })),
        "per-currency balance must not be satisfied by offsetting different currencies"
    );
}

/// INV-L5: corrections are new entries.
#[test]
fn reversal_is_an_entry_not_an_edit() {
    let mut l = setup();
    let e = JournalEntry::new(
        "je:1",
        t(T0),
        "original",
        vec![
            Posting::debit(
                AccountId::receivable(&org("org:duap/acme")),
                Money::new(Currency::EUR, 500),
            ),
            Posting::credit(
                AccountId::clearing_fee_revenue(),
                Money::new(Currency::EUR, 500),
            ),
        ],
    );
    l.post(e.clone()).unwrap();
    let r = e.reversal("je:1r", t(T0 + 10), "billed in error");
    l.post(r).unwrap();
    assert_eq!(l.balance(&AccountId::receivable(&org("org:duap/acme"))), 0);
    assert_eq!(l.entry_count(), 2, "the original entry is still there");
    assert_eq!(l.entry("je:1r").unwrap().reverses.as_deref(), Some("je:1"));
    assert!(l.is_balanced());
}

// ---------------------------------------------------------------------------
// Invoicing and INV-L3
// ---------------------------------------------------------------------------

fn usage_key(subject: Option<SubjectRef>) -> UsageKey {
    UsageKey {
        controller: org("org:duap/acme"),
        processor: None,
        subject,
        scope_tag: if subject.is_some() {
            ScopeTag::Subject
        } else {
            ScopeTag::NonPersonal
        },
        data_class: DataClass::LocationCoarse,
        operation: Operation::AccessQuery,
        purpose: Purpose::MarketingAdvertisingBehavioral,
        country: "DE".into(),
        unit: Unit::Query,
        window_start: t(T0),
    }
}

fn breakdown(nmu: i128) -> PriceBreakdown {
    PriceBreakdown {
        base_unit_price: Precise::new(Currency::EUR, nmu),
        quantity: 1,
        multipliers: Vec::new(),
        combined_factor: Ratio::ONE,
        amount: Precise::new(Currency::EUR, nmu),
        rule: "test".into(),
        floored: false,
    }
}

#[test]
fn invoice_arithmetic_is_checked() {
    let mut b = InvoiceBuilder::new(
        InvoiceId([1u8; 16]),
        org("org:duap/clearing"),
        org("org:duap/acme"),
        TimeRange::new(t(T0), t(T0 + 86_400)).unwrap(),
        Currency::EUR,
    );
    let root = Digest::of(HashAlg::Sha2_256, "x", b"y");
    b.line(
        usage_key(None),
        root,
        breakdown(1_500_000_000),
        "queries",
        None,
    )
    .unwrap();
    b.line(
        usage_key(None),
        root,
        breakdown(2_500_000_000),
        "more queries",
        None,
    )
    .unwrap();
    let inv = b.build(&NoTax, t(T0), t(T0 + 30 * 86_400)).unwrap();
    assert_eq!(inv.subtotal, Money::new(Currency::EUR, 4));
    assert_eq!(inv.total, inv.subtotal);
    inv.check_arithmetic().unwrap();

    let mut tampered = inv.clone();
    tampered.total = Money::new(Currency::EUR, 9_999);
    assert!(tampered.check_arithmetic().is_err());
}

/// INV-L3: rounding conserves value across an invoice.
#[test]
fn invoice_rounding_residue_is_accounted() {
    let mut b = InvoiceBuilder::new(
        InvoiceId([2u8; 16]),
        org("org:duap/clearing"),
        org("org:duap/acme"),
        TimeRange::new(t(T0), t(T0 + 86_400)).unwrap(),
        Currency::EUR,
    );
    let root = Digest::of(HashAlg::Sha2_256, "x", b"y");
    let amounts = [1_234_567_890i128, 987_654_321, 5_000_000_000, 499_999_999];
    for (i, a) in amounts.iter().enumerate() {
        b.line(
            usage_key(None),
            root,
            breakdown(*a),
            format!("line {i}"),
            None,
        )
        .unwrap();
    }
    let inv = b.build(&NoTax, t(T0), t(T0 + 1)).unwrap();
    let exact: i128 = amounts.iter().sum();
    assert_eq!(
        inv.subtotal.minor * NANO + inv.residue.nmu,
        exact,
        "rounded lines plus residue must equal the exact total"
    );
}

#[test]
fn invoice_posts_a_balanced_entry_with_subject_shares() {
    let subject = SubjectRef([9u8; 16]);
    let mut b = InvoiceBuilder::new(
        InvoiceId([3u8; 16]),
        org("org:duap/clearing"),
        org("org:duap/acme"),
        TimeRange::new(t(T0), t(T0 + 86_400)).unwrap(),
        Currency::EUR,
    );
    let root = Digest::of(HashAlg::Sha2_256, "x", b"y");
    b.line(
        usage_key(Some(subject)),
        root,
        breakdown(100_000_000_000), // 100 minor units
        "queries",
        Some(Ratio::new(7, 10).unwrap()),
    )
    .unwrap();
    let inv = b
        .build(
            &FlatRateTax {
                code: "VAT".into(),
                jurisdiction: "DE".into(),
                rate: Ratio::from_bps(1900),
                rounding: Rounding::HalfEven,
            },
            t(T0),
            t(T0 + 1),
        )
        .unwrap();
    assert_eq!(inv.subtotal, Money::new(Currency::EUR, 100));
    assert_eq!(inv.taxes[0].amount, Money::new(Currency::EUR, 19));
    assert_eq!(inv.total, Money::new(Currency::EUR, 119));
    assert_eq!(
        inv.lines[0].subject_share,
        Some(Money::new(Currency::EUR, 70))
    );

    let mut l = setup();
    for (id, kind) in inv.required_accounts() {
        l.ensure_account(id, kind, Currency::EUR);
    }
    let entry = inv.to_journal_entry(t(T0)).unwrap();
    assert!(entry.balances());
    l.post(entry).unwrap();
    assert!(l.is_balanced());
    assert_eq!(l.balance(&AccountId::subject_payable(&subject)), -70);
    assert_eq!(l.balance(&AccountId::tax_payable("DE", Currency::EUR)), -19);
    assert_eq!(l.balance(&AccountId::clearing_fee_revenue()), -30);
    assert_eq!(
        l.balance(&AccountId::receivable(&org("org:duap/acme"))),
        119
    );
}

#[test]
fn a_priced_counter_flows_through_to_an_invoice() {
    let engine = PriceEngine::new(Currency::EUR);
    let key = usage_key(Some(SubjectRef([4u8; 16])));
    let counter = duap_meter::UsageCounter {
        quantity: 100_000,
        event_count: 100_000,
        first: t(T0),
        last: t(T0 + 3600),
        evidence_root: Digest::of(HashAlg::Sha2_256, "x", b"y"),
        evidence_size: 100_000,
    };
    let rule = PricingRule::PerUnit {
        unit: Unit::Query,
        unit_price: Precise::new(Currency::EUR, 250_000),
    };
    let bd = engine
        .price(&key, &counter, &rule, &PricingInputs::default())
        .unwrap();
    let mut b = InvoiceBuilder::new(
        InvoiceId([5u8; 16]),
        org("org:duap/clearing"),
        org("org:duap/acme"),
        TimeRange::new(t(T0), t(T0 + 86_400)).unwrap(),
        Currency::EUR,
    );
    b.line(
        key,
        counter.evidence_root,
        bd,
        "behavioural advertising queries",
        Some(Ratio::new(1, 2).unwrap()),
    )
    .unwrap();
    let inv = b.build(&NoTax, t(T0), t(T0 + 1)).unwrap();
    inv.check_arithmetic().unwrap();
    // 100_000 queries x 250_000 nmu x 2 (t2 sensitivity) = 5e10 nmu = 50 minor.
    assert_eq!(inv.subtotal, Money::new(Currency::EUR, 50));
    assert_eq!(
        inv.lines[0].subject_share,
        Some(Money::new(Currency::EUR, 25))
    );
}

// ---------------------------------------------------------------------------
// Disputes
// ---------------------------------------------------------------------------

fn a_dispute() -> Dispute {
    Dispute::file(
        DisputeId([1u8; 16]),
        "sub1:0909090909090909090909090909",
        org("org:duap/acme"),
        Claim::OperationDenied {
            event: Digest::of(HashAlg::Sha2_256, "duap.event.v1", b"e"),
        },
        vec![Evidence::LogInclusion {
            entry: Digest::of(HashAlg::Sha2_256, "duap.log-entry.v1", b"e"),
            log: "log:eu-1".into(),
            size: 1000,
        }],
        t(T0),
    )
    .unwrap()
}

#[test]
fn a_dispute_needs_verifiable_evidence() {
    let r = Dispute::file(
        DisputeId([2u8; 16]),
        "claimant",
        org("org:duap/acme"),
        Claim::UnreportedUsage {
            description: "they profiled me".into(),
        },
        vec![Evidence::Assertion {
            by: "claimant".into(),
            statement: "I know they did".into(),
        }],
        t(T0),
    );
    assert!(matches!(r, Err(DisputeError::NoVerifiableEvidence)));
}

#[test]
fn dispute_state_machine_rejects_shortcuts() {
    let mut d = a_dispute();
    assert!(matches!(
        d.transition(DisputeState::Closed, t(T0 + 1), None),
        Err(DisputeError::BadTransition { .. })
    ));
    d.transition(DisputeState::UnderReview, t(T0 + 1), None)
        .unwrap();
    // Upholding without an adjustment is refused.
    assert!(matches!(
        d.transition(DisputeState::Upheld, t(T0 + 2), None),
        Err(DisputeError::AdjustmentRequired)
    ));
    d.propose_adjustment(Money::new(Currency::EUR, -20));
    d.transition(
        DisputeState::Upheld,
        t(T0 + 2),
        Some("event unprovable".into()),
    )
    .unwrap();
    d.transition(DisputeState::Closed, t(T0 + 3), None).unwrap();
    assert!(d.state.is_terminal());
    assert_eq!(d.resolved_at, Some(t(T0 + 2)));
}

#[test]
fn an_upheld_dispute_produces_a_balanced_adjustment() {
    let subject = SubjectRef([9u8; 16]);
    let mut b = InvoiceBuilder::new(
        InvoiceId([7u8; 16]),
        org("org:duap/clearing"),
        org("org:duap/acme"),
        TimeRange::new(t(T0), t(T0 + 86_400)).unwrap(),
        Currency::EUR,
    );
    b.line(
        usage_key(Some(subject)),
        Digest::of(HashAlg::Sha2_256, "x", b"y"),
        breakdown(100_000_000_000),
        "queries",
        Some(Ratio::new(1, 2).unwrap()),
    )
    .unwrap();
    let inv = b.build(&NoTax, t(T0), t(T0 + 1)).unwrap();

    let mut l = setup();
    for (id, kind) in inv.required_accounts() {
        l.ensure_account(id, kind, Currency::EUR);
    }
    l.post(inv.to_journal_entry(t(T0)).unwrap()).unwrap();

    let mut d = a_dispute();
    d.invoice = Some(inv.id);
    d.transition(DisputeState::UnderReview, t(T0 + 1), None)
        .unwrap();
    d.propose_adjustment(Money::new(Currency::EUR, 40));
    d.transition(DisputeState::Settled, t(T0 + 2), Some("agreed".into()))
        .unwrap();

    let adj = d.to_adjustment_entry(&l, t(T0 + 3)).unwrap().unwrap();
    assert!(adj.balances(), "an adjustment must balance");
    l.post(adj).unwrap();
    assert!(l.is_balanced());
    assert_eq!(
        l.balance(&AccountId::receivable(&org("org:duap/acme"))),
        60,
        "the receivable falls by the adjusted amount"
    );
}

// ---------------------------------------------------------------------------
// INV-L4: netting
// ---------------------------------------------------------------------------

#[test]
fn netting_conserves_positions_and_bounds_transfers() {
    let obligations = vec![
        Obligation {
            from: "a".into(),
            to: "b".into(),
            amount: Money::new(Currency::EUR, 100),
        },
        Obligation {
            from: "b".into(),
            to: "c".into(),
            amount: Money::new(Currency::EUR, 80),
        },
        Obligation {
            from: "c".into(),
            to: "a".into(),
            amount: Money::new(Currency::EUR, 50),
        },
        Obligation {
            from: "a".into(),
            to: "c".into(),
            amount: Money::new(Currency::EUR, 20),
        },
    ];
    let (transfers, positions) = net(&obligations, Currency::EUR).unwrap();

    // Positions sum to zero.
    assert_eq!(positions.values().sum::<i128>(), 0);
    // a: -100 -20 +50 = -70; b: +100 -80 = +20; c: +80 -50 +20 = +50
    assert_eq!(positions["a"], -70);
    assert_eq!(positions["b"], 20);
    assert_eq!(positions["c"], 50);

    // Transfers reproduce the positions exactly.
    let mut after: std::collections::BTreeMap<String, i128> = positions.clone();
    for tr in &transfers {
        *after.get_mut(&tr.from).unwrap() += tr.amount.minor;
        *after.get_mut(&tr.to).unwrap() -= tr.amount.minor;
    }
    assert!(
        after.values().all(|v| *v == 0),
        "netting must settle every position"
    );
    assert!(transfers.len() < positions.len());
}

#[test]
fn netting_refuses_mixed_currencies() {
    let obligations = vec![
        Obligation {
            from: "a".into(),
            to: "b".into(),
            amount: Money::new(Currency::EUR, 10),
        },
        Obligation {
            from: "b".into(),
            to: "a".into(),
            amount: Money::new(Currency::USD, 10),
        },
    ];
    assert!(net(&obligations, Currency::EUR).is_err());
}

// ---------------------------------------------------------------------------
// Settlement
// ---------------------------------------------------------------------------

#[test]
fn settlement_state_machine_and_journal() {
    let subject = SubjectRef([9u8; 16]);
    let mut si = SettlementInstruction::new(
        SettlementId([1u8; 16]),
        "acct:system/cash/EUR",
        "subject:payout",
        Money::new(Currency::EUR, 70),
        Rail::Internal,
        t(T0),
    );
    assert!(matches!(
        si.transition(SettlementState::Confirmed, None),
        Err(SettlementError::BadTransition { .. })
    ));
    let rail = InternalRail;
    let r = rail.submit(&si).unwrap();
    si.transition(SettlementState::Submitted, Some(r)).unwrap();
    si.transition(SettlementState::Confirmed, None).unwrap();

    let mut l = setup();
    l.ensure_account(
        AccountId::subject_payable(&subject),
        AccountKind::Liability,
        Currency::EUR,
    );
    // Seed the liability.
    l.post(JournalEntry::new(
        "je:seed",
        t(T0),
        "seed",
        vec![
            Posting::debit(
                AccountId::receivable(&org("org:duap/acme")),
                Money::new(Currency::EUR, 70),
            ),
            Posting::credit(
                AccountId::subject_payable(&subject),
                Money::new(Currency::EUR, 70),
            ),
        ],
    ))
    .unwrap();
    // Cash in from the controller.
    l.post(JournalEntry::new(
        "je:cash",
        t(T0 + 1),
        "payment received",
        vec![
            Posting::debit(
                AccountId::settlement_cash(Currency::EUR),
                Money::new(Currency::EUR, 70),
            ),
            Posting::credit(
                AccountId::receivable(&org("org:duap/acme")),
                Money::new(Currency::EUR, 70),
            ),
        ],
    ))
    .unwrap();

    let e = si.to_journal_entry(
        AccountId::subject_payable(&subject),
        AccountId::settlement_cash(Currency::EUR),
        t(T0 + 2),
    );
    l.post(e).unwrap();
    assert!(l.is_balanced());
    assert_eq!(l.balance(&AccountId::subject_payable(&subject)), 0);
    assert_eq!(l.balance(&AccountId::settlement_cash(Currency::EUR)), 0);
    assert_eq!(l.balance(&AccountId::receivable(&org("org:duap/acme"))), 0);
}

#[test]
fn unimplemented_rails_refuse_to_execute() {
    let si = SettlementInstruction::new(
        SettlementId([2u8; 16]),
        "a",
        "b",
        Money::new(Currency::EUR, 10),
        Rail::DistributedLedger {
            network: "example".into(),
            address_ref: "ref".into(),
        },
        t(T0),
    );
    assert!(!si.rail.executable_here());
    assert!(InternalRail.submit(&si).is_err());
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    /// INV-L2 under arbitrary posting sequences.
    #[test]
    fn ledger_always_balances(amounts in proptest::collection::vec(-1_000_000i128..1_000_000, 1..40)) {
        let mut l = setup();
        for (i, a) in amounts.iter().enumerate() {
            let e = JournalEntry::new(
                format!("je:{i}"), t(T0 + i as u64), "p",
                vec![
                    Posting::debit(AccountId::receivable(&org("org:duap/acme")), Money::new(Currency::EUR, *a)),
                    Posting::credit(AccountId::clearing_fee_revenue(), Money::new(Currency::EUR, *a)),
                ],
            );
            l.post(e).unwrap();
            prop_assert!(l.is_balanced());
        }
        let dc = l.debits_and_credits();
        if let Some((d, c)) = dc.get("EUR") {
            prop_assert_eq!(d, c);
        }
    }

    /// INV-L4 under arbitrary obligation graphs.
    #[test]
    fn netting_always_settles(
        edges in proptest::collection::vec((0usize..8, 0usize..8, 1i128..10_000), 1..40)
    ) {
        let obligations: Vec<Obligation> = edges.iter()
            .filter(|(a, b, _)| a != b)
            .map(|(a, b, m)| Obligation {
                from: format!("p{a}"), to: format!("p{b}"),
                amount: Money::new(Currency::EUR, *m),
            })
            .collect();
        prop_assume!(!obligations.is_empty());
        let (transfers, positions) = net(&obligations, Currency::EUR).unwrap();
        prop_assert_eq!(positions.values().sum::<i128>(), 0);
        let mut after = positions.clone();
        for tr in &transfers {
            *after.get_mut(&tr.from).unwrap() += tr.amount.minor;
            *after.get_mut(&tr.to).unwrap() -= tr.amount.minor;
        }
        prop_assert!(after.values().all(|v| *v == 0));
        prop_assert!(transfers.len() < positions.len().max(1));
    }

    /// INV-L3 over arbitrary invoices.
    #[test]
    fn invoice_residue_always_reconciles(
        nmus in proptest::collection::vec(0i128..10_000_000_000, 1..25)
    ) {
        let mut b = InvoiceBuilder::new(
            InvoiceId([8u8; 16]), org("org:duap/clearing"), org("org:duap/acme"),
            TimeRange::new(t(T0), t(T0 + 86_400)).unwrap(), Currency::EUR,
        );
        let root = Digest::of(HashAlg::Sha2_256, "x", b"y");
        for (i, n) in nmus.iter().enumerate() {
            b.line(usage_key(None), root, breakdown(*n), format!("l{i}"), None).unwrap();
        }
        let inv = b.build(&NoTax, t(T0), t(T0 + 1)).unwrap();
        let exact: i128 = nmus.iter().sum();
        prop_assert_eq!(inv.subtotal.minor * NANO + inv.residue.nmu, exact);
    }
}
