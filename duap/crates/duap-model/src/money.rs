//! Exact monetary arithmetic.
//!
//! STATUS: PRODUCTION.
//!
//! # Why not floats
//!
//! Floating point cannot represent 0.01 exactly, so a system that prices in
//! floats and settles in integers loses or invents money at a rate that grows
//! with volume. At DUAP's target scale (trillions of events) even a
//! 2^-53 relative error per operation accumulates into real money, and
//! reconciliation against a bank statement becomes impossible. Every amount in
//! DUAP is an integer.
//!
//! # Two scales
//!
//! * [`Money`] holds an integer number of **minor units** (cents, pence, yen).
//!   This is the settlement scale: invoices, ledger postings and payments.
//! * [`Precise`] holds an integer number of **nano-minor-units** (1e-9 of a
//!   minor unit). This is the computation scale: per-event prices are far
//!   smaller than a cent, and rounding each one to a cent would either zero
//!   them all or overcharge by orders of magnitude.
//!
//! Rounding happens exactly once per invoice line, via
//! [`Precise::round_to_money`], and the residue is returned rather than
//! discarded so that the ledger can prove nothing was created or destroyed.
//! `docs/adr/0008-monetary-arithmetic.md` records the decision;
//! `duap-ledger` asserts the conservation invariant in tests.

use crate::error::{ModelError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Serde helper encoding `i128` as a canonical decimal string.
///
/// The DUAP canonical data model caps integers at the 64-bit CBOR range, but
/// monetary sums need headroom above it: a billion events at nano-minor-unit
/// precision already exceeds `u64`. Rather than widen the data model -- which
/// would mean bignum tags, and tags are banned -- amounts are carried as
/// decimal text with exactly one representation per value: no leading `+`, no
/// leading zeros, no `-0`. A non-canonical spelling is rejected on parse, so
/// two encodings of the same amount cannot both be accepted.
pub mod i128_str {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &i128, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&v.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<i128, D::Error> {
        let s = String::deserialize(d)?;
        parse_canonical(&s).map_err(serde::de::Error::custom)
    }

    /// Parse a canonical decimal integer, rejecting alternative spellings.
    pub fn parse_canonical(s: &str) -> Result<i128, String> {
        let body = s.strip_prefix('-').unwrap_or(s);
        if body.is_empty() || !body.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("{s:?} is not a decimal integer"));
        }
        if body.len() > 1 && body.starts_with('0') {
            return Err(format!("{s:?} has a leading zero"));
        }
        if s == "-0" {
            return Err("negative zero is not a canonical amount".to_owned());
        }
        s.parse::<i128>().map_err(|e| format!("{s:?}: {e}"))
    }
}

/// An ISO 4217 currency code plus the number of decimal digits in its minor
/// unit.
///
/// The exponent is carried explicitly because it is not derivable from the
/// code: JPY has 0, USD has 2, BHD has 3. Carrying it in the value means a
/// receipt is interpretable without a currency table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Currency {
    /// Three uppercase ASCII letters.
    pub code: [u8; 3],
    /// Decimal digits in the minor unit (0, 2 or 3 in practice).
    pub exponent: u8,
}

impl Currency {
    pub const USD: Currency = Currency {
        code: *b"USD",
        exponent: 2,
    };
    pub const EUR: Currency = Currency {
        code: *b"EUR",
        exponent: 2,
    };
    pub const GBP: Currency = Currency {
        code: *b"GBP",
        exponent: 2,
    };
    pub const JPY: Currency = Currency {
        code: *b"JPY",
        exponent: 0,
    };
    pub const CHF: Currency = Currency {
        code: *b"CHF",
        exponent: 2,
    };
    pub const BRL: Currency = Currency {
        code: *b"BRL",
        exponent: 2,
    };
    pub const INR: Currency = Currency {
        code: *b"INR",
        exponent: 2,
    };
    pub const KRW: Currency = Currency {
        code: *b"KRW",
        exponent: 0,
    };
    pub const BHD: Currency = Currency {
        code: *b"BHD",
        exponent: 3,
    };

    /// Look up a currency by ISO 4217 code.
    ///
    /// Only the currencies used by the reference deployment are listed. A
    /// production registry should be loaded from the ISO 4217 table; this
    /// function returns `None` rather than guessing an exponent, because
    /// guessing would silently misprice by a factor of 100.
    pub fn from_code(code: &str) -> Option<Currency> {
        let c: [u8; 3] = code.as_bytes().try_into().ok()?;
        [
            Currency::USD,
            Currency::EUR,
            Currency::GBP,
            Currency::JPY,
            Currency::CHF,
            Currency::BRL,
            Currency::INR,
            Currency::KRW,
            Currency::BHD,
        ]
        .into_iter()
        .find(|cur| cur.code == c)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.code).unwrap_or("???")
    }

    /// 10^exponent, the number of minor units in one major unit.
    pub const fn scale(&self) -> i128 {
        let mut s = 1i128;
        let mut i = 0;
        while i < self.exponent {
            s *= 10;
            i += 1;
        }
        s
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An exact monetary amount in minor units. May be negative (credits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Money {
    pub currency: Currency,
    /// Signed count of minor units.
    #[serde(with = "i128_str")]
    pub minor: i128,
}

impl Money {
    pub const fn new(currency: Currency, minor: i128) -> Money {
        Money { currency, minor }
    }

    pub const fn zero(currency: Currency) -> Money {
        Money { currency, minor: 0 }
    }

    pub fn is_zero(&self) -> bool {
        self.minor == 0
    }

    fn same_currency(&self, other: &Money) -> Result<()> {
        if self.currency != other.currency {
            return Err(ModelError::CurrencyMismatch {
                a: self.currency.to_string(),
                b: other.currency.to_string(),
            });
        }
        Ok(())
    }

    pub fn add(&self, other: &Money) -> Result<Money> {
        self.same_currency(other)?;
        Ok(Money {
            currency: self.currency,
            minor: self
                .minor
                .checked_add(other.minor)
                .ok_or(ModelError::Overflow("Money::add"))?,
        })
    }

    pub fn sub(&self, other: &Money) -> Result<Money> {
        self.same_currency(other)?;
        Ok(Money {
            currency: self.currency,
            minor: self
                .minor
                .checked_sub(other.minor)
                .ok_or(ModelError::Overflow("Money::sub"))?,
        })
    }

    pub fn neg(&self) -> Money {
        Money {
            currency: self.currency,
            minor: -self.minor,
        }
    }

    /// Sum a sequence, returning `None` for an empty sequence (no currency to
    /// attribute the zero to).
    pub fn sum<'a, I: IntoIterator<Item = &'a Money>>(it: I) -> Result<Option<Money>> {
        let mut acc: Option<Money> = None;
        for m in it {
            acc = Some(match acc {
                None => *m,
                Some(a) => a.add(m)?,
            });
        }
        Ok(acc)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = self.currency.scale();
        let neg = self.minor < 0;
        let abs = self.minor.unsigned_abs();
        let major = abs / scale as u128;
        let minor = abs % scale as u128;
        if self.currency.exponent == 0 {
            write!(
                f,
                "{}{} {}",
                if neg { "-" } else { "" },
                major,
                self.currency
            )
        } else {
            write!(
                f,
                "{}{}.{:0width$} {}",
                if neg { "-" } else { "" },
                major,
                minor,
                self.currency,
                width = self.currency.exponent as usize
            )
        }
    }
}

/// Rounding modes for converting a precise amount to settled money.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rounding {
    /// Round half away from zero. Simple, but biased away from zero.
    HalfUp,
    /// Round half to even ("banker's rounding"). Unbiased over many lines;
    /// the DUAP default.
    HalfEven,
    /// Toward negative infinity.
    Floor,
    /// Toward positive infinity.
    Ceil,
    /// Toward zero. Used where a payer must never be over-charged by
    /// rounding (the residue accrues to the payer).
    TowardZero,
}

/// Number of nano-minor-units in one minor unit.
pub const NANO: i128 = 1_000_000_000;

/// An exact amount at computation scale: nano-minor-units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Precise {
    pub currency: Currency,
    /// Signed count of 1e-9 minor units.
    #[serde(with = "i128_str")]
    pub nmu: i128,
}

impl Precise {
    pub const fn new(currency: Currency, nmu: i128) -> Precise {
        Precise { currency, nmu }
    }

    pub const fn zero(currency: Currency) -> Precise {
        Precise { currency, nmu: 0 }
    }

    pub fn from_money(m: Money) -> Result<Precise> {
        Ok(Precise {
            currency: m.currency,
            nmu: m
                .minor
                .checked_mul(NANO)
                .ok_or(ModelError::Overflow("Precise::from_money"))?,
        })
    }

    pub fn add(&self, other: &Precise) -> Result<Precise> {
        if self.currency != other.currency {
            return Err(ModelError::CurrencyMismatch {
                a: self.currency.to_string(),
                b: other.currency.to_string(),
            });
        }
        Ok(Precise {
            currency: self.currency,
            nmu: self
                .nmu
                .checked_add(other.nmu)
                .ok_or(ModelError::Overflow("Precise::add"))?,
        })
    }

    /// Multiply by an exact rational factor.
    pub fn mul_ratio(&self, r: Ratio) -> Result<Precise> {
        let num = self
            .nmu
            .checked_mul(r.num)
            .ok_or(ModelError::Overflow("Precise::mul_ratio"))?;
        if r.den == 0 {
            return Err(ModelError::DivZero("Precise::mul_ratio"));
        }
        // Truncating division here is exact-enough at nano scale and is
        // deterministic; the final rounding step is the one that matters.
        Ok(Precise {
            currency: self.currency,
            nmu: num / r.den as i128,
        })
    }

    /// Multiply by an integer count (e.g. a metered quantity).
    pub fn mul_u64(&self, n: u64) -> Result<Precise> {
        Ok(Precise {
            currency: self.currency,
            nmu: self
                .nmu
                .checked_mul(n as i128)
                .ok_or(ModelError::Overflow("Precise::mul_u64"))?,
        })
    }

    /// Round to settled money, returning the money and the unsettled residue.
    ///
    /// `money * NANO + residue == self`, always. The caller must post the
    /// residue somewhere (the DUAP ledger posts it to a per-currency rounding
    /// account) so that the conservation invariant holds.
    pub fn round_to_money(&self, mode: Rounding) -> (Money, Precise) {
        let q = self.nmu.div_euclid(NANO);
        let r = self.nmu.rem_euclid(NANO); // 0 <= r < NANO
        let rounded = match mode {
            Rounding::Floor => q,
            Rounding::Ceil => {
                if r == 0 {
                    q
                } else {
                    q + 1
                }
            }
            Rounding::TowardZero => {
                // `q` is the floor. For a non-negative value the floor is
                // already toward zero; for a negative one, toward zero is
                // the ceiling, which is `q + 1` unless the division was
                // exact.
                if self.nmu >= 0 || r == 0 { q } else { q + 1 }
            }
            Rounding::HalfUp => {
                // Half away from zero.
                if self.nmu >= 0 {
                    if r * 2 >= NANO { q + 1 } else { q }
                } else if r * 2 > NANO {
                    q + 1
                } else {
                    q
                }
            }
            Rounding::HalfEven => {
                let half = NANO / 2;
                match r.cmp(&half) {
                    std::cmp::Ordering::Less => q,
                    std::cmp::Ordering::Greater => q + 1,
                    std::cmp::Ordering::Equal => {
                        if q % 2 == 0 {
                            q
                        } else {
                            q + 1
                        }
                    }
                }
            }
        };
        let money = Money {
            currency: self.currency,
            minor: rounded,
        };
        let residue = Precise {
            currency: self.currency,
            nmu: self.nmu - rounded * NANO,
        };
        (money, residue)
    }

    pub fn sum<'a, I: IntoIterator<Item = &'a Precise>>(it: I) -> Result<Option<Precise>> {
        let mut acc: Option<Precise> = None;
        for p in it {
            acc = Some(match acc {
                None => *p,
                Some(a) => a.add(p)?,
            });
        }
        Ok(acc)
    }
}

impl fmt::Display for Precise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let neg = self.nmu < 0;
        let abs = self.nmu.unsigned_abs();
        write!(
            f,
            "{}{}.{:09} minor {}",
            if neg { "-" } else { "" },
            abs / NANO as u128,
            abs % NANO as u128,
            self.currency
        )
    }
}

/// An exact non-negative-denominator rational, used for rates, weights and
/// attribution shares.
///
/// Normalised on construction (gcd reduced, denominator positive) so that
/// equality is structural and the canonical encoding of an equal value is
/// unique -- which matters because ratios appear inside signed objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RatioRepr", into = "RatioRepr")]
pub struct Ratio {
    pub num: i128,
    pub den: u64,
}

#[derive(Serialize, Deserialize)]
struct RatioRepr {
    n: String,
    d: u64,
}

impl TryFrom<RatioRepr> for Ratio {
    type Error = ModelError;
    fn try_from(r: RatioRepr) -> Result<Ratio> {
        let num: i128 = r.n.parse().map_err(|_| ModelError::Invalid {
            field: "ratio.n",
            reason: r.n.clone(),
        })?;
        Ratio::new(num, r.d)
    }
}

impl From<Ratio> for RatioRepr {
    fn from(r: Ratio) -> RatioRepr {
        RatioRepr {
            n: r.num.to_string(),
            d: r.den,
        }
    }
}

impl Ratio {
    pub const ONE: Ratio = Ratio { num: 1, den: 1 };
    pub const ZERO: Ratio = Ratio { num: 0, den: 1 };

    pub fn new(num: i128, den: u64) -> Result<Ratio> {
        if den == 0 {
            return Err(ModelError::DivZero("Ratio::new"));
        }
        let g = gcd(num.unsigned_abs(), den as u128);
        let g = if g == 0 { 1 } else { g };
        Ok(Ratio {
            num: num / g as i128,
            den: (den as u128 / g) as u64,
        })
    }

    /// Construct from a percentage in basis points (1 bp = 0.01%).
    pub fn from_bps(bps: i64) -> Ratio {
        Ratio::new(bps as i128, 10_000).expect("denominator is non-zero")
    }

    pub fn mul(&self, other: Ratio) -> Result<Ratio> {
        let num = self
            .num
            .checked_mul(other.num)
            .ok_or(ModelError::Overflow("Ratio::mul"))?;
        let den = (self.den as u128)
            .checked_mul(other.den as u128)
            .and_then(|d| u64::try_from(d).ok())
            .ok_or(ModelError::Overflow("Ratio::mul"))?;
        Ratio::new(num, den)
    }

    pub fn add(&self, other: Ratio) -> Result<Ratio> {
        let den = (self.den as u128)
            .checked_mul(other.den as u128)
            .and_then(|d| u64::try_from(d).ok())
            .ok_or(ModelError::Overflow("Ratio::add"))?;
        let num = self
            .num
            .checked_mul(other.den as i128)
            .and_then(|a| a.checked_add(other.num.checked_mul(self.den as i128)?))
            .ok_or(ModelError::Overflow("Ratio::add"))?;
        Ratio::new(num, den)
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    /// Approximate value as `f64`. For display only: never feed this back
    /// into accounting.
    pub fn to_f64(&self) -> f64 {
        self.num as f64 / self.den as f64
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

impl FromStr for Ratio {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Ratio> {
        match s.split_once('/') {
            Some((n, d)) => {
                let num = n.trim().parse::<i128>().map_err(|_| ModelError::Invalid {
                    field: "ratio",
                    reason: s.to_owned(),
                })?;
                let den = d.trim().parse::<u64>().map_err(|_| ModelError::Invalid {
                    field: "ratio",
                    reason: s.to_owned(),
                })?;
                Ratio::new(num, den)
            }
            None => {
                let num = s.trim().parse::<i128>().map_err(|_| ModelError::Invalid {
                    field: "ratio",
                    reason: s.to_owned(),
                })?;
                Ratio::new(num, 1)
            }
        }
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
