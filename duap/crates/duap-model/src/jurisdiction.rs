//! Jurisdiction tags.
//!
//! STATUS: PRODUCTION.
//!
//! A jurisdiction tag records *where the protocol participant asserts the
//! processing sits*, plus the regimes it asserts apply. DUAP does not decide
//! applicability: that depends on facts (establishment, targeting, the
//! subject's residence, sectoral status) that the protocol cannot observe. The
//! value of recording the assertion is that it is signed, so a later audit can
//! compare what was asserted against what was true.

use crate::error::{ModelError, Result};
use crate::taxonomy::Regime;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An asserted jurisdiction and regime set.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Jurisdiction {
    /// ISO 3166-1 alpha-2 country code, uppercase.
    #[serde(rename = "c")]
    pub country: String,
    /// ISO 3166-2 subdivision code without the country prefix, e.g. `CA` for
    /// California. `None` where the country has no relevant subdivision.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub subdivision: Option<String>,
    /// Regimes the participant asserts apply to this processing.
    #[serde(rename = "r", default, skip_serializing_if = "Vec::is_empty")]
    pub regimes: Vec<Regime>,
}

impl Jurisdiction {
    pub fn new(country: &str) -> Result<Jurisdiction> {
        if country.len() != 2 || !country.bytes().all(|b| b.is_ascii_uppercase()) {
            return Err(ModelError::Invalid {
                field: "jurisdiction.country",
                reason: format!("{country:?} is not an ISO 3166-1 alpha-2 code"),
            });
        }
        Ok(Jurisdiction {
            country: country.to_owned(),
            subdivision: None,
            regimes: Vec::new(),
        })
    }

    pub fn with_subdivision(mut self, s: &str) -> Result<Jurisdiction> {
        if s.is_empty() || s.len() > 3 || !s.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return Err(ModelError::Invalid {
                field: "jurisdiction.subdivision",
                reason: format!("{s:?} is not an ISO 3166-2 subdivision suffix"),
            });
        }
        self.subdivision = Some(s.to_owned());
        Ok(self)
    }

    pub fn with_regimes(mut self, r: impl IntoIterator<Item = Regime>) -> Jurisdiction {
        self.regimes = r.into_iter().collect();
        self.regimes.sort_unstable();
        self.regimes.dedup();
        self
    }

    pub fn asserts(&self, r: Regime) -> bool {
        self.regimes.contains(&r)
    }

    /// Whether this jurisdiction is in the European Economic Area, for the
    /// purposes of the reference policy engine's cross-border rules.
    pub fn is_eea(&self) -> bool {
        const EEA: [&str; 30] = [
            "AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR", "DE", "GR", "HU", "IS",
            "IE", "IT", "LV", "LI", "LT", "LU", "MT", "NL", "NO", "PL", "PT", "RO", "SK", "SI",
            "ES", "SE",
        ];
        EEA.contains(&self.country.as_str())
    }
}

impl fmt::Display for Jurisdiction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subdivision {
            Some(s) => write!(f, "{}-{}", self.country, s),
            None => f.write_str(&self.country),
        }
    }
}
