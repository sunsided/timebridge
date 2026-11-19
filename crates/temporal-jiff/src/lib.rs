//! Lossless conversions between [`temporal_core`] and [Jiff](jiff).
//!
//! | Jiff                     | temporal-core                  |
//! | ------------------------ | ------------------------------ |
//! | [`jiff::Timestamp`]      | [`temporal_core::Timestamp`]   |
//! | [`jiff::SignedDuration`] | [`temporal_core::ExactDuration`] |
//!
//! Both directions are lossless (`CONVERSION-001`) and fail with [`RangeError`] instead of
//! clamping when a value does not fit (`TIMESTAMP-004`). Jiff's timestamp range (years -9999
//! to 9999) is narrower than [`temporal_core::Timestamp`], so [`ToJiff`] can fail for
//! timestamps. Jiff's duration range is slightly wider at the negative end, so [`ToTemporal`]
//! can fail for durations within one second of [`jiff::SignedDuration::MIN`].
//!
//! A `jiff::Zoned` has no direct conversion: call `Zoned::timestamp` to drop the
//! zone explicitly (`CONVERSION-002`).
//!
//! # Examples
//!
//! ```
//! use temporal_jiff::{ToJiff, ToTemporal};
//!
//! let jiff_time = jiff::Timestamp::from_second(1_700_000_000).unwrap();
//! let neutral = jiff_time.to_temporal()?;
//! assert_eq!(neutral.unix_parts(), (1_700_000_000, 0));
//! assert_eq!(neutral.to_jiff()?, jiff_time);
//! # Ok::<(), temporal_core::RangeError>(())
//! ```

#![no_std]

pub use temporal_core::RangeError;
use temporal_core::{ExactDuration, Timestamp};

/// Converts a Jiff value into its [`temporal_core`] counterpart.
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

/// Converts a [`temporal_core`] value into its Jiff counterpart.
pub trait ToJiff {
    /// The Jiff type this value converts into.
    type Jiff;

    /// Converts losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError`] when the value is outside Jiff's range.
    fn to_jiff(self) -> Result<Self::Jiff, RangeError>;
}

impl ToTemporal for jiff::Timestamp {
    type Temporal = Timestamp;

    fn to_temporal(self) -> Result<Timestamp, RangeError> {
        Timestamp::from_unix_nanos(self.as_nanosecond()).ok_or(RangeError)
    }
}

impl ToTemporal for jiff::SignedDuration {
    type Temporal = ExactDuration;

    fn to_temporal(self) -> Result<ExactDuration, RangeError> {
        ExactDuration::from_nanos(self.as_nanos()).ok_or(RangeError)
    }
}

impl ToJiff for Timestamp {
    type Jiff = jiff::Timestamp;

    fn to_jiff(self) -> Result<jiff::Timestamp, RangeError> {
        jiff::Timestamp::from_nanosecond(self.as_unix_nanos()).map_err(|_| RangeError)
    }
}

impl ToJiff for ExactDuration {
    type Jiff = jiff::SignedDuration;

    fn to_jiff(self) -> Result<jiff::SignedDuration, RangeError> {
        jiff::SignedDuration::try_from_nanos_i128(self.as_nanos()).ok_or(RangeError)
    }
}
