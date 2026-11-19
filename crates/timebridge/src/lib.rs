//! Engine-neutral temporal vocabulary for Rust library boundaries.
//!
//! This crate defines a tiny set of unambiguous values and capability traits that libraries
//! can expose in their public APIs without choosing a datetime engine (Jiff, Chrono, `time`)
//! for their consumers. The normative contract lives in `SPEC.md` at the repository root;
//! rule identifiers such as `TIMESTAMP-003` refer to it.
//!
//! - [`Timestamp`]: an instant in Unix time with nanosecond resolution.
//! - [`ExactDuration`]: a precise signed interval with nanosecond resolution.
//! - [`WallClock`] and [`MonotonicClock`]: injectable sources of time.
//!
//! The crate has no dependencies and is `no_std`. The default `std` feature only adds
//! conversions to and from [`std::time::SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html).
//!
//! ```mermaid
//! graph LR
//!     Lib[Library API] -->|exposes| Core[timebridge values]
//!     Lib -->|asks for| Traits[WallClock / MonotonicClock]
//!     Jiff[timebridge-jiff] --> Core
//!     Chrono[timebridge-chrono] --> Core
//!     Time[timebridge-time] --> Core
//!     Clock[timebridge-clock] --> Traits
//! ```
//!
//! # Examples
//!
//! ```
//! use timebridge::{ExactDuration, Timestamp};
//!
//! let issued = Timestamp::from_unix_seconds(1_700_000_000);
//! let expires = issued.checked_add(ExactDuration::from_secs(3_600)).unwrap();
//! assert_eq!(expires.checked_duration_since(issued), Some(ExactDuration::from_secs(3_600)));
//! ```

#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
extern crate std;

mod clock;
mod duration;
mod error;
mod nanos;
mod timestamp;

pub use clock::{MonotonicClock, WallClock};
pub use duration::ExactDuration;
pub use error::RangeError;
pub use timestamp::Timestamp;

/// Doctests for the code examples in README.md (not part of the API).
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}
