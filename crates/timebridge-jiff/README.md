# timebridge-jiff

[![crates.io](https://img.shields.io/crates/v/timebridge-jiff.svg)](https://crates.io/crates/timebridge-jiff)
[![docs.rs](https://img.shields.io/docsrs/timebridge-jiff)](https://docs.rs/timebridge-jiff)
[![CI](https://github.com/sunsided/timebridge/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/timebridge/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/timebridge/graph/badge.svg)](https://codecov.io/gh/sunsided/timebridge)
[![MSRV](https://img.shields.io/crates/msrv/timebridge-jiff)](https://crates.io/crates/timebridge-jiff)
[![License](https://img.shields.io/crates/l/timebridge-jiff)](https://github.com/sunsided/timebridge#license)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Lossless conversions between [`timebridge`](https://crates.io/crates/timebridge) and [Jiff](https://crates.io/crates/jiff).

`jiff::Timestamp` and `jiff::SignedDuration` convert to `timebridge::Timestamp` and
`timebridge::ExactDuration` and back. Values outside the destination's range fail with
`RangeError` instead of being clamped. A `jiff::Zoned` is converted by first calling
`.timestamp()`, which drops the zone explicitly. See [SPEC.md](https://github.com/sunsided/timebridge/blob/main/SPEC.md).

## Install

```sh
cargo add timebridge-jiff
```

## Usage

```rust
use timebridge_jiff::{ToBridge, ToJiff};

let jiff_time = jiff::Timestamp::from_second(1_700_000_000).unwrap();
let neutral = jiff_time.to_bridge()?;

assert_eq!(neutral.unix_parts(), (1_700_000_000, 0));
assert_eq!(neutral.to_jiff()?, jiff_time);
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
