//! Capability traits for sources of time.
//!
//! Wall time and elapsed time are deliberately separate capabilities (`CLOCK-001`). A wall
//! clock tells you *when* something happens and can jump when the system clock is adjusted.
//! A monotonic clock measures *how long* something takes and never runs backwards, but its
//! readings have no calendar meaning.

use crate::{ExactDuration, Timestamp};

/// A source of the current wall-clock instant.
///
/// Use it for audit timestamps, token expiry and anything compared against stored instants.
/// Readings may jump forwards or backwards between calls (`CLOCK-002`); never use them to
/// measure elapsed time or drive timeouts. Use [`MonotonicClock`] for that.
///
/// The trait is dyn-compatible, so libraries can accept `&dyn WallClock`.
///
/// # Examples
///
/// ```
/// use temporal_core::{Timestamp, WallClock};
///
/// struct Fixed(Timestamp);
///
/// impl WallClock for Fixed {
///     fn now(&self) -> Timestamp {
///         self.0
///     }
/// }
///
/// fn is_expired(clock: &dyn WallClock, expires_at: Timestamp) -> bool {
///     clock.now() >= expires_at
/// }
///
/// let clock = Fixed(Timestamp::from_unix_seconds(100));
/// assert!(is_expired(&clock, Timestamp::from_unix_seconds(100)));
/// ```
pub trait WallClock {
    /// Returns the current instant.
    fn now(&self) -> Timestamp;
}

/// A source of non-decreasing instants for measuring elapsed time.
///
/// Readings from one clock never decrease (`CLOCK-003`). They carry no calendar meaning and
/// are not comparable across different clock instances, which is why [`Self::Instant`] is an
/// associated type rather than [`Timestamp`].
pub trait MonotonicClock {
    /// An opaque reading of this clock.
    type Instant: Copy + Ord;

    /// Returns the current reading.
    fn read(&self) -> Self::Instant;

    /// Returns the elapsed time from `start` to `end`, both read from this clock.
    ///
    /// The result is negative when `end` precedes `start`.
    fn duration_between(&self, start: Self::Instant, end: Self::Instant) -> ExactDuration;

    /// Returns the time elapsed since `start`, a reading taken earlier from this clock.
    fn elapsed(&self, start: Self::Instant) -> ExactDuration {
        self.duration_between(start, self.read())
    }
}

impl<C: WallClock + ?Sized> WallClock for &C {
    fn now(&self) -> Timestamp {
        (**self).now()
    }
}

impl<C: MonotonicClock + ?Sized> MonotonicClock for &C {
    type Instant = C::Instant;

    fn read(&self) -> Self::Instant {
        (**self).read()
    }

    fn duration_between(&self, start: Self::Instant, end: Self::Instant) -> ExactDuration {
        (**self).duration_between(start, end)
    }
}
