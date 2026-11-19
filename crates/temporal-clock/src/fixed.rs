//! [`FixedClock`]: a wall clock frozen at one instant.

use temporal_core::{Timestamp, WallClock};

/// A wall clock that always reports the same instant.
///
/// # Examples
///
/// ```
/// use temporal_clock::FixedClock;
/// use temporal_core::{Timestamp, WallClock};
///
/// let clock = FixedClock(Timestamp::from_unix_seconds(42));
/// assert_eq!(clock.now(), Timestamp::from_unix_seconds(42));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedClock(pub Timestamp);

impl WallClock for FixedClock {
    fn now(&self) -> Timestamp {
        self.0
    }
}
