# temporal

A common temporal protocol for Rust library boundaries: a tiny vocabulary of unambiguous values,
an agreed set of semantics ([SPEC.md](SPEC.md)), and capability traits that existing datetime
engines implement through adapters.

| Crate             | Contents                                                              |
| ----------------- | --------------------------------------------------------------------- |
| `temporal-core`   | `Timestamp`, `ExactDuration`, `WallClock`, `MonotonicClock`. `no_std`, zero deps. |
| `temporal-jiff`   | Lossless conversions to and from Jiff 1.0.                            |
| `temporal-chrono` | Lossless conversions to and from Chrono, rejecting leap seconds.      |
| `temporal-time`   | Lossless conversions to and from `time`.                              |
| `temporal-clock`  | `SystemClock`, `FixedClock`, `ManualClock`.                           |

A library exposes only `temporal-core` types:

```rust,ignore
pub fn is_expired(&self, clock: &dyn WallClock) -> bool {
    clock.now() >= self.expires_at
}
```

and each application converts at its edge with the adapter for its engine:

```rust,ignore
use temporal_chrono::ToTemporal;
let lifetime = chrono::TimeDelta::minutes(30).to_temporal()?;
```

See `examples/token-expiry` (with `with_jiff`, `with_chrono` and `with_time` examples) and the
comparison against using Jiff directly in [EVALUATION.md](EVALUATION.md).

Civil dates and times, zone rules, recurrence, serde and sqlx integrations are not part of this
iteration.

Note: the crate name `temporal_core` is already taken on crates.io; rename before publishing.
