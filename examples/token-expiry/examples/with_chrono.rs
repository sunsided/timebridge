//! An application built on Chrono using `token-expiry`.

use temporal_chrono::{ToChrono, ToTemporal};
use temporal_core::{Timestamp, WallClock};
use token_expiry::TokenClaims;

/// The application's own clock, backed by Chrono.
struct ChronoClock;

impl WallClock for ChronoClock {
    fn now(&self) -> Timestamp {
        chrono::Utc::now()
            .to_temporal()
            .expect("the system clock is not inside a leap second")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lifetime = chrono::TimeDelta::minutes(30).to_temporal()?;
    let claims = TokenClaims::issue(&ChronoClock, lifetime).ok_or("lifetime out of range")?;
    let expires: chrono::DateTime<chrono::Utc> = claims.expires_at.to_chrono()?;

    println!(
        "expires at {expires}, expired: {}",
        claims.is_expired(&ChronoClock)
    );
    Ok(())
}
