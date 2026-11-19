//! Lossless conversions between [`timebridge`] and the [`time`] crate.
//!
//! | `time`                     | timebridge                    |
//! | -------------------------- | -------------------------------- |
//! | [`time::UtcDateTime`]      | [`timebridge::Timestamp`]     |
//! | [`time::SignedDuration`]   | [`timebridge::ExactDuration`] |
//!
//! Both directions are lossless (`CONVERSION-001`) and fail with [`RangeError`] instead of
//! clamping (`TIMESTAMP-004`). `time`'s default date range (years -9999 to 9999) is narrower
//! than [`timebridge::Timestamp`], so [`ToTime`] can fail for timestamps. `time`'s duration
//! range is slightly wider at the negative end, so [`ToBridge`] can fail for durations within
//! one second of [`time::SignedDuration::MIN`].
//!
//! [`time::OffsetDateTime`] has no direct conversion because it carries an offset that a
//! [`timebridge::Timestamp`] would drop. Call [`time::OffsetDateTime::to_utc`] first to
//! drop it explicitly (`CONVERSION-002`).
//!
//! # Examples
//!
//! ```
//! use timebridge_time::{ToBridge, ToTime};
//!
//! let time_value = time::UtcDateTime::from_unix_timestamp(1_700_000_000).unwrap();
//! let neutral = time_value.to_bridge()?;
//! assert_eq!(neutral.unix_parts(), (1_700_000_000, 0));
//! assert_eq!(neutral.to_time()?, time_value);
//! # Ok::<(), timebridge::RangeError>(())
//! ```

#![no_std]
#![forbid(unsafe_code)]

use time::{SignedDuration, UtcDateTime};
pub use timebridge::RangeError;
use timebridge::{ExactDuration, Timestamp};

/// Converts a `time` value into its [`timebridge`] counterpart.
pub trait ToBridge {
    /// The [`timebridge`] type this value converts into.
    type Bridge;

    /// Converts losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError`] when the value is outside the destination's range.
    fn to_bridge(self) -> Result<Self::Bridge, RangeError>;
}

/// Converts a [`timebridge`] value into its `time` counterpart.
pub trait ToTime {
    /// The `time` type this value converts into.
    type Time;

    /// Converts losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError`] when the value is outside `time`'s range.
    fn to_time(self) -> Result<Self::Time, RangeError>;
}

impl ToBridge for UtcDateTime {
    type Bridge = Timestamp;

    fn to_bridge(self) -> Result<Timestamp, RangeError> {
        Timestamp::from_unix_nanos(self.unix_timestamp_nanos()).ok_or(RangeError)
    }
}

impl ToBridge for SignedDuration {
    type Bridge = ExactDuration;

    fn to_bridge(self) -> Result<ExactDuration, RangeError> {
        ExactDuration::from_nanos(self.whole_nanoseconds()).ok_or(RangeError)
    }
}

impl ToTime for Timestamp {
    type Time = UtcDateTime;

    fn to_time(self) -> Result<UtcDateTime, RangeError> {
        UtcDateTime::from_unix_timestamp_nanos(self.as_unix_nanos()).map_err(|_| RangeError)
    }
}

impl ToTime for ExactDuration {
    type Time = SignedDuration;

    fn to_time(self) -> Result<SignedDuration, RangeError> {
        // Floor-form seconds always fit `time`'s truncated `i64` seconds, so this never
        // reaches the panicking path of `SignedDuration::new`.
        let (seconds, nanoseconds) = self.parts();
        let nanoseconds = i32::try_from(nanoseconds).expect("sub-second part is below 1e9");

        Ok(SignedDuration::new(seconds, nanoseconds))
    }
}

/// Doctests for the code examples in README.md (not part of the API).
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}
