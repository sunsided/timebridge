//! Conformance tests for the `time` adapter, named after `SPEC.md` rules.

use proptest::prelude::*;
use time::{SignedDuration, UtcDateTime};
use timebridge::{ExactDuration, RangeError, Timestamp};
use timebridge_time::{ToBridge, ToTime};

/// `TIMESTAMP-003`: pre-epoch sub-second values keep floor form.
#[test]
fn timestamp_003_negative_fraction() {
    let time_value = UtcDateTime::from_unix_timestamp_nanos(-500_000_000).unwrap();
    let neutral = time_value.to_bridge().unwrap();

    assert_eq!(neutral.unix_parts(), (-1, 500_000_000));
    assert_eq!(neutral.to_time(), Ok(time_value));
}

/// `TIMESTAMP-004`: `time`'s range edges convert; values beyond them fail.
#[test]
fn timestamp_004_range_edges() {
    for edge in [UtcDateTime::MIN, UtcDateTime::MAX] {
        assert_eq!(edge.to_bridge().unwrap().to_time(), Ok(edge));
    }

    assert_eq!(Timestamp::MAX.to_time(), Err(RangeError));
    assert_eq!(SignedDuration::MIN.to_bridge(), Err(RangeError));
    assert_eq!(
        ExactDuration::MIN.to_time(),
        Ok(SignedDuration::new(i64::MIN, 0))
    );
    assert_eq!(ExactDuration::MAX.to_time(), Ok(SignedDuration::MAX));
}

proptest! {
    /// `CONVERSION-001`: every `time` instant round-trips through `Timestamp`.
    #[test]
    fn conversion_001_timestamp_round_trip(
        total in UtcDateTime::MIN.unix_timestamp_nanos()..=UtcDateTime::MAX.unix_timestamp_nanos()
    ) {
        let time_value = UtcDateTime::from_unix_timestamp_nanos(total).unwrap();
        let neutral = time_value.to_bridge().unwrap();

        prop_assert_eq!(neutral.as_unix_nanos(), total);
        prop_assert_eq!(neutral.to_time(), Ok(time_value));
    }

    /// `CONVERSION-001`: every `ExactDuration` round-trips through `time`.
    #[test]
    fn conversion_001_duration_round_trip(
        total in ExactDuration::MIN.as_nanos()..=ExactDuration::MAX.as_nanos()
    ) {
        let neutral = ExactDuration::from_nanos(total).unwrap();
        let time_value = neutral.to_time().unwrap();

        prop_assert_eq!(time_value.whole_nanoseconds(), total);
        prop_assert_eq!(time_value.to_bridge(), Ok(neutral));
    }
}
