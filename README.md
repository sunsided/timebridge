# timebridge

A common temporal protocol for Rust library boundaries: a tiny vocabulary of unambiguous values,
an agreed set of semantics ([SPEC.md](SPEC.md)), and capability traits that existing datetime
engines implement through adapters.

| Crate             | Contents                                                              |
| ----------------- | --------------------------------------------------------------------- |
| `timebridge`   | `Timestamp`, `ExactDuration`, `WallClock`, `MonotonicClock`. `no_std`, zero deps. |
| `timebridge-jiff`   | Lossless conversions to and from Jiff 1.0.                            |
| `timebridge-chrono` | Lossless conversions to and from Chrono, rejecting leap seconds.      |
| `timebridge-time`   | Lossless conversions to and from `time`.                              |
| `timebridge-clock`  | `SystemClock`, `FixedClock`, `ManualClock`.                           |

A library exposes only `timebridge` types:

```rust,ignore
pub fn is_expired(&self, clock: &dyn WallClock) -> bool {
    clock.now() >= self.expires_at
}
```

and each application converts at its edge with the adapter for its engine:

```rust,ignore
use timebridge_chrono::ToBridge;
let lifetime = chrono::TimeDelta::minutes(30).to_bridge()?;
```

See `examples/token-expiry` (with `with_jiff`, `with_chrono` and `with_time` examples) and the
comparison against using Jiff directly in [EVALUATION.md](EVALUATION.md).

Civil dates and times, zone rules, recurrence, serde and sqlx integrations are not part of this
iteration.

## Prior art

- [`unix-ts`](https://crates.io/crates/unix-ts): same `(i64 seconds, u32 nanos)` timestamp layout,
  but no engine adapters, clock traits or spec.
- [`timetraveler`](https://crates.io/crates/timetraveler): converts directly between Chrono, Jiff
  and `time`. There is no neutral type in between, so libraries still expose an engine type.
- [`clock-lib`](https://crates.io/crates/clock-lib) and similar crates (`chronoshift`, `monotony`,
  `qubit-clock`): injectable clocks, each with its own incompatible trait and value types.

timebridge combines a zero-dependency value vocabulary, lossless engine adapters, separate wall
and monotonic clock traits, and a normative spec with conformance tests.
