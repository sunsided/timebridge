//! Lossless conversions between [`temporal_core`] and the [`time`] crate.
//!
//! | `time`                     | temporal-core                    |
//! | -------------------------- | -------------------------------- |
//! | [`time::UtcDateTime`]      | [`temporal_core::Timestamp`]     |
//! | [`time::SignedDuration`]   | [`temporal_core::ExactDuration`] |
//!
//! Both directions are lossless (`CONVERSION-001`) and fail with [`RangeError`] instead of
//! clamping (`TIMESTAMP-004`). `time`'s default date range (years -9999 to 9999) is narrower
//! than [`temporal_core::Timestamp`].
//!
//! [`time::OffsetDateTime`] has no direct conversion because it carries an offset that a
//! [`temporal_core::Timestamp`] would drop. Call [`time::OffsetDateTime::to_utc`] first to
//! drop it explicitly (`CONVERSION-002`).
//!
//! # Examples
//!
//! ```
//! use temporal_time::{ToTemporal, ToTime};
//!
//! let time_value = time::UtcDateTime::from_unix_timestamp(1_700_000_000).unwrap();
//! let neutral = time_value.to_temporal()?;
//! assert_eq!(neutral.unix_parts(), (1_700_000_000, 0));
//! assert_eq!(neutral.to_time()?, time_value);
//! # Ok::<(), temporal_core::RangeError>(())
//! ```

#![no_std]

pub use temporal_core::RangeError;
use temporal_core::{ExactDuration, Timestamp};
use time::{SignedDuration, UtcDateTime};

/// Converts a `time` value into its [`temporal_core`] counterpart.
pub trait ToTemporal {
    /// The [`temporal_core`] type this value converts into.
    type Temporal;

    /// Converts losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError`] when the value is outside the destination's range.
    fn to_temporal(self) -> Result<Self::Temporal, RangeError>;
}

/// Converts a [`temporal_core`] value into its `time` counterpart.
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

impl ToTemporal for UtcDateTime {
    type Temporal = Timestamp;

    fn to_temporal(self) -> Result<Timestamp, RangeError> {
        Timestamp::from_unix_nanos(self.unix_timestamp_nanos()).ok_or(RangeError)
    }
}

impl ToTemporal for SignedDuration {
    type Temporal = ExactDuration;

    fn to_temporal(self) -> Result<ExactDuration, RangeError> {
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
