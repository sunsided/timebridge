//! Tests for the wall readings of the ready-made clocks, which no `SPEC.md` rule covers.

use std::time::SystemTime;

use timebridge::{Timestamp, WallClock};
use timebridge_clock::{FixedClock, SystemClock};

/// A fixed clock reports its instant on every read.
#[test]
fn fixed_clock_reports_its_instant() {
    let clock = FixedClock(Timestamp::from_unix_seconds(42));

    assert_eq!(clock.now(), Timestamp::from_unix_seconds(42));
    assert_eq!(clock.now(), Timestamp::from_unix_seconds(42));
}

/// The system clock reads [`SystemTime::now`]: its reading lies between two direct reads.
#[test]
fn system_clock_reads_system_time() {
    let before = Timestamp::try_from(SystemTime::now()).unwrap();
    let reading = SystemClock.now();
    let after = Timestamp::try_from(SystemTime::now()).unwrap();

    assert!(before <= reading && reading <= after);
}
