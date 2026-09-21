//! Run the synthetic end-to-end DUAP demonstration.
//!
//! `cargo run -p duap-demo` prints a human-readable transcript.
//! `cargo run -p duap-demo -- --json` prints the machine-readable result.

fn main() {
    let json = std::env::args().any(|a| a == "--json");
    match duap_demo::run() {
        Ok(r) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&r).expect("result is serialisable")
                );
                return;
            }
            println!("DUAP end-to-end demonstration -- Acme Example Corp (synthetic)");
            println!("scenario digest: {}", duap_demo::scenario_digest());
            println!();
            for s in &r.steps {
                println!("{:>2}. [{}] {}", s.n, s.stage, s.detail);
                for f in &s.facts {
                    println!("      {f}");
                }
            }
            println!();
            println!("summary");
            println!("  events accepted ............ {}", r.events_accepted);
            println!("  events refused ............. {}", r.events_refused);
            println!("  receipts issued ............ {}", r.receipts);
            println!("  invoice total .............. {}", r.invoice_total);
            println!("  subject share .............. {}", r.subject_share);
            println!("  subject paid ............... {}", r.subject_paid);
            println!("  subject balance carried .... {}", r.subject_outstanding);
            println!("  transparency log entries ... {}", r.log_size);
            println!("  log root ................... {}", r.log_root);
            println!("  trial balance is zero ...... {}", r.trial_balance_zero);
            println!(
                "  independent verification ... {}",
                r.independent_verification_passed
            );
            println!(
                "  commitment opens correctly . {}",
                duap_demo::check_commitment()
            );
            for (s, share) in &r.attribution {
                println!("  attribution {s} = {share}");
            }
        }
        Err(e) => {
            eprintln!("demonstration failed: {e}");
            std::process::exit(1);
        }
    }
}
