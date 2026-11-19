//! Conformance tests for the clock implementations, named after `SPEC.md` rules.

use std::time::Duration;

use temporal_clock::{ManualClock, SystemClock};
use temporal_core::{ExactDuration, MonotonicClock, Timestamp, WallClock};

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
