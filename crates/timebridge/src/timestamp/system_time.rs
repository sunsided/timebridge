//! Conversions between [`Timestamp`] and [`std::time::SystemTime`].

use core::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{RangeError, Timestamp, nanos};

impl TryFrom<SystemTime> for Timestamp {
    type Error = RangeError;

    /// Converts a system time losslessly, including instants before the epoch.
    ///
    /// `SystemTime` reports pre-epoch instants as a positive distance *before* the epoch; this
    /// conversion turns that into the floor form required by `TIMESTAMP-003`.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError`] when the instant is outside [`Timestamp::MIN`]`..=`[`Timestamp::MAX`].
    fn try_from(value: SystemTime) -> Result<Self, Self::Error> {
        let total = match value.duration_since(UNIX_EPOCH) {
            Ok(after) => nanos_of(after)?,
            Err(before) => -nanos_of(before.duration())?,
        };

        Timestamp::from_unix_nanos(total).ok_or(RangeError)
    }
}

impl TryFrom<Timestamp> for SystemTime {
    type Error = RangeError;

    /// Converts a timestamp losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError`] when the platform's `SystemTime` cannot represent the instant
    /// exactly: out of its range, or finer than its resolution (Windows counts 100 ns ticks).
    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        let total = value.as_unix_nanos();
        let distance = duration_of(total.unsigned_abs())?;
        let shifted = if total < 0 {
            UNIX_EPOCH.checked_sub(distance)
        } else {
            UNIX_EPOCH.checked_add(distance)
        }
        .ok_or(RangeError)?;

        // Platforms may silently round to their tick size; reject instead (CONVERSION-001).
        if Timestamp::try_from(shifted) != Ok(value) {
            return Err(RangeError);
        }

        Ok(shifted)
    }
}

/// Returns the length of `duration` in nanoseconds as a signed count.
fn nanos_of(duration: Duration) -> Result<i128, RangeError> {
    i128::try_from(duration.as_nanos()).map_err(|_| RangeError)
}

/// Builds a [`Duration`] from an unsigned nanosecond count.
fn duration_of(total: u128) -> Result<Duration, RangeError> {
    let per_second = u128::from(nanos::PER_SECOND);
    let seconds = u64::try_from(total / per_second).map_err(|_| RangeError)?;
    let nanoseconds = u32::try_from(total % per_second).expect("remainder of 1e9 fits in u32");

    Ok(Duration::new(seconds, nanoseconds))
}
