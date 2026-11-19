//! Ready-made implementations of the [`temporal_core`] clock traits.
//!
//! | Clock           | [`WallClock`] | [`MonotonicClock`] | Use for                       |
//! | --------------- | ------------- | ------------------ | ----------------------------- |
//! | [`SystemClock`] | yes           | yes                | production                    |
//! | [`FixedClock`]  | yes           | no                 | tests that need one instant   |
//! | [`ManualClock`] | yes           | yes                | tests that simulate time flow |
//!
//! [`WallClock`]: temporal_core::WallClock
//! [`MonotonicClock`]: temporal_core::MonotonicClock

mod fixed;
mod manual;
mod system;

pub use fixed::FixedClock;
pub use manual::ManualClock;
pub use system::SystemClock;

/// Converts a non-negative std interval into an [`temporal_core::ExactDuration`].
///
/// # Panics
///
/// Panics when the interval exceeds `i64::MAX` seconds (about 292 billion years), which no
/// real or simulated clock reaches.
fn exact(duration: std::time::Duration) -> temporal_core::ExactDuration {
    duration
        .try_into()
        .expect("clock intervals never exceed i64::MAX seconds")
}
