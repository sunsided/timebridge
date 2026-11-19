//! Lossless conversions between [`timebridge`] and [Chrono](chrono).
//!
//! | Chrono                          | timebridge                    |
//! | ------------------------------- | -------------------------------- |
//! | [`chrono::DateTime<Utc>`]       | [`timebridge::Timestamp`]     |
//! | [`chrono::TimeDelta`]           | [`timebridge::ExactDuration`] |
//!
//! Both directions are lossless (`CONVERSION-001`):
//!
//! - Chrono can represent a leap second (sub-second nanoseconds of 1e9 or more).
//!   [`timebridge::Timestamp`] cannot, so such values fail with [`Error::LeapSecond`]
//!   instead of collapsing into a neighbouring second (`CONVERSION-003`).
//! - Chrono's ranges are narrower than timebridge's, so [`ToChrono`] can fail with
//!   [`Error::OutOfRange`].
//!
//! Only `DateTime<Utc>` converts directly. For any other time zone, call
//! [`chrono::DateTime::to_utc`] first to drop the zone explicitly (`CONVERSION-002`).
//!
//! [`chrono::DateTime<Utc>`]: chrono::DateTime
//!
//! # Examples
//!
//! ```
//! use timebridge_chrono::{ToChrono, ToBridge};
//!
//! let chrono_time = chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap();
//! let neutral = chrono_time.to_bridge()?;
//! assert_eq!(neutral.unix_parts(), (1_700_000_000, 0));
//! assert_eq!(neutral.to_chrono()?, chrono_time);
//! # Ok::<(), timebridge_chrono::Error>(())
//! ```

#![no_std]
#![forbid(unsafe_code)]

use core::fmt;

use chrono::{DateTime, TimeDelta, Utc};
use timebridge::{ExactDuration, RangeError, Timestamp};

/// Why a conversion between Chrono and [`timebridge`] failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// The value is outside the destination's representable range.
    OutOfRange,
    /// The Chrono value lies inside a leap second, which [`timebridge::Timestamp`] cannot
    /// represent.
    LeapSecond,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange => fmt::Display::fmt(&RangeError, f),
            Self::LeapSecond => f.write_str("leap seconds cannot be represented as a Timestamp"),
        }
    }
}

impl core::error::Error for Error {}

impl From<RangeError> for Error {
    fn from(_: RangeError) -> Self {
        Self::OutOfRange
    }
}

/// Converts a Chrono value into its [`timebridge`] counterpart.
pub trait ToBridge {
    /// The [`timebridge`] type this value converts into.
    type Bridge;

    /// Converts losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`Error::LeapSecond`] for a `DateTime` inside a leap second, and
    /// [`Error::OutOfRange`] when the value is outside the destination's range.
    fn to_bridge(self) -> Result<Self::Bridge, Error>;
}

/// Converts a [`timebridge`] value into its Chrono counterpart.
pub trait ToChrono {
    /// The Chrono type this value converts into.
    type Chrono;

    /// Converts losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] when the value is outside Chrono's range.
    fn to_chrono(self) -> Result<Self::Chrono, Error>;
}

impl ToBridge for DateTime<Utc> {
    type Bridge = Timestamp;

    fn to_bridge(self) -> Result<Timestamp, Error> {
        // Chrono's `timestamp()` is already floored; the time of day is never negative.
        Timestamp::new(self.timestamp(), self.timestamp_subsec_nanos()).ok_or(Error::LeapSecond)
    }
}

impl ToBridge for TimeDelta {
    type Bridge = ExactDuration;

    fn to_bridge(self) -> Result<ExactDuration, Error> {
        // `num_seconds` and `subsec_nanos` truncate toward zero and share a sign, so their
        // sum is the exact total regardless of sign.
        let total =
            i128::from(self.num_seconds()) * 1_000_000_000 + i128::from(self.subsec_nanos());

        Ok(ExactDuration::from_nanos(total).ok_or(RangeError)?)
    }
}

impl ToChrono for Timestamp {
    type Chrono = DateTime<Utc>;

    fn to_chrono(self) -> Result<DateTime<Utc>, Error> {
        let (seconds, nanoseconds) = self.unix_parts();

        DateTime::from_timestamp(seconds, nanoseconds).ok_or(Error::OutOfRange)
    }
}

impl ToChrono for ExactDuration {
    type Chrono = TimeDelta;

    fn to_chrono(self) -> Result<TimeDelta, Error> {
        let (seconds, nanoseconds) = self.parts();

        TimeDelta::new(seconds, nanoseconds).ok_or(Error::OutOfRange)
    }
}
