//! Event storage.
//!
//! STATUS: PRODUCTION for the in-memory store; PRODUCTION-CANDIDATE for the
//! SQLite store (feature `sqlite`).
//!
//! # The storage question
//!
//! `docs/adr/0010-event-storage.md` records the comparison with measured
//! numbers. In summary: the protocol needs an append-only sequence with
//! point lookup by digest and range scan by time, and it needs a Merkle root
//! over that sequence. It does *not* need multi-writer consensus, because
//! every entry is signed by its author and the log's integrity comes from
//! the Merkle structure rather than from agreement between replicas.
//!
//! That rules a blockchain out on cost grounds rather than ideology: a
//! permissioned chain buys ordering agreement DUAP already gets from a
//! single sequencer plus consistency proofs, at a throughput penalty of
//! several orders of magnitude. What DUAP does take from that world is the
//! transparency log, which provides detection of equivocation without
//! consensus.
//!
//! The trait exists so that the reference implementation can be exercised
//! against an in-memory store in tests and a durable one in deployment
//! without the clearing logic knowing which.

use duap_canon::digest::Digest;
use duap_model::prelude::*;

/// Errors from the storage layer.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("storage backend error: {0}")]
    Backend(String),
    #[error("{0}")]
    Model(#[from] ModelError),
    #[error("{0}")]
    Canon(#[from] duap_canon::CanonError),
}

/// A stored event with its canonical identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredEvent {
    pub digest: Digest,
    pub sequence: u64,
    pub event: DataUsageEvent,
}

/// Append-only event storage.
pub trait EventStore: Send {
    /// Append an event, returning its store sequence number.
    fn append(&mut self, digest: Digest, event: &DataUsageEvent) -> Result<u64, StoreError>;
    /// Fetch by canonical digest.
    fn get(&self, digest: &Digest) -> Result<Option<StoredEvent>, StoreError>;
    /// Every event whose occurrence time falls in the range, in store order.
    fn range(&self, period: TimeRange) -> Result<Vec<StoredEvent>, StoreError>;
    /// Number of stored events.
    fn len(&self) -> Result<u64, StoreError>;
    fn is_empty(&self) -> Result<bool, StoreError> {
        Ok(self.len()? == 0)
    }
}

/// In-memory store. The reference against which durable stores are tested.
#[derive(Debug, Default)]
pub struct MemoryStore {
    by_digest: std::collections::BTreeMap<String, usize>,
    events: Vec<StoredEvent>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl EventStore for MemoryStore {
    fn append(&mut self, digest: Digest, event: &DataUsageEvent) -> Result<u64, StoreError> {
        let key = digest.to_string();
        if let Some(i) = self.by_digest.get(&key) {
            return Ok(self.events[*i].sequence);
        }
        let seq = self.events.len() as u64;
        self.by_digest.insert(key, self.events.len());
        self.events.push(StoredEvent {
            digest,
            sequence: seq,
            event: event.clone(),
        });
        Ok(seq)
    }

    fn get(&self, digest: &Digest) -> Result<Option<StoredEvent>, StoreError> {
        Ok(self
            .by_digest
            .get(&digest.to_string())
            .and_then(|i| self.events.get(*i))
            .cloned())
    }

    fn range(&self, period: TimeRange) -> Result<Vec<StoredEvent>, StoreError> {
        Ok(self
            .events
            .iter()
            .filter(|e| period.contains(e.event.occurred_at))
            .cloned()
            .collect())
    }

    fn len(&self) -> Result<u64, StoreError> {
        Ok(self.events.len() as u64)
    }
}

/// A durable store backed by SQLite.
///
/// Events are held as canonical CBOR blobs plus the indexed columns the
/// clearing node actually queries on. The blob is the authority: the columns
/// are derived and are rebuilt from it, so a column-level bug cannot corrupt
/// the record that a signature covers.
#[cfg(feature = "sqlite")]
pub struct SqliteStore {
    conn: rusqlite::Connection,
}

#[cfg(feature = "sqlite")]
impl SqliteStore {
    pub fn open(path: &str) -> Result<Self, StoreError> {
        let conn =
            rusqlite::Connection::open(path).map_err(|e| StoreError::Backend(e.to_string()))?;
        Self::init(conn)
    }

    pub fn in_memory() -> Result<Self, StoreError> {
        let conn = rusqlite::Connection::open_in_memory()
            .map_err(|e| StoreError::Backend(e.to_string()))?;
        Self::init(conn)
    }

    fn init(conn: rusqlite::Connection) -> Result<Self, StoreError> {
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             CREATE TABLE IF NOT EXISTS events (
                 seq      INTEGER PRIMARY KEY AUTOINCREMENT,
                 digest   TEXT NOT NULL UNIQUE,
                 occurred INTEGER NOT NULL,
                 body     BLOB NOT NULL
             );
             CREATE INDEX IF NOT EXISTS events_occurred ON events(occurred);",
        )
        .map_err(|e| StoreError::Backend(e.to_string()))?;
        Ok(SqliteStore { conn })
    }
}

#[cfg(feature = "sqlite")]
impl EventStore for SqliteStore {
    fn append(&mut self, digest: Digest, event: &DataUsageEvent) -> Result<u64, StoreError> {
        let body = event.to_canonical()?;
        let key = digest.to_string();
        let existing: Option<i64> = self
            .conn
            .query_row(
                "SELECT seq FROM events WHERE digest = ?1",
                rusqlite::params![key],
                |r| r.get(0),
            )
            .ok();
        if let Some(seq) = existing {
            return Ok(seq as u64);
        }
        self.conn
            .execute(
                "INSERT INTO events (digest, occurred, body) VALUES (?1, ?2, ?3)",
                rusqlite::params![key, event.occurred_at.0 as i64, body],
            )
            .map_err(|e| StoreError::Backend(e.to_string()))?;
        Ok(self.conn.last_insert_rowid() as u64)
    }

    fn get(&self, digest: &Digest) -> Result<Option<StoredEvent>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT seq, body FROM events WHERE digest = ?1")
            .map_err(|e| StoreError::Backend(e.to_string()))?;
        let mut rows = stmt
            .query(rusqlite::params![digest.to_string()])
            .map_err(|e| StoreError::Backend(e.to_string()))?;
        match rows
            .next()
            .map_err(|e| StoreError::Backend(e.to_string()))?
        {
            None => Ok(None),
            Some(r) => {
                let seq: i64 = r.get(0).map_err(|e| StoreError::Backend(e.to_string()))?;
                let body: Vec<u8> = r.get(1).map_err(|e| StoreError::Backend(e.to_string()))?;
                Ok(Some(StoredEvent {
                    digest: *digest,
                    sequence: seq as u64,
                    event: DataUsageEvent::from_canonical(&body)?,
                }))
            }
        }
    }

    fn range(&self, period: TimeRange) -> Result<Vec<StoredEvent>, StoreError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT seq, digest, body FROM events
                 WHERE occurred >= ?1 AND occurred < ?2 ORDER BY seq",
            )
            .map_err(|e| StoreError::Backend(e.to_string()))?;
        let mut rows = stmt
            .query(rusqlite::params![
                period.start.0 as i64,
                period.end.0 as i64
            ])
            .map_err(|e| StoreError::Backend(e.to_string()))?;
        let mut out = Vec::new();
        while let Some(r) = rows
            .next()
            .map_err(|e| StoreError::Backend(e.to_string()))?
        {
            let seq: i64 = r.get(0).map_err(|e| StoreError::Backend(e.to_string()))?;
            let d: String = r.get(1).map_err(|e| StoreError::Backend(e.to_string()))?;
            let body: Vec<u8> = r.get(2).map_err(|e| StoreError::Backend(e.to_string()))?;
            out.push(StoredEvent {
                digest: d
                    .parse()
                    .map_err(|e: duap_canon::CanonError| StoreError::Backend(e.to_string()))?,
                sequence: seq as u64,
                event: DataUsageEvent::from_canonical(&body)?,
            });
        }
        Ok(out)
    }

    fn len(&self) -> Result<u64, StoreError> {
        let n: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))
            .map_err(|e| StoreError::Backend(e.to_string()))?;
        Ok(n as u64)
    }
}
