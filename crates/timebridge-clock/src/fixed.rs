//! [`FixedClock`]: a wall clock frozen at one instant.

use timebridge::{Timestamp, WallClock};

/// A wall clock that always reports the same instant.
///
/// # Examples
///
/// ```
/// use timebridge_clock::FixedClock;
/// use timebridge::{Timestamp, WallClock};
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
