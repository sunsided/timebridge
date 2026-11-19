//! Demo library: token expiry built only on [`timebridge`].
//!
//! This crate stands in for a JWT or session library. Its public API speaks
//! [`timebridge::Timestamp`] and [`timebridge::WallClock`], so it depends on no datetime
//! engine. The `examples/` directory shows applications using Jiff, Chrono and `time` calling
//! it unchanged; `token-expiry-jiff` is the same library written directly against Jiff, for
//! comparison.

#![no_std]
#![forbid(unsafe_code)]

use timebridge::{ExactDuration, Timestamp, WallClock};

/// The time-related claims of an issued token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenClaims {
    /// When the token was issued.
    pub issued_at: Timestamp,
    /// The first instant at which the token is no longer valid.
    pub expires_at: Timestamp,
}

impl TokenClaims {
    /// Issues claims valid for `lifetime` starting at the clock's current instant.
    ///
    /// Returns `None` when `issued_at + lifetime` is out of range.
    #[must_use]
    pub fn issue(clock: &dyn WallClock, lifetime: ExactDuration) -> Option<Self> {
        let issued_at = clock.now();
        let expires_at = issued_at.checked_add(lifetime)?;

        Some(Self {
            issued_at,
            expires_at,
        })
    }

    /// Returns `true` once the clock has reached [`Self::expires_at`].
    #[must_use]
    pub fn is_expired(&self, clock: &dyn WallClock) -> bool {
        clock.now() >= self.expires_at
    }
}
