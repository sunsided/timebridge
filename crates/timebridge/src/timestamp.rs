//! [`Timestamp`]: an instant in Unix time.

use crate::{ExactDuration, nanos};

#[cfg(feature = "std")]
mod system_time;

/// An instant in Unix time with nanosecond resolution.
///
/// The epoch is `1970-01-01T00:00:00Z` (`TIMESTAMP-001`) and the resolution is one nanosecond
/// (`TIMESTAMP-002`). The value is stored in floor form (`TIMESTAMP-003`): `seconds` is the
/// floor of the signed Unix time and the sub-second part is always in `0..1_000_000_000`. Half
/// a second before the epoch is therefore `(-1, 500_000_000)`.
///
/// A `Timestamp` carries no time zone, offset, calendar or leap-second information. It does
/// not represent leap seconds or other time scales such as TAI (`TIMESTAMP-005`).
///
/// The full `i64` second range is representable. Every engine supports a narrower range, so
/// conversions *into* an engine are fallible (`TIMESTAMP-004`).
///
/// # Examples
///
/// ```
/// use timebridge::Timestamp;
///
/// let before_epoch = Timestamp::from_unix_nanos(-500_000_000).unwrap();
/// assert_eq!(before_epoch.unix_parts(), (-1, 500_000_000));
/// assert!(before_epoch < Timestamp::UNIX_EPOCH);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Timestamp {
    /// Floored whole seconds since the Unix epoch. Must stay the first field so the derived
    /// `Ord` orders instants chronologically.
    seconds: i64,
    /// Nonnegative sub-second remainder. Always in `0..1_000_000_000`.
    nanoseconds: u32,
}

impl Timestamp {
    /// The Unix epoch, `1970-01-01T00:00:00Z`.
    pub const UNIX_EPOCH: Self = Self::from_unix_seconds(0);

    /// The earliest representable instant: `i64::MIN` seconds.
    pub const MIN: Self = Self::from_unix_seconds(i64::MIN);

    /// The latest representable instant: `i64::MAX` seconds and 999,999,999 ns.
    pub const MAX: Self = Self {
        seconds: i64::MAX,
        nanoseconds: nanos::PER_SECOND - 1,
    };

    /// Creates a timestamp from floor-form Unix parts.
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

    /// Creates a timestamp a whole number of seconds from the Unix epoch.
    #[must_use]
    pub const fn from_unix_seconds(seconds: i64) -> Self {
        Self {
            seconds,
            nanoseconds: 0,
        }
    }

    /// Creates a timestamp from total nanoseconds since the Unix epoch.
    ///
    /// Returns `None` when the value is outside [`Self::MIN`]`..=`[`Self::MAX`].
    #[must_use]
    pub fn from_unix_nanos(total: i128) -> Option<Self> {
        let (seconds, nanoseconds) = nanos::split(total)?;

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Returns the floor-form `(seconds, nanoseconds)` Unix parts.
    #[must_use]
    pub const fn unix_parts(self) -> (i64, u32) {
        (self.seconds, self.nanoseconds)
    }

    /// Returns total nanoseconds since the Unix epoch. Lossless for every value.
    #[must_use]
    pub fn as_unix_nanos(self) -> i128 {
        nanos::join(self.seconds, self.nanoseconds)
    }

    /// Returns the instant `duration` later, or `None` if it is out of range.
    #[must_use]
    pub fn checked_add(self, duration: ExactDuration) -> Option<Self> {
        Self::from_unix_nanos(self.as_unix_nanos() + duration.as_nanos())
    }

    /// Returns the instant `duration` earlier, or `None` if it is out of range.
    #[must_use]
    pub fn checked_sub(self, duration: ExactDuration) -> Option<Self> {
        Self::from_unix_nanos(self.as_unix_nanos() - duration.as_nanos())
    }

    /// Returns the signed interval from `earlier` to `self`.
    ///
    /// The result is negative when `earlier` is actually later. Returns `None` when the
    /// interval does not fit in an [`ExactDuration`], which can only happen near the ends of
    /// the range.
    #[must_use]
    pub fn checked_duration_since(self, earlier: Self) -> Option<ExactDuration> {
        ExactDuration::from_nanos(self.as_unix_nanos() - earlier.as_unix_nanos())
    }
}
