//! Conformance tests for the clock implementations, named after `SPEC.md` rules.

use std::time::Duration;

use timebridge::{ExactDuration, MonotonicClock, Timestamp, WallClock};
use timebridge_clock::{ManualClock, SystemClock};

/// `CLOCK-002` / `CLOCK-003`: a backwards wall jump leaves elapsed time untouched.
#[test]
fn clock_003_wall_jump_keeps_monotonic_reading() {
    let clock = ManualClock::new(Timestamp::from_unix_seconds(1_000));
    let start = clock.read();

    clock.advance(Duration::from_secs(3));
    clock.set_wall(Timestamp::from_unix_seconds(0));

    assert_eq!(clock.now(), Timestamp::from_unix_seconds(0));
    assert_eq!(clock.elapsed(start), ExactDuration::from_secs(3));
    assert!(clock.read() >= start);
}

/// An `advance` that overflows panics without moving either reading.
#[test]
fn advance_overflow_leaves_readings_untouched() {
    // Wall time has headroom left when elapsed time overflows.
    let clock = ManualClock::new(Timestamp::MIN);
    clock.advance(Duration::from_secs(i64::MAX.unsigned_abs()));
    let (wall, elapsed) = (clock.now(), clock.read());

    let result = std::panic::catch_unwind(|| clock.advance(Duration::from_secs(1)));

    assert!(result.is_err());
    assert_eq!(clock.now(), wall);
    assert_eq!(clock.read(), elapsed);
}

/// `CLOCK-003`: system readings never decrease, and reversed order yields a negative span.
#[test]
fn clock_003_system_monotonic_order() {
    let clock = SystemClock;
    let earlier = clock.read();
    std::thread::sleep(Duration::from_millis(1));
    let later = clock.read();

    let forward = clock.duration_between(earlier, later);
    let backward = clock.duration_between(later, earlier);

    assert!(later >= earlier);
    assert!(!forward.is_negative());
    assert_eq!(backward.checked_neg(), Some(forward));
}
