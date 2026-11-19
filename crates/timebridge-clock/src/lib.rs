//! Ready-made implementations of the [`timebridge`] clock traits.
//!
//! | Clock           | [`WallClock`] | [`MonotonicClock`] | Use for                       |
//! | --------------- | ------------- | ------------------ | ----------------------------- |
//! | [`SystemClock`] | yes           | yes                | production                    |
//! | [`FixedClock`]  | yes           | no                 | tests that need one instant   |
//! | [`ManualClock`] | yes           | yes                | tests that simulate time flow |
//!
//! [`WallClock`]: timebridge::WallClock
//! [`MonotonicClock`]: timebridge::MonotonicClock

#![forbid(unsafe_code)]

mod fixed;
mod manual;
mod system;

pub use fixed::FixedClock;
pub use manual::ManualClock;
pub use system::SystemClock;

/// Converts a non-negative std interval into an [`timebridge::ExactDuration`].
///
/// # Panics
///
/// Panics when the interval exceeds `i64::MAX` seconds (about 292 billion years), which no
/// real or simulated clock reaches.
fn exact(duration: std::time::Duration) -> timebridge::ExactDuration {
    duration
        .try_into()
        .expect("clock intervals never exceed i64::MAX seconds")
}

/// Doctests for the code examples in README.md (not part of the API).
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}
