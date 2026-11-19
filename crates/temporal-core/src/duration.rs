//! [`ExactDuration`]: a precise signed interval.

use core::time::Duration;

use crate::{RangeError, nanos};

/// A precise, signed interval of elapsed time with nanosecond resolution.
///
/// An `ExactDuration` always means the same number of nanoseconds, independent of any
/// calendar or time zone. "One day" in the calendar sense is *not* an `ExactDuration`: across a
/// DST transition it lasts 23 or 25 hours. Calendar-relative spans are a separate, future type.
///
/// The value is stored in floor form like [`crate::Timestamp`] (`DURATION-002`): a signed
/// second count plus a sub-second part in `0..1_000_000_000`. Minus half a second is
/// `(-1, 500_000_000)`. The range is therefore `i64::MIN` seconds up to `i64::MAX` seconds plus
/// 999,999,999 nanoseconds.
///
/// # Examples
///
/// ```
/// use temporal_core::ExactDuration;
///
/// let half = ExactDuration::from_nanos(-500_000_000).unwrap();
/// assert_eq!(half.parts(), (-1, 500_000_000));
/// assert!(half.is_negative());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExactDuration {
    /// Floored whole seconds. Must stay the first field so the derived `Ord` is correct.
    seconds: i64,
    /// Nonnegative sub-second remainder. Always in `0..1_000_000_000`.
    nanoseconds: u32,
}

impl ExactDuration {
    /// The empty interval.
    pub const ZERO: Self = Self::from_secs(0);

    /// The most negative representable interval: `i64::MIN` seconds.
    pub const MIN: Self = Self::from_secs(i64::MIN);

    /// The most positive representable interval: `i64::MAX` seconds and 999,999,999 ns.
    pub const MAX: Self = Self {
        seconds: i64::MAX,
        nanoseconds: nanos::PER_SECOND - 1,
    };

    /// Creates a duration from floor-form parts.
    ///
    /// Returns `None` when `nanoseconds` is not in `0..1_000_000_000`.
    #[must_use]
    pub const fn new(seconds: i64, nanoseconds: u32) -> Option<Self> {
        if nanoseconds >= nanos::PER_SECOND {
            return None;
        }

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Creates a duration of a whole number of seconds.
    #[must_use]
    pub const fn from_secs(seconds: i64) -> Self {
        Self {
            seconds,
            nanoseconds: 0,
        }
    }

    /// Creates a duration from a total nanosecond count.
    ///
    /// Returns `None` when the value is outside [`Self::MIN`]`..=`[`Self::MAX`].
    #[must_use]
    pub fn from_nanos(total: i128) -> Option<Self> {
        let (seconds, nanoseconds) = nanos::split(total)?;

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Returns the floor-form `(seconds, nanoseconds)` parts.
    #[must_use]
    pub const fn parts(self) -> (i64, u32) {
        (self.seconds, self.nanoseconds)
    }

    /// Returns the total length in nanoseconds. Lossless for every value.
    #[must_use]
    pub fn as_nanos(self) -> i128 {
        nanos::join(self.seconds, self.nanoseconds)
    }

    /// Returns `true` if the duration is shorter than zero.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.seconds < 0
    }

    /// Adds two durations, returning `None` on overflow.
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        Self::from_nanos(self.as_nanos() + rhs.as_nanos())
    }

    /// Subtracts `rhs` from `self`, returning `None` on overflow.
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        Self::from_nanos(self.as_nanos() - rhs.as_nanos())
    }

    /// Negates the duration, returning `None` for values whose negation is out of range,
    /// such as [`Self::MIN`].
    #[must_use]
    pub fn checked_neg(self) -> Option<Self> {
        Self::from_nanos(-self.as_nanos())
    }
}

impl TryFrom<Duration> for ExactDuration {
    type Error = RangeError;

    /// Converts an unsigned [`Duration`] losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError`] when the duration exceeds [`ExactDuration::MAX`].
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let seconds = i64::try_from(value.as_secs()).map_err(|_| RangeError)?;

        Ok(Self {
            seconds,
            nanoseconds: value.subsec_nanos(),
        })
    }
}

impl TryFrom<ExactDuration> for Duration {
    type Error = RangeError;

    /// Converts a nonnegative [`ExactDuration`] losslessly.
    ///
    /// # Errors
    ///
    /// Returns [`RangeError`] when the duration is negative, since [`Duration`] is unsigned.
    fn try_from(value: ExactDuration) -> Result<Self, Self::Error> {
        let seconds = u64::try_from(value.seconds).map_err(|_| RangeError)?;

        Ok(Duration::new(seconds, value.nanoseconds))
    }
}
