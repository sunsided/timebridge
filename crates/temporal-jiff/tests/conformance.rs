//! Conformance tests for the Jiff adapter, named after `SPEC.md` rules.

use proptest::prelude::*;
use temporal_core::{ExactDuration, RangeError, Timestamp};
use temporal_jiff::{ToJiff, ToTemporal};

/// `TIMESTAMP-003`: pre-epoch sub-second values map to floor form, not truncated form.
#[test]
fn timestamp_003_negative_fraction() {
    let jiff_value = jiff::Timestamp::from_nanosecond(-500_000_000).unwrap();
    let neutral = jiff_value.to_temporal().unwrap();

    assert_eq!(neutral.unix_parts(), (-1, 500_000_000));
    assert_eq!(neutral.to_jiff(), Ok(jiff_value));
}

/// `TIMESTAMP-004`: Jiff's range edges convert; one nanosecond beyond them fails.
#[test]
fn timestamp_004_range_edges() {
    for edge in [jiff::Timestamp::MIN, jiff::Timestamp::MAX] {
        assert_eq!(edge.to_temporal().unwrap().to_jiff(), Ok(edge));
    }

    let beyond = Timestamp::from_unix_nanos(jiff::Timestamp::MAX.as_nanosecond() + 1).unwrap();
    assert_eq!(beyond.to_jiff(), Err(RangeError));
    assert_eq!(Timestamp::MIN.to_jiff(), Err(RangeError));
}

/// `TIMESTAMP-004`: Jiff durations within one second of `MIN` do not fit.
#[test]
fn duration_range_edges() {
    assert_eq!(jiff::SignedDuration::MIN.to_temporal(), Err(RangeError));
    assert_eq!(
        jiff::SignedDuration::MAX.to_temporal(),
        Ok(ExactDuration::MAX)
    );
    assert_eq!(
        ExactDuration::MIN.to_jiff(),
        Ok(jiff::SignedDuration::new(i64::MIN, 0))
    );
}

proptest! {
    /// `CONVERSION-001`: every Jiff timestamp round-trips through `Timestamp`.
    #[test]
    fn conversion_001_timestamp_round_trip(
        total in jiff::Timestamp::MIN.as_nanosecond()..=jiff::Timestamp::MAX.as_nanosecond()
    ) {
        let jiff_value = jiff::Timestamp::from_nanosecond(total).unwrap();
        let neutral = jiff_value.to_temporal().unwrap();

        prop_assert_eq!(neutral.as_unix_nanos(), total);
        prop_assert_eq!(neutral.to_jiff(), Ok(jiff_value));
    }

    /// `CONVERSION-001`: every `ExactDuration` round-trips through Jiff.
    #[test]
    fn conversion_001_duration_round_trip(
        total in ExactDuration::MIN.as_nanos()..=ExactDuration::MAX.as_nanos()
    ) {
        let neutral = ExactDuration::from_nanos(total).unwrap();
        let jiff_value = neutral.to_jiff().unwrap();

        prop_assert_eq!(jiff_value.as_nanos(), total);
        prop_assert_eq!(jiff_value.to_temporal(), Ok(neutral));
    }
}
