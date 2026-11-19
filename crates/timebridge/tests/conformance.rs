//! Conformance tests for the `timebridge` value types, named after `SPEC.md` rules.

#![cfg(feature = "std")]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use proptest::prelude::*;
use timebridge::{ExactDuration, RangeError, Timestamp};

/// Every total nanosecond count a [`Timestamp`] or [`ExactDuration`] can hold.
fn representable_nanos() -> impl Strategy<Value = i128> {
    Timestamp::MIN.as_unix_nanos()..=Timestamp::MAX.as_unix_nanos()
}

/// `TIMESTAMP-001`: the epoch is the zero instant, and agrees with `std`'s epoch.
#[test]
fn timestamp_001_epoch() {
    assert_eq!(Timestamp::UNIX_EPOCH.unix_parts(), (0, 0));
    assert_eq!(Timestamp::try_from(UNIX_EPOCH), Ok(Timestamp::UNIX_EPOCH));
}

/// `TIMESTAMP-002` / `TIMESTAMP-003`: nanosecond resolution in floor form.
#[test]
fn timestamp_003_floor_form() {
    let half_before = Timestamp::from_unix_nanos(-500_000_000).unwrap();
    let one_before = Timestamp::from_unix_nanos(-1).unwrap();

    assert_eq!(half_before.unix_parts(), (-1, 500_000_000));
    assert_eq!(one_before.unix_parts(), (-1, 999_999_999));
    assert_eq!(Timestamp::new(0, 1_000_000_000), None);
    assert_eq!(
        Timestamp::try_from(UNIX_EPOCH - Duration::from_millis(500)),
        Ok(half_before)
    );
}

/// `TIMESTAMP-004`: arithmetic and conversions out of range fail instead of clamping.
#[test]
fn timestamp_004_range() {
    let one_ns = ExactDuration::from_nanos(1).unwrap();

    assert_eq!(Timestamp::MAX.checked_add(one_ns), None);
    assert_eq!(Timestamp::MIN.checked_sub(one_ns), None);
    assert_eq!(Timestamp::MAX.checked_duration_since(Timestamp::MIN), None);
    assert_eq!(
        Timestamp::from_unix_nanos(Timestamp::MAX.as_unix_nanos() + 1),
        None
    );
}

/// `DURATION-002`: durations use the same floor form; negation can overflow at `MIN`.
#[test]
fn duration_002_floor_form() {
    assert_eq!(
        ExactDuration::from_nanos(-500_000_000).unwrap().parts(),
        (-1, 500_000_000)
    );
    assert_eq!(ExactDuration::MIN.checked_neg(), None);
    assert_eq!(
        ExactDuration::MAX.checked_add(ExactDuration::from_nanos(1).unwrap()),
        None
    );
}

/// `CONVERSION-001`: unsigned std durations convert losslessly; negative ones are rejected.
#[test]
fn conversion_001_std_duration() {
    let negative = ExactDuration::from_secs(-1);

    assert_eq!(Duration::try_from(negative), Err(RangeError));
    assert_eq!(
        ExactDuration::try_from(Duration::new(5, 7)),
        Ok(ExactDuration::new(5, 7).unwrap())
    );
    assert_eq!(ExactDuration::try_from(Duration::MAX), Err(RangeError));
}

proptest! {
    /// `TIMESTAMP-003`: parts and total nanoseconds round-trip for the whole range.
    #[test]
    fn timestamp_003_nanos_round_trip(total in representable_nanos()) {
        let timestamp = Timestamp::from_unix_nanos(total).unwrap();
        let (_, nanoseconds) = timestamp.unix_parts();

        prop_assert!(nanoseconds < 1_000_000_000);
        prop_assert_eq!(timestamp.as_unix_nanos(), total);
    }

    /// `TIMESTAMP-003`: the derived ordering is chronological.
    #[test]
    fn timestamp_003_order_is_chronological(a in representable_nanos(), b in representable_nanos()) {
        let (x, y) = (Timestamp::from_unix_nanos(a).unwrap(), Timestamp::from_unix_nanos(b).unwrap());

        prop_assert_eq!(x.cmp(&y), a.cmp(&b));
    }

    /// `CONVERSION-001`: `SystemTime` round-trips losslessly across the epoch.
    ///
    /// Inputs are whole 100 ns ticks within about ±3,600 years so they are exactly representable
    /// on every tier-1 platform, including Windows.
    #[test]
    fn conversion_001_system_time_round_trip(ticks in -(1_i128 << 60)..(1_i128 << 60)) {
        let timestamp = Timestamp::from_unix_nanos(ticks * 100).unwrap();
        let system = SystemTime::try_from(timestamp).unwrap();

        prop_assert_eq!(Timestamp::try_from(system), Ok(timestamp));
    }

    /// `DURATION-001`: adding then subtracting a duration is the identity when in range.
    #[test]
    fn duration_001_add_sub_inverse(start in representable_nanos(), step in representable_nanos()) {
        let timestamp = Timestamp::from_unix_nanos(start).unwrap();
        let duration = ExactDuration::from_nanos(step).unwrap();

        if let Some(later) = timestamp.checked_add(duration) {
            prop_assert_eq!(later.checked_sub(duration), Some(timestamp));
            prop_assert_eq!(later.checked_duration_since(timestamp), Some(duration));
        }
    }
}
