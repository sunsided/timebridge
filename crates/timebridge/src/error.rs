//! The error shared by every fallible conversion in the protocol.

use core::fmt;

/// A value could not be represented in the destination type.
///
/// Returned when a conversion or arithmetic result falls outside the destination's
/// representable range (`TIMESTAMP-004`), or when a negative [`crate::ExactDuration`] is
/// converted to an unsigned duration type. Conversions never clamp or wrap instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RangeError;

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("value is outside the destination's representable range")
    }
}

impl core::error::Error for RangeError {}
