# timebridge

[![CI](https://github.com/sunsided/timebridge/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/timebridge/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/timebridge/graph/badge.svg)](https://codecov.io/gh/sunsided/timebridge)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](https://github.com/sunsided/timebridge#license)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A common temporal protocol for Rust library boundaries: a tiny vocabulary of unambiguous values,
an agreed set of semantics ([SPEC.md](https://github.com/sunsided/timebridge/blob/main/SPEC.md)), and capability traits that existing datetime
engines implement through adapters.

| Crate | Version | Contents |
| ----- | ------- | -------- |
| [`timebridge`](https://crates.io/crates/timebridge) | [![crates.io](https://img.shields.io/crates/v/timebridge.svg)](https://crates.io/crates/timebridge) | `Timestamp`, `ExactDuration`, `WallClock`, `MonotonicClock`. `no_std`, zero deps. |
| [`timebridge-jiff`](https://crates.io/crates/timebridge-jiff) | [![crates.io](https://img.shields.io/crates/v/timebridge-jiff.svg)](https://crates.io/crates/timebridge-jiff) | Lossless conversions to and from Jiff 1.0. |
| [`timebridge-chrono`](https://crates.io/crates/timebridge-chrono) | [![crates.io](https://img.shields.io/crates/v/timebridge-chrono.svg)](https://crates.io/crates/timebridge-chrono) | Lossless conversions to and from Chrono, rejecting leap seconds. |
| [`timebridge-time`](https://crates.io/crates/timebridge-time) | [![crates.io](https://img.shields.io/crates/v/timebridge-time.svg)](https://crates.io/crates/timebridge-time) | Lossless conversions to and from `time`. |
| [`timebridge-clock`](https://crates.io/crates/timebridge-clock) | [![crates.io](https://img.shields.io/crates/v/timebridge-clock.svg)](https://crates.io/crates/timebridge-clock) | `SystemClock`, `FixedClock`, `ManualClock`. |

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

See [`examples/token-expiry`](https://github.com/sunsided/timebridge/tree/main/examples/token-expiry) (with `with_jiff`, `with_chrono` and `with_time` examples) and the
comparison against using Jiff directly in [EVALUATION.md](https://github.com/sunsided/timebridge/blob/main/EVALUATION.md).

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

## MSRV

Rust 1.88, checked in CI. The MSRV is a signal, not a contract: it may be raised in a minor
release.

## Contributing

See [CONTRIBUTING.md](https://github.com/sunsided/timebridge/blob/main/CONTRIBUTING.md). Review rules are indexed in
[docs/review/README.md](https://github.com/sunsided/timebridge/blob/main/docs/review/README.md).

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/sunsided/timebridge/blob/main/LICENSES/Apache-2.0.txt)
or [MIT license](https://github.com/sunsided/timebridge/blob/main/LICENSES/MIT.txt) at your option. Code under
[`examples/`](https://github.com/sunsided/timebridge/tree/main/examples) is public domain
([CC0-1.0](https://github.com/sunsided/timebridge/blob/main/LICENSES/CC0-1.0.txt)), so you can copy it into anything.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
