//! Tests for the parts of the Chrono adapter API that no `SPEC.md` rule covers.

use timebridge::RangeError;
use timebridge_chrono::Error;

/// Each error variant has its own message; out-of-range reuses the core crate's wording.
#[test]
fn error_display() {
    assert_eq!(Error::OutOfRange.to_string(), RangeError.to_string());
    assert_eq!(
        Error::LeapSecond.to_string(),
        "leap seconds cannot be represented as a Timestamp"
    );
}

/// A core [`RangeError`] converts into [`Error::OutOfRange`], so `?` works in adapter code.
#[test]
fn range_error_converts() {
    assert_eq!(Error::from(RangeError), Error::OutOfRange);
}
