//! The vertical slice, asserted.
//!
//! These are the tests that decide whether the protocol kernel works: they
//! run the whole transaction and check the invariants the specification
//! claims, rather than checking any one component in isolation.

use duap_demo::{DemoResult, run, scenario_digest};

fn once() -> DemoResult {
    run().expect("the demonstration must complete")
}

#[test]
fn the_whole_transaction_completes() {
    let r = once();
    assert_eq!(r.events_accepted, 8);
    assert_eq!(r.events_refused, 6);
    assert_eq!(r.receipts, 6);
    assert!(r.independent_verification_passed);
}

/// INV-L2: the ledger balances after every stage, including the dispute
/// adjustment and the settlement.
#[test]
fn the_ledger_balances_at_the_end() {
    assert!(once().trial_balance_zero);
}

/// Half of every charge reached the subject, exactly.
#[test]
fn the_subject_share_is_what_the_configuration_says() {
    let r = once();
    assert_eq!(r.invoice_total, "0.39 EUR");
    assert_eq!(r.subject_share, "0.19 EUR");
    // 39 minor units, half rounded toward zero across six lines, is 19.
    assert_eq!(r.subject_paid, "2.28 EUR");
    assert_eq!(r.subject_outstanding, "0.000000000 minor EUR");
}

/// Determinism is what makes the artefacts usable as conformance vectors.
#[test]
fn the_demonstration_is_deterministic() {
    let a = once();
    let b = once();
    assert_eq!(a.log_root, b.log_root, "the transparency-log root must reproduce");
    assert_eq!(a.invoice_total, b.invoice_total);
    assert_eq!(a.attribution, b.attribution);
    assert_eq!(a.steps.len(), b.steps.len());
    for (x, y) in a.steps.iter().zip(b.steps.iter()) {
        assert_eq!(x.stage, y.stage);
        assert_eq!(x.detail, y.detail);
        assert_eq!(x.facts, y.facts);
    }
    assert_eq!(scenario_digest(), scenario_digest());
}

/// Every refusal in the run is a *different* control failing, not the same
/// one six times. A slice that only ever exercises one rejection path is not
/// evidence that the others work.
#[test]
fn each_refusal_exercises_a_distinct_control() {
    let r = once();
    let refusal_steps: Vec<&String> = r
        .steps
        .iter()
        .filter(|s| s.stage == "refusal" || s.stage == "double_counting" || s.stage == "transfer")
        .map(|s| &s.detail)
        .collect();
    assert!(refusal_steps.iter().any(|d| d.contains("Behavioural advertising")));
    assert!(refusal_steps.iter().any(|d| d.contains("Precise location")));
    assert!(refusal_steps.iter().any(|d| d.contains("Sale refused")));
    assert!(refusal_steps.iter().any(|d| d.contains("allowlist")));
    assert!(refusal_steps.iter().any(|d| d.contains("same aggregation")));
}

/// The receipt must carry as many explicit non-claims as claims.
#[test]
fn receipts_publish_their_own_limits() {
    let r = once();
    let audit = r
        .steps
        .iter()
        .find(|s| s.stage == "audit")
        .expect("the audit step runs");
    let established: u32 = audit
        .facts
        .iter()
        .find_map(|f| f.strip_prefix("claims_established="))
        .and_then(|v| v.parse().ok())
        .expect("claims counted");
    let not: u32 = audit
        .facts
        .iter()
        .find_map(|f| f.strip_prefix("claims_explicitly_not_established="))
        .and_then(|v| v.parse().ok())
        .expect("non-claims counted");
    assert_eq!(established, 5 * r.receipts as u32, "anchored receipts establish five things each");
    assert_eq!(not, 5 * r.receipts as u32);
}

/// Revocation stops future use and says plainly that it does not unlearn a
/// model.
#[test]
fn revocation_is_prospective_and_says_so() {
    let r = once();
    let rev = r
        .steps
        .iter()
        .find(|s| s.stage == "revocation")
        .expect("the revocation step runs");
    assert!(rev.facts.iter().any(|f| f == "training_after=refused"));
    assert!(rev.facts.iter().any(|f| f == "service_after=accepted"));
    assert!(
        rev.facts.iter().any(|f| f == "already_trained_model=NOT_UNLEARNED"),
        "the demonstration must not imply the model forgot anything"
    );
}

#[test]
fn attribution_is_exact_and_labelled() {
    let r = once();
    assert_eq!(r.attribution.len(), 1);
    assert_eq!(r.attribution[0].1, "1", "the sole contributor holds the whole share");
    let prov = r
        .steps
        .iter()
        .find(|s| s.stage == "provenance")
        .expect("the provenance step runs");
    assert!(
        prov.facts
            .iter()
            .any(|f| f.contains("NOT influence") && f.contains("NOT economic contribution"))
    );
}

#[test]
fn commitments_open_and_only_to_the_committed_value() {
    assert!(duap_demo::check_commitment());
}

/// The transcript is an artefact: CI diffs it, so a silent behaviour change
/// shows up as a diff rather than as a passing test.
#[test]
fn transcript_matches_the_committed_golden_file() {
    let r = once();
    let rendered = render(&r);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/transcript.txt");
    let expected = std::fs::read_to_string(path).unwrap_or_default();
    if expected.trim() != rendered.trim() {
        if std::env::var("DUAP_UPDATE_GOLDEN").is_ok() {
            std::fs::create_dir_all(
                std::path::Path::new(path).parent().expect("has a parent"),
            )
            .expect("golden directory is writable");
            std::fs::write(path, &rendered).expect("golden file is writable");
            return;
        }
        panic!(
            "the demonstration transcript changed.\n\
             Review the diff, then re-run with DUAP_UPDATE_GOLDEN=1 to accept it.\n\
             --- got ---\n{rendered}"
        );
    }
}

fn render(r: &DemoResult) -> String {
    let mut s = String::new();
    for step in &r.steps {
        s.push_str(&format!("{:>2}. [{}] {}\n", step.n, step.stage, step.detail));
        for f in &step.facts {
            s.push_str(&format!("      {f}\n"));
        }
    }
    s.push_str(&format!(
        "summary accepted={} refused={} receipts={} total={} subject={} paid={} log={} balanced={} verified={}\n",
        r.events_accepted,
        r.events_refused,
        r.receipts,
        r.invoice_total,
        r.subject_share,
        r.subject_paid,
        r.log_size,
        r.trial_balance_zero,
        r.independent_verification_passed
    ));
    s
}
