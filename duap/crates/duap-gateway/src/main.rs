//! Run the DUAP reference gateway.

use clap::Parser;
use duap_clearing::{ClearingConfig, ClearingNode};
use duap_crypto::{SecretKey, SuiteId};
use duap_gateway::{Gateway, Server};
use duap_model::prelude::*;
use std::sync::Arc;

#[derive(Parser)]
#[command(
    name = "duap-gateway",
    version,
    about = "DUAP reference ingest gateway"
)]
struct Cli {
    /// Address to bind.
    #[arg(long, default_value = "127.0.0.1:8787")]
    bind: String,
    /// Organisation identifier of this clearing node.
    #[arg(long, default_value = "org:duap/clearing-local")]
    org: String,
    /// Hex seed for the node's signing key. Generated if absent.
    #[arg(long)]
    seed_hex: Option<String>,
    /// Signature suite for the node's key.
    #[arg(long, default_value = "ed25519+ml-dsa-44")]
    suite: String,
    /// Settlement currency.
    #[arg(long, default_value = "EUR")]
    currency: String,
    /// Worker threads.
    #[arg(long, default_value_t = 32)]
    workers: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let org: OrgId = cli.org.parse()?;
    let suite: SuiteId = cli.suite.parse()?;
    let key = match &cli.seed_hex {
        Some(h) => {
            let raw = hex::decode(h)?;
            let seed: [u8; 32] = raw.try_into().map_err(|_| "seed must be 32 bytes of hex")?;
            SecretKey::from_seed(suite, seed)
        }
        None => SecretKey::generate(suite)?,
    };
    let currency = Currency::from_code(&cli.currency)
        .ok_or("unknown currency; supply one with a known ISO 4217 exponent")?;

    let node = ClearingNode::new(org.clone(), key, ClearingConfig::reference(currency));
    let node_key_id = node.public_key().key_id();
    let gw = Arc::new(Gateway::new(node));
    let g = Arc::clone(&gw);
    let server = Server::start(&cli.bind, cli.workers, move |req| g.handle(req))?;

    eprintln!(
        "{}",
        serde_json::json!({
            "event": "gateway.started",
            "bind": cli.bind,
            "port": server.port,
            "org": org.to_string(),
            "suite": suite.label(),
            "node_key": node_key_id.to_string(),
            "currency": currency.as_str(),
            "protocol": duap_canon::PROTOCOL_ID,
        })
    );
    eprintln!("gateway: press Ctrl-C to stop");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
