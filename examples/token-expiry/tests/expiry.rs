//! Deterministic expiry tests using an injected fake clock.

use std::time::Duration;

use temporal_clock::ManualClock;
use temporal_core::{ExactDuration, Timestamp};
use token_expiry::TokenClaims;

/// A token is valid until the clock reaches its expiry, and expired from then on.
#[test]
fn expires_exactly_at_lifetime() {
    let clock = ManualClock::new(Timestamp::from_unix_seconds(1_700_000_000));
    let claims = TokenClaims::issue(&clock, ExactDuration::from_secs(60)).unwrap();

    clock.advance(Duration::from_secs(59));
    assert!(!claims.is_expired(&clock));

    clock.advance(Duration::from_secs(1));
    assert!(claims.is_expired(&clock));
}

/// Expiry follows wall time, so a backwards clock correction un-expires a token.
#[test]
fn follows_wall_clock_jumps() {
    let clock = ManualClock::new(Timestamp::from_unix_seconds(1_000));
    let claims = TokenClaims::issue(&clock, ExactDuration::from_secs(10)).unwrap();

    clock.set_wall(Timestamp::from_unix_seconds(2_000));
    assert!(claims.is_expired(&clock));

    clock.set_wall(Timestamp::from_unix_seconds(1_005));
    assert!(!claims.is_expired(&clock));
}
