//! Conformance tests for the Chrono adapter, named after `SPEC.md` rules.

use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use proptest::prelude::*;
use timebridge::{ExactDuration, Timestamp};
use timebridge_chrono::{Error, ToBridge, ToChrono};

/// `TIMESTAMP-003`: pre-epoch sub-second values keep floor form.
#[test]
fn timestamp_003_negative_fraction() {
    let neutral = Timestamp::from_unix_nanos(-500_000_000).unwrap();
    let chrono_value = neutral.to_chrono().unwrap();

    assert_eq!(chrono_value.timestamp_nanos_opt(), Some(-500_000_000));
    assert_eq!(chrono_value.to_bridge(), Ok(neutral));
}

/// `CONVERSION-003`: a leap second is rejected, never collapsed into a neighbour.
#[test]
fn conversion_003_leap_second_rejected() {
    let leap = NaiveDate::from_ymd_opt(2016, 12, 31)
        .unwrap()
        .and_hms_nano_opt(23, 59, 59, 1_500_000_000)
        .unwrap()
        .and_utc();

    assert_eq!(leap.to_bridge(), Err(Error::LeapSecond));
}

/// `TIMESTAMP-004`: Chrono's range edges convert; values beyond them fail.
#[test]
fn timestamp_004_range_edges() {
    for edge in [DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC] {
        assert_eq!(edge.to_bridge().unwrap().to_chrono(), Ok(edge));
    }

    assert_eq!(Timestamp::MAX.to_chrono(), Err(Error::OutOfRange));
    assert_eq!(ExactDuration::MAX.to_chrono(), Err(Error::OutOfRange));
    assert_eq!(
        TimeDelta::MIN.to_bridge().unwrap().to_chrono(),
        Ok(TimeDelta::MIN)
    );
}

proptest! {
    /// `CONVERSION-001`: Chrono timestamps round-trip through `Timestamp`.
    #[test]
    fn conversion_001_timestamp_round_trip(seconds in -8_000_000_000_000_i64..8_000_000_000_000, nanos in 0..1_000_000_000_u32) {
        let chrono_value = DateTime::from_timestamp(seconds, nanos).unwrap();
        let neutral = chrono_value.to_bridge().unwrap();

        prop_assert_eq!(neutral.unix_parts(), (seconds, nanos));
        prop_assert_eq!(neutral.to_chrono(), Ok(chrono_value));
    }

    /// `CONVERSION-001`: Chrono deltas round-trip through `ExactDuration`, sign included.
    #[test]
    fn conversion_001_delta_round_trip(millis in i64::MIN + 1..=i64::MAX, nanos in 0..1_000_000_i64) {
        let Some(delta) = TimeDelta::try_milliseconds(millis)
            .and_then(|delta| delta.checked_add(&TimeDelta::nanoseconds(nanos)))
        else {
            return Ok(());
        };
        let neutral = delta.to_bridge().unwrap();

        prop_assert_eq!(neutral.as_nanos(), i128::from(millis) * 1_000_000 + i128::from(nanos));
        prop_assert_eq!(neutral.to_chrono(), Ok(delta));
    }
}
