//! `duax`: the DUAP command-line interface.
//!
//! STATUS: PRODUCTION (reference implementation).
//!
//! # Design
//!
//! Every command operates on files in one of two formats, and the choice is
//! never implicit:
//!
//! * **Canonical CBOR** (`.duap`) for anything signed or hashed. These are
//!   the bytes the protocol defines; the CLI never re-encodes them before
//!   verifying.
//! * **The JSON view** (`.json`) for anything a person writes or reads. The
//!   mapping is lossless and specified, so a policy authored by hand
//!   produces exactly the same digest as one produced by an SDK.
//!
//! Commands that produce key material write it to a file with restrictive
//! permissions and print only the public half. Commands that verify print a
//! machine-readable summary on stdout and a human explanation on stderr, so
//! `duax receipt verify ... | jq` works in a pipeline while a person still
//! sees why something failed.

use clap::{Parser, Subcommand};
use duap_canon::digest::{Digest, HashAlg};
use duap_crypto::{
    Envelope, KeyRecord, KeyRegistry, KeyRole, PublicKey, SecretKey, SuiteId, SuitePolicy,
    VerificationContext,
};
use duap_model::prelude::*;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "duax",
    version,
    about = "Data Usage Accounting Protocol command-line interface",
    long_about = "duax operates on DUAP protocol objects: keys, authorizations, \
                  events, receipts and conformance vectors.\n\n\
                  Canonical CBOR is used for anything signed; the JSON view is \
                  used for anything authored or read by a person. Both produce \
                  the same digests."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a signing key and write the seed to a file.
    Keygen {
        /// Suite: ed25519, ml-dsa-44, ml-dsa-65, or ed25519+ml-dsa-44.
        #[arg(long, default_value = "ed25519")]
        suite: String,
        /// Where to write the 32-byte seed, hex encoded.
        #[arg(long)]
        out: PathBuf,
        /// Use this seed instead of the system CSPRNG. For tests and
        /// reproducible fixtures only; never for a key that matters.
        #[arg(long)]
        seed_hex: Option<String>,
    },
    /// Print the public key and identifier of a key file.
    Id {
        #[arg(long)]
        key: PathBuf,
        #[arg(long, default_value = "ed25519")]
        suite: String,
    },
    /// Build a key registry from key files.
    Registry {
        /// Entries as holder=suite:keyfile:role[,role...]
        #[arg(long = "entry", required = true)]
        entries: Vec<String>,
        #[arg(long)]
        out: PathBuf,
    },
    /// Canonical-encoding utilities.
    #[command(subcommand)]
    Canon(CanonCmd),
    /// Authorization grants.
    #[command(subcommand)]
    Grant(GrantCmd),
    /// Data usage events.
    #[command(subcommand)]
    Event(EventCmd),
    /// Receipts.
    #[command(subcommand)]
    Receipt(ReceiptCmd),
    /// Price a metered quantity under a rule.
    Price {
        /// Pricing rule, as a JSON-view document.
        #[arg(long)]
        rule: PathBuf,
        #[arg(long)]
        data_class: String,
        #[arg(long)]
        operation: String,
        #[arg(long)]
        purpose: String,
        #[arg(long)]
        quantity: u64,
        #[arg(long, default_value = "EUR")]
        currency: String,
    },
    /// Conformance vectors.
    #[command(subcommand)]
    Conformance(ConformanceCmd),
    /// Print the ontology the binary was built against.
    Taxonomy {
        /// One of: classes, operations, purposes, units.
        #[arg(default_value = "classes")]
        what: String,
    },
}

#[derive(Subcommand)]
enum CanonCmd {
    /// Convert a JSON-view document to canonical CBOR.
    Encode {
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Convert canonical CBOR to the JSON view.
    Decode { input: PathBuf },
    /// Print the digest of a canonical CBOR file under a domain.
    Digest {
        input: PathBuf,
        #[arg(long, default_value = "duap.test.v1")]
        domain: String,
        #[arg(long, default_value = "sha2-256")]
        alg: String,
    },
    /// Check whether a file is canonical.
    Check { input: PathBuf },
}

#[derive(Subcommand)]
enum GrantCmd {
    /// Show a grant in the JSON view, with its digest.
    Show { input: PathBuf },
    /// Verify a sealed grant against a registry.
    Verify {
        input: PathBuf,
        #[arg(long)]
        registry: PathBuf,
    },
}

#[derive(Subcommand)]
enum EventCmd {
    /// Validate an event file against the structural rules.
    Validate { input: PathBuf },
    /// Show an event with its canonical digest.
    Show { input: PathBuf },
    /// Verify a sealed event envelope.
    Verify {
        input: PathBuf,
        #[arg(long)]
        registry: PathBuf,
    },
}

#[derive(Subcommand)]
enum ReceiptCmd {
    /// Verify a sealed receipt and print what it does and does not prove.
    Verify {
        input: PathBuf,
        #[arg(long)]
        registry: PathBuf,
    },
    /// Show a receipt's claims without verifying a signature.
    Claims { input: PathBuf },
}

#[derive(Subcommand)]
enum ConformanceCmd {
    /// Write the vector suite.
    Generate {
        #[arg(default_value = "spec/vectors")]
        dir: PathBuf,
    },
    /// Re-derive the vectors and compare with the committed files.
    Check {
        #[arg(default_value = "spec/vectors")]
        dir: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("duax: {e}");
            ExitCode::FAILURE
        }
    }
}

type R = Result<(), String>;

fn run(cli: Cli) -> R {
    match cli.command {
        Command::Keygen {
            suite,
            out,
            seed_hex,
        } => keygen(&suite, &out, seed_hex.as_deref()),
        Command::Id { key, suite } => show_id(&key, &suite),
        Command::Registry { entries, out } => build_registry(&entries, &out),
        Command::Canon(c) => canon_cmd(c),
        Command::Grant(g) => grant_cmd(g),
        Command::Event(e) => event_cmd(e),
        Command::Receipt(r) => receipt_cmd(r),
        Command::Price {
            rule,
            data_class,
            operation,
            purpose,
            quantity,
            currency,
        } => price(
            &rule,
            &data_class,
            &operation,
            &purpose,
            quantity,
            &currency,
        ),
        Command::Conformance(c) => conformance_cmd(c),
        Command::Taxonomy { what } => taxonomy(&what),
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn read(path: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

fn read_text(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

fn write(path: &Path, bytes: &[u8]) -> R {
    std::fs::write(path, bytes).map_err(|e| format!("cannot write {}: {e}", path.display()))
}

fn parse_suite(s: &str) -> Result<SuiteId, String> {
    s.parse::<SuiteId>().map_err(|e| e.to_string())
}

fn load_key(path: &Path, suite: SuiteId) -> Result<SecretKey, String> {
    let text = read_text(path)?;
    let raw = hex::decode(text.trim()).map_err(|e| format!("key file is not hex: {e}"))?;
    let seed: [u8; 32] = raw
        .try_into()
        .map_err(|_| "key file must contain exactly 32 bytes of hex".to_owned())?;
    Ok(SecretKey::from_seed(suite, seed))
}

/// A registry file is the JSON view of the serialised `KeyRegistry`.
fn load_registry(path: &Path) -> Result<KeyRegistry, String> {
    let text = read_text(path)?;
    serde_json::from_str(&text).map_err(|e| format!("cannot parse registry: {e}"))
}

fn load_envelope(path: &Path) -> Result<Envelope, String> {
    let bytes = read(path)?;
    duap_canon::from_canonical_cbor(&bytes).map_err(|e| format!("not a canonical envelope: {e}"))
}

fn json_view_to_canonical(path: &Path) -> Result<Vec<u8>, String> {
    let text = read_text(path)?;
    let v = duap_canon::json::from_json_str(&text).map_err(|e| e.to_string())?;
    Ok(duap_canon::encode(&v))
}

// ---------------------------------------------------------------------------
// commands
// ---------------------------------------------------------------------------

fn keygen(suite: &str, out: &Path, seed_hex: Option<&str>) -> R {
    let suite = parse_suite(suite)?;
    let key = match seed_hex {
        Some(h) => {
            eprintln!(
                "duax: WARNING -- a key derived from a supplied seed is only as secret as that \
                 seed. Use this for fixtures, never for production."
            );
            let raw = hex::decode(h).map_err(|e| format!("seed is not hex: {e}"))?;
            let seed: [u8; 32] = raw
                .try_into()
                .map_err(|_| "seed must be 32 bytes".to_owned())?;
            SecretKey::from_seed(suite, seed)
        }
        None => SecretKey::generate(suite).map_err(|e| e.to_string())?,
    };
    write(out, hex::encode(key.seed()).as_bytes())?;
    restrict(out);
    let pk = key.public_key();
    println!(
        "{}",
        serde_json::json!({
            "suite": suite.label(),
            "key_id": key.key_id().to_string(),
            "public_key_hex": hex::encode(&pk.bytes),
            "public_key_bytes": pk.bytes.len(),
            "quantum_resistant": suite.quantum_resistant(),
            "seed_file": out.display().to_string(),
        })
    );
    Ok(())
}

#[cfg(unix)]
fn restrict(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn restrict(_path: &Path) {}

fn show_id(key: &Path, suite: &str) -> R {
    let suite = parse_suite(suite)?;
    let k = load_key(key, suite)?;
    let pk = k.public_key();
    println!(
        "{}",
        serde_json::json!({
            "suite": suite.label(),
            "key_id": k.key_id().to_string(),
            "public_key_hex": hex::encode(&pk.bytes),
        })
    );
    Ok(())
}

fn build_registry(entries: &[String], out: &Path) -> R {
    let mut reg = KeyRegistry::new();
    for e in entries {
        let (holder, rest) = e
            .split_once('=')
            .ok_or_else(|| format!("entry {e:?} is not holder=suite:keyfile:roles"))?;
        let mut parts = rest.splitn(3, ':');
        let suite = parse_suite(parts.next().unwrap_or_default())?;
        let keyfile = parts
            .next()
            .ok_or_else(|| format!("entry {e:?} has no key file"))?;
        let roles_str = parts.next().unwrap_or("event_signer");
        let mut roles = Vec::new();
        for r in roles_str.split(',') {
            roles.push(match r.trim() {
                "event_signer" => KeyRole::EventSigner,
                "authorization_signer" => KeyRole::AuthorizationSigner,
                "receipt_signer" => KeyRole::ReceiptSigner,
                "log_signer" => KeyRole::LogSigner,
                "settlement_signer" => KeyRole::SettlementSigner,
                "registry_admin" => KeyRole::RegistryAdmin,
                "auditor" => KeyRole::Auditor,
                other => return Err(format!("unknown role {other:?}")),
            });
        }
        let key = load_key(Path::new(keyfile), suite)?;
        reg.enroll(KeyRecord::new(key.public_key(), holder, roles, 0, None))
            .map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&reg).map_err(|e| e.to_string())?;
    write(out, text.as_bytes())?;
    println!(
        "{}",
        serde_json::json!({
            "keys": reg.len(),
            "state_digest": reg.state_digest().map_err(|e| e.to_string())?.to_string(),
            "file": out.display().to_string(),
        })
    );
    Ok(())
}

fn canon_cmd(c: CanonCmd) -> R {
    match c {
        CanonCmd::Encode { input, out } => {
            let bytes = json_view_to_canonical(&input)?;
            match out {
                Some(p) => {
                    write(&p, &bytes)?;
                    println!(
                        "{}",
                        serde_json::json!({ "bytes": bytes.len(), "file": p.display().to_string() })
                    );
                }
                None => println!("{}", hex::encode(&bytes)),
            }
            Ok(())
        }
        CanonCmd::Decode { input } => {
            let bytes = read(&input)?;
            let v = duap_canon::decode(&bytes).map_err(|e| e.to_string())?;
            println!("{}", duap_canon::json::to_json_pretty(&v));
            Ok(())
        }
        CanonCmd::Digest { input, domain, alg } => {
            let bytes = read(&input)?;
            duap_canon::decode(&bytes).map_err(|e| format!("input is not canonical: {e}"))?;
            let alg = HashAlg::parse(&alg).map_err(|e| e.to_string())?;
            println!("{}", Digest::of(alg, &domain, &bytes));
            Ok(())
        }
        CanonCmd::Check { input } => {
            let bytes = read(&input)?;
            match duap_canon::decode(&bytes) {
                Ok(v) => {
                    println!(
                        "{}",
                        serde_json::json!({
                            "canonical": true,
                            "bytes": bytes.len(),
                            "nodes": v.node_count(),
                            "depth": v.depth(),
                        })
                    );
                    Ok(())
                }
                Err(e) => {
                    println!(
                        "{}",
                        serde_json::json!({ "canonical": false, "reason": e.to_string() })
                    );
                    Err("input is not canonical".into())
                }
            }
        }
    }
}

fn grant_cmd(g: GrantCmd) -> R {
    match g {
        GrantCmd::Show { input } => {
            let bytes = read(&input)?;
            let grant = duap_auth::Grant::from_canonical(&bytes)
                .map_err(|e| format!("not a canonical grant: {e}"))?;
            grant.validate().map_err(|e| e.to_string())?;
            let v = duap_canon::to_value(&grant).map_err(|e| e.to_string())?;
            println!("{}", duap_canon::json::to_json_pretty(&v));
            eprintln!("digest: {}", grant.digest().map_err(|e| e.to_string())?);
            eprintln!(
                "terms: {} (default effect {:?})",
                grant.terms.len(),
                grant.default_effect
            );
            Ok(())
        }
        GrantCmd::Verify { input, registry } => {
            let env = load_envelope(&input)?;
            let reg = load_registry(&registry)?;
            let signers = env
                .verify(
                    &reg,
                    &SuitePolicy::draft_default(),
                    &VerificationContext::archival(Timestamp::now().0),
                )
                .map_err(|e| e.to_string())?;
            let grant: duap_auth::Grant = env.open().map_err(|e| e.to_string())?;
            grant.validate().map_err(|e| e.to_string())?;
            for kid in &signers {
                let rec = reg.get(kid).ok_or("signer vanished from the registry")?;
                if !rec.has_role(KeyRole::AuthorizationSigner) {
                    return Err(format!("{kid} is not authorised to sign authorizations"));
                }
            }
            println!(
                "{}",
                serde_json::json!({
                    "verified": true,
                    "grant": grant.id.to_string(),
                    "epoch": grant.epoch,
                    "digest": grant.digest().map_err(|e| e.to_string())?.to_string(),
                    "signers": signers.iter().map(|k| k.to_string()).collect::<Vec<_>>(),
                })
            );
            Ok(())
        }
    }
}

fn event_cmd(e: EventCmd) -> R {
    match e {
        EventCmd::Validate { input } => {
            let bytes = load_event_bytes(&input)?;
            let ev = DataUsageEvent::from_canonical(&bytes).map_err(|e| e.to_string())?;
            match ev.validate() {
                Ok(()) => {
                    println!(
                        "{}",
                        serde_json::json!({
                            "valid": true,
                            "digest": ev.digest().map_err(|e| e.to_string())?.to_string(),
                            "bytes": bytes.len(),
                        })
                    );
                    Ok(())
                }
                Err(err) => {
                    println!(
                        "{}",
                        serde_json::json!({ "valid": false, "reason": err.to_string() })
                    );
                    Err("event failed validation".into())
                }
            }
        }
        EventCmd::Show { input } => {
            let bytes = load_event_bytes(&input)?;
            let ev = DataUsageEvent::from_canonical(&bytes).map_err(|e| e.to_string())?;
            let v = duap_canon::to_value(&ev).map_err(|e| e.to_string())?;
            println!("{}", duap_canon::json::to_json_pretty(&v));
            eprintln!("digest: {}", ev.digest().map_err(|e| e.to_string())?);
            eprintln!(
                "metered: {} {} of {} for {}",
                ev.quantity.amount,
                ev.quantity.unit.code(),
                ev.data_class.code(),
                ev.purpose.code()
            );
            Ok(())
        }
        EventCmd::Verify { input, registry } => {
            let env = load_envelope(&input)?;
            let reg = load_registry(&registry)?;
            let signers = env
                .verify(
                    &reg,
                    &SuitePolicy::draft_default(),
                    &VerificationContext::archival(Timestamp::now().0),
                )
                .map_err(|e| e.to_string())?;
            let ev: DataUsageEvent = env.open().map_err(|e| e.to_string())?;
            ev.validate().map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::json!({
                    "verified": true,
                    "digest": ev.digest().map_err(|e| e.to_string())?.to_string(),
                    "signers": signers.iter().map(|k| k.to_string()).collect::<Vec<_>>(),
                    "controller": ev.controller.to_string(),
                    "operation": ev.operation.code(),
                })
            );
            Ok(())
        }
    }
}

/// Accept either canonical CBOR or a JSON-view document.
fn load_event_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let raw = read(path)?;
    if duap_canon::is_canonical(&raw) {
        return Ok(raw);
    }
    let text = String::from_utf8(raw)
        .map_err(|_| "input is neither canonical CBOR nor UTF-8 JSON".to_owned())?;
    let v = duap_canon::json::from_json_str(&text).map_err(|e| e.to_string())?;
    Ok(duap_canon::encode(&v))
}

fn receipt_cmd(r: ReceiptCmd) -> R {
    match r {
        ReceiptCmd::Verify { input, registry } => {
            let env = load_envelope(&input)?;
            let reg = load_registry(&registry)?;
            let (v, claims) = duap_sdk::verify_receipt(
                &env,
                &reg,
                &SuitePolicy::draft_default(),
                Timestamp::now(),
            )
            .map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::json!({
                    "verified": true,
                    "anchored": v.anchored,
                    "receipt": v.receipt.id().map_err(|e| e.to_string())?,
                    "controller": v.receipt.controller.to_string(),
                    "charge": v.receipt.charge.to_string(),
                    "subject_share": v.receipt.subject_share.map(|m| m.to_string()),
                    "claims": claims,
                })
            );
            eprintln!("This receipt establishes:");
            for c in claims.iter().filter(|c| c.established) {
                eprintln!("  + {}", c.statement);
            }
            eprintln!("This receipt does NOT establish:");
            for c in claims.iter().filter(|c| !c.established) {
                eprintln!("  - {}", c.statement);
            }
            Ok(())
        }
        ReceiptCmd::Claims { input } => {
            let bytes = read(&input)?;
            let receipt = duap_receipt::Receipt::from_canonical(&bytes)
                .map_err(|e| format!("not a canonical receipt: {e}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(&receipt.claims()).map_err(|e| e.to_string())?
            );
            Ok(())
        }
    }
}

fn price(
    rule_path: &Path,
    data_class: &str,
    operation: &str,
    purpose: &str,
    quantity: u64,
    currency: &str,
) -> R {
    let rule_bytes = json_view_to_canonical(rule_path)?;
    let rule: PricingRule =
        duap_canon::from_canonical_cbor(&rule_bytes).map_err(|e| format!("bad rule: {e}"))?;
    rule.validate().map_err(|e| e.to_string())?;
    let class = DataClass::parse(data_class).map_err(|e| e.to_string())?;
    let op = Operation::parse(operation).map_err(|e| e.to_string())?;
    let pp = Purpose::parse(purpose).map_err(|e| e.to_string())?;
    let cur = Currency::from_code(currency)
        .ok_or_else(|| format!("unknown currency {currency}; supply its ISO 4217 exponent"))?;

    let key = duap_meter::UsageKey {
        controller: "org:duap/cli"
            .parse()
            .map_err(|e: ModelError| e.to_string())?,
        processor: None,
        subject: Some(SubjectRef([0u8; 16])),
        scope_tag: duap_meter::ScopeTag::Subject,
        data_class: class,
        operation: op,
        purpose: pp,
        country: "ZZ".into(),
        unit: op.meter(),
        window_start: Timestamp::from_secs(0),
    };
    let counter = duap_meter::UsageCounter {
        quantity,
        event_count: 1,
        first: Timestamp::from_secs(0),
        last: Timestamp::from_secs(0),
        evidence_root: Digest::of(HashAlg::Sha2_256, "duap.test.v1", b""),
        evidence_size: 1,
    };
    let engine = duap_valuation::PriceEngine::new(cur);
    let b = engine
        .price(
            &key,
            &counter,
            &rule,
            &duap_valuation::PricingInputs::default(),
        )
        .map_err(|e| e.to_string())?;
    let (money, residue) = b.amount.round_to_money(Rounding::HalfEven);
    println!(
        "{}",
        serde_json::json!({
            "unit": key.unit.code(),
            "quantity": quantity,
            "combined_factor": b.combined_factor.to_string(),
            "multipliers": b.multipliers,
            "amount_nmu": b.amount.nmu.to_string(),
            "rounded": money.to_string(),
            "residue_nmu": residue.nmu.to_string(),
            "rule": b.rule,
        })
    );
    Ok(())
}

fn conformance_cmd(c: ConformanceCmd) -> R {
    match c {
        ConformanceCmd::Generate { dir } => {
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut total = 0;
            for (name, file) in duap_conformance::vectors::all() {
                total += file.vectors.len();
                write(&dir.join(name), file.to_json().as_bytes())?;
            }
            println!(
                "{}",
                serde_json::json!({ "vectors": total, "dir": dir.display().to_string() })
            );
            Ok(())
        }
        ConformanceCmd::Check { dir } => {
            let mut files = Vec::new();
            for (name, _) in duap_conformance::vectors::all() {
                let text = read_text(&dir.join(name))?;
                let f: duap_conformance::VectorFile =
                    serde_json::from_str(&text).map_err(|e| e.to_string())?;
                files.push((name.to_owned(), f));
            }
            let s = duap_conformance::vectors::self_check(&files);
            println!(
                "{}",
                serde_json::json!({ "passed": s.passed, "failed": s.failed })
            );
            for f in &s.failures {
                eprintln!("FAIL {}: {}", f.id, f.detail);
            }
            if s.ok() {
                Ok(())
            } else {
                Err("conformance check failed".into())
            }
        }
    }
}

fn taxonomy(what: &str) -> R {
    let out = match what {
        "classes" => serde_json::json!(
            DataClass::ALL
                .iter()
                .map(|c| serde_json::json!({
                    "code": c.code(),
                    "label": c.label(),
                    "sensitivity": c.sensitivity().code(),
                    "reid_risk": c.reid_risk(),
                    "special_category": c.special_categories(),
                }))
                .collect::<Vec<_>>()
        ),
        "operations" => serde_json::json!(
            Operation::ALL
                .iter()
                .map(|o| serde_json::json!({
                    "code": o.code(),
                    "family": o.family().code(),
                    "meter": o.meter().code(),
                    "derives": o.derives(),
                }))
                .collect::<Vec<_>>()
        ),
        "purposes" => serde_json::json!(
            Purpose::ALL
                .iter()
                .map(|p| serde_json::json!({
                    "code": p.code(),
                    "parent": p.parent().map(|x| x.code()),
                    "commercial": p.commercial(),
                }))
                .collect::<Vec<_>>()
        ),
        "units" => serde_json::json!(
            Unit::ALL
                .iter()
                .map(|u| serde_json::json!({ "code": u.code(), "additive": u.additive() }))
                .collect::<Vec<_>>()
        ),
        other => return Err(format!("unknown taxonomy view {other:?}")),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&out).map_err(|e| e.to_string())?
    );
    eprintln!(
        "ontology {} (sha256 {})",
        duap_model::taxonomy::ONTOLOGY_VERSION,
        &duap_model::taxonomy::ONTOLOGY_SHA256[..16]
    );
    Ok(())
}

/// Unused-import guard for the public-key type, which appears only in
/// generated JSON above.
const _: Option<PublicKey> = None;
