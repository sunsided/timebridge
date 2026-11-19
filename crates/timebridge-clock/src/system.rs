//! [`SystemClock`]: the operating system's clocks.

use std::time::{Instant, SystemTime};

use timebridge::{ExactDuration, MonotonicClock, Timestamp, WallClock};

/// The operating system's clocks: [`SystemTime`] for wall time, [`Instant`] for elapsed time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SystemClock;

impl WallClock for SystemClock {
    /// Reads [`SystemTime::now`].
    ///
    /// # Panics
    ///
    /// Panics if the system clock reports an instant outside [`Timestamp`]'s `i64`-second
    /// range, which no supported platform can do.
    fn now(&self) -> Timestamp {
        SystemTime::now()
            .try_into()
            .expect("system time fits in an i64-second Timestamp")
    }
}

impl MonotonicClock for SystemClock {
    type Instant = Instant;

    fn read(&self) -> Instant {
        Instant::now()
    }

    fn duration_between(&self, start: Instant, end: Instant) -> ExactDuration {
        match end.checked_duration_since(start) {
            Some(forward) => crate::exact(forward),
            None => crate::exact(start.duration_since(end))
                .checked_neg()
                .expect("a positive ExactDuration always negates"),
        }
    }
}
