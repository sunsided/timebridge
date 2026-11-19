//! An application built on Jiff using `token-expiry`.

use temporal_core::{Timestamp, WallClock};
use temporal_jiff::{ToJiff, ToTemporal};
use token_expiry::TokenClaims;

/// The application's own clock, backed by Jiff.
struct JiffClock;

impl WallClock for JiffClock {
    fn now(&self) -> Timestamp {
        jiff::Timestamp::now()
            .to_temporal()
            .expect("Jiff timestamps always fit")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lifetime = jiff::SignedDuration::from_mins(30).to_temporal()?;
    let claims = TokenClaims::issue(&JiffClock, lifetime).ok_or("lifetime out of range")?;
    let expires: jiff::Timestamp = claims.expires_at.to_jiff()?;

    println!(
        "expires at {expires}, expired: {}",
        claims.is_expired(&JiffClock)
    );
    Ok(())
}
