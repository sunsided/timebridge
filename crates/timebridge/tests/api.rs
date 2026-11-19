//! Tests for the parts of the `timebridge` API that no `SPEC.md` rule covers.

use std::cell::Cell;

use timebridge::{ExactDuration, MonotonicClock, RangeError, Timestamp, WallClock};

/// A wall clock frozen at one instant.
struct Fixed(Timestamp);

impl WallClock for Fixed {
    fn now(&self) -> Timestamp {
        self.0
    }
}

/// A monotonic clock whose readings are whole seconds that advance by one on every read.
struct Ticking {
    /// The reading the next [`MonotonicClock::read`] returns.
    next: Cell<i64>,
}

impl MonotonicClock for Ticking {
    type Instant = i64;

    fn read(&self) -> i64 {
        let reading = self.next.get();
        self.next.set(reading + 1);

        reading
    }

    fn duration_between(&self, start: i64, end: i64) -> ExactDuration {
        ExactDuration::from_secs(end - start)
    }
}

/// Reads a wall clock through a generic parameter, so a `&C` argument uses the blanket impl.
fn now_of<C: WallClock>(clock: C) -> Timestamp {
    clock.now()
}

/// Measures one read through a generic parameter, so a `&C` argument uses the blanket impl.
fn one_read_elapsed<C: MonotonicClock>(clock: C) -> ExactDuration {
    let start = clock.read();

    clock.elapsed(start)
}

/// The error message names the failure without engine-specific wording.
#[test]
fn range_error_display() {
    assert_eq!(
        RangeError.to_string(),
        "value is outside the destination's representable range"
    );
}

/// A reference to a wall clock, sized or `dyn`, reads the clock it points to.
#[test]
fn wall_clock_reference_forwards() {
    let clock = Fixed(Timestamp::from_unix_seconds(7));
    let dynamic: &dyn WallClock = &clock;

    assert_eq!(now_of(&clock), Timestamp::from_unix_seconds(7));
    assert_eq!(now_of(dynamic), Timestamp::from_unix_seconds(7));
}

/// A reference to a monotonic clock reads and measures with the clock it points to.
#[test]
fn monotonic_clock_reference_forwards() {
    let clock = Ticking { next: Cell::new(0) };

    assert_eq!(one_read_elapsed(&clock), ExactDuration::from_secs(1));
    assert_eq!(clock.next.get(), 2);
}
