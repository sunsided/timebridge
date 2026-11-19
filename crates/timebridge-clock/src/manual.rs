//! [`ManualClock`]: a clock driven explicitly by test code.

use std::sync::{Mutex, PoisonError};
use std::time::Duration;

use timebridge::{ExactDuration, MonotonicClock, Timestamp, WallClock};

/// A clock whose wall time and elapsed time only change when told to.
///
/// [`ManualClock::advance`] moves both readings forward, like real time passing.
/// [`ManualClock::set_wall`] moves only the wall reading, in either direction, like an NTP
/// correction or a manual clock change. The monotonic reading never decreases (`CLOCK-003`).
///
/// Monotonic readings are the time elapsed since the clock was created.
///
/// The clock is `Sync`, so one instance can be shared between the code under test and the test.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use timebridge_clock::ManualClock;
/// use timebridge::{ExactDuration, MonotonicClock, Timestamp, WallClock};
///
/// let clock = ManualClock::new(Timestamp::from_unix_seconds(1_000));
/// let start = clock.read();
///
/// clock.advance(Duration::from_secs(5));
/// clock.set_wall(Timestamp::from_unix_seconds(0));
///
/// assert_eq!(clock.now(), Timestamp::from_unix_seconds(0));
/// assert_eq!(clock.elapsed(start), ExactDuration::from_secs(5));
/// ```
#[derive(Debug)]
pub struct ManualClock {
    /// Current readings, behind a mutex because the clock traits take `&self`.
    state: Mutex<State>,
}

/// The two readings of a [`ManualClock`].
#[derive(Debug, Clone, Copy)]
struct State {
    /// Current wall-clock instant.
    wall: Timestamp,
    /// Total time advanced since creation. Never negative, never decreases.
    elapsed: ExactDuration,
}

impl ManualClock {
    /// Creates a clock whose wall time starts at `start` and whose elapsed time starts at zero.
    #[must_use]
    pub fn new(start: Timestamp) -> Self {
        Self {
            state: Mutex::new(State {
                wall: start,
                elapsed: ExactDuration::ZERO,
            }),
        }
    }

    /// Lets `duration` pass: both wall time and elapsed time move forward by it.
    ///
    /// # Panics
    ///
    /// Panics if `duration` exceeds `i64::MAX` seconds, or if either reading would leave its
    /// representable range. Both readings are then left unchanged, so a caller that catches the
    /// panic can keep using the clock.
    pub fn advance(&self, duration: Duration) {
        let step = crate::exact(duration);

        self.update(|state| {
            // Compute both before writing, so a panic leaves the readings untouched.
            let wall = state.wall.checked_add(step).expect("wall time overflowed");
            let elapsed = state
                .elapsed
                .checked_add(step)
                .expect("elapsed time overflowed");
            *state = State { wall, elapsed };
        });
    }

    /// Jumps the wall time to `wall` without affecting elapsed time.
    pub fn set_wall(&self, wall: Timestamp) {
        self.update(|state| state.wall = wall);
    }

    /// Returns a copy of the current readings.
    fn snapshot(&self) -> State {
        *self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Applies `change` to the readings under the lock.
    ///
    /// Poisoning is ignored: `State` is `Copy` and every write is a whole-field assignment, so a
    /// panic in another thread cannot leave it half-updated.
    fn update(&self, change: impl FnOnce(&mut State)) {
        change(&mut self.state.lock().unwrap_or_else(PoisonError::into_inner));
    }
}

impl WallClock for ManualClock {
    fn now(&self) -> Timestamp {
        self.snapshot().wall
    }
}

impl MonotonicClock for ManualClock {
    type Instant = ExactDuration;

    fn read(&self) -> ExactDuration {
        self.snapshot().elapsed
    }

    /// Returns `end - start`.
    ///
    /// # Panics
    ///
    /// Never in practice: both readings are in `0..=i64::MAX` seconds, so their difference
    /// always fits.
    fn duration_between(&self, start: ExactDuration, end: ExactDuration) -> ExactDuration {
        end.checked_sub(start)
            .expect("difference of nonnegative readings fits")
    }
}

/// Overflow tests for [`ManualClock::advance`]: a panic must leave both readings unchanged.
#[cfg(test)]
mod tests {
    use std::panic::{self, AssertUnwindSafe};
    use std::time::Duration;

    use timebridge::{MonotonicClock, Timestamp, WallClock};

    use super::ManualClock;

    /// Calls `clock.advance(duration)`, expecting it to panic with `message`, and asserts that
    /// neither reading moved.
    fn assert_advance_panics_untouched(clock: &ManualClock, duration: Duration, message: &str) {
        let (wall, elapsed) = (clock.now(), clock.read());

        let payload = panic::catch_unwind(AssertUnwindSafe(|| clock.advance(duration)))
            .expect_err("advance should have panicked");
        let actual = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(String::as_str));

        assert_eq!(actual, Some(message));
        assert_eq!(clock.now(), wall);
        assert_eq!(clock.read(), elapsed);
    }

    /// An elapsed-time overflow leaves the wall reading untouched, though it had headroom.
    #[test]
    fn advance_elapsed_overflow_leaves_readings_untouched() {
        let clock = ManualClock::new(Timestamp::MIN);
        clock.advance(Duration::from_secs(i64::MAX.unsigned_abs()));

        assert_advance_panics_untouched(&clock, Duration::from_secs(1), "elapsed time overflowed");
    }

    /// A wall-time overflow leaves the elapsed reading untouched, though it had headroom.
    #[test]
    fn advance_wall_overflow_leaves_readings_untouched() {
        let clock = ManualClock::new(Timestamp::MAX);

        assert_advance_panics_untouched(&clock, Duration::from_secs(1), "wall time overflowed");
    }
}
