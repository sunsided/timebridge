# timebridge-chrono

[![crates.io](https://img.shields.io/crates/v/timebridge-chrono.svg)](https://crates.io/crates/timebridge-chrono)
[![docs.rs](https://img.shields.io/docsrs/timebridge-chrono)](https://docs.rs/timebridge-chrono)
[![CI](https://github.com/sunsided/timebridge/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/timebridge/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/timebridge/graph/badge.svg)](https://codecov.io/gh/sunsided/timebridge)
[![MSRV](https://img.shields.io/crates/msrv/timebridge-chrono)](https://crates.io/crates/timebridge-chrono)
[![License](https://img.shields.io/crates/l/timebridge-chrono)](https://github.com/sunsided/timebridge#license)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Lossless conversions between [`timebridge`](https://crates.io/crates/timebridge) and [Chrono](https://crates.io/crates/chrono).

`chrono::DateTime<Utc>` and `chrono::TimeDelta` convert to `timebridge::Timestamp` and
`timebridge::ExactDuration` and back. Chrono leap seconds have no `Timestamp` equivalent and fail
with `Error::LeapSecond` instead of collapsing into a neighbouring second. Other time zones are
converted with `.to_utc()` first. See [SPEC.md](https://github.com/sunsided/timebridge/blob/main/SPEC.md).

## Install

```sh
cargo add timebridge-chrono
```

## Usage

```rust
use timebridge_chrono::{ToBridge, ToChrono};

let chrono_time = chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap();
let neutral = chrono_time.to_bridge()?;

assert_eq!(neutral.unix_parts(), (1_700_000_000, 0));
assert_eq!(neutral.to_chrono()?, chrono_time);
# Ok::<(), timebridge_chrono::Error>(())
```

## MSRV

Rust 1.88. The MSRV is a signal, not a contract: it may be raised in a minor release.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/sunsided/timebridge/blob/main/LICENSES/Apache-2.0.txt)
or [MIT license](https://github.com/sunsided/timebridge/blob/main/LICENSES/MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
