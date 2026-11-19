//! Phase-4 baseline: the `token-expiry` demo written directly against Jiff.
//!
//! Identical behavior, but the public API exposes [`jiff::Timestamp`], so every consumer
//! depends on Jiff. Jiff has no clock trait, so the caller passes the current instant instead
//! of a clock.

#![no_std]
#![forbid(unsafe_code)]

use jiff::{SignedDuration, Timestamp};

/// The time-related claims of an issued token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenClaims {
    /// When the token was issued.
    pub issued_at: Timestamp,
    /// The first instant at which the token is no longer valid.
    pub expires_at: Timestamp,
}

impl TokenClaims {
    /// Issues claims valid for `lifetime` starting at `now`.
    ///
    /// Returns `None` when `now + lifetime` is out of Jiff's range.
    #[must_use]
    pub fn issue(now: Timestamp, lifetime: SignedDuration) -> Option<Self> {
        let expires_at = now.checked_add(lifetime).ok()?;

        Some(Self {
            issued_at: now,
            expires_at,
        })
    }

    /// Returns `true` once `now` has reached [`Self::expires_at`].
    #[must_use]
    pub fn is_expired(&self, now: Timestamp) -> bool {
        now >= self.expires_at
    }
}
