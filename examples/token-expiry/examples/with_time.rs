//! An application built on the `time` crate using `token-expiry`.

use timebridge::{Timestamp, WallClock};
use timebridge_time::{ToBridge, ToTime};
use token_expiry::TokenClaims;

/// The application's own clock, backed by `time`.
struct TimeClock;

impl WallClock for TimeClock {
    fn now(&self) -> Timestamp {
        time::UtcDateTime::now()
            .to_bridge()
            .expect("time's dates always fit")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lifetime = time::SignedDuration::minutes(30).to_bridge()?;
    let claims = TokenClaims::issue(&TimeClock, lifetime).ok_or("lifetime out of range")?;
    let expires: time::UtcDateTime = claims.expires_at.to_time()?;

    println!(
        "expires at {expires:?}, expired: {}",
        claims.is_expired(&TimeClock)
    );
    Ok(())
}
