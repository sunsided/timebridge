# timebridge-time

[![crates.io](https://img.shields.io/crates/v/timebridge-time.svg)](https://crates.io/crates/timebridge-time)
[![docs.rs](https://img.shields.io/docsrs/timebridge-time)](https://docs.rs/timebridge-time)
[![CI](https://github.com/sunsided/timebridge/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/timebridge/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/timebridge/graph/badge.svg)](https://codecov.io/gh/sunsided/timebridge)
[![MSRV](https://img.shields.io/crates/msrv/timebridge-time)](https://crates.io/crates/timebridge-time)
[![License](https://img.shields.io/crates/l/timebridge-time)](https://github.com/sunsided/timebridge#license)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Lossless conversions between [`timebridge`](https://crates.io/crates/timebridge) and the [`time`](https://crates.io/crates/time) crate.

`time::UtcDateTime` and `time::SignedDuration` convert to `timebridge::Timestamp` and
`timebridge::ExactDuration` and back. A `time::OffsetDateTime` is converted with `.to_utc()`
first, which drops the offset explicitly. See [SPEC.md](https://github.com/sunsided/timebridge/blob/main/SPEC.md).

## Install

```sh
cargo add timebridge-time
```

## Usage

```rust
use timebridge_time::{ToBridge, ToTime};

let time_value = time::UtcDateTime::from_unix_timestamp(1_700_000_000).unwrap();
let neutral = time_value.to_bridge()?;

assert_eq!(neutral.unix_parts(), (1_700_000_000, 0));
assert_eq!(neutral.to_time()?, time_value);
# Ok::<(), timebridge::RangeError>(())
```

## MSRV

Rust 1.88. The MSRV is a signal, not a contract: it may be raised in a minor release.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/sunsided/timebridge/blob/main/LICENSES/Apache-2.0.txt)
or [MIT license](https://github.com/sunsided/timebridge/blob/main/LICENSES/MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
