# timebridge-clock

[![crates.io](https://img.shields.io/crates/v/timebridge-clock.svg)](https://crates.io/crates/timebridge-clock)
[![docs.rs](https://img.shields.io/docsrs/timebridge-clock)](https://docs.rs/timebridge-clock)
[![CI](https://github.com/sunsided/timebridge/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/timebridge/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/timebridge/graph/badge.svg)](https://codecov.io/gh/sunsided/timebridge)
[![MSRV](https://img.shields.io/crates/msrv/timebridge-clock)](https://crates.io/crates/timebridge-clock)
[![License](https://img.shields.io/crates/l/timebridge-clock)](https://github.com/sunsided/timebridge#license)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

System, fixed and manually driven clocks implementing the [`timebridge`](https://crates.io/crates/timebridge) clock traits.

| Clock         | `WallClock` | `MonotonicClock` | Use for                       |
| ------------- | ----------- | ---------------- | ----------------------------- |
| `SystemClock` | yes         | yes              | production                    |
| `FixedClock`  | yes         | no               | tests that need one instant   |
| `ManualClock` | yes         | yes              | tests that simulate time flow |

`ManualClock::advance` moves wall and elapsed time forward together; `ManualClock::set_wall`
jumps only the wall time, like an NTP correction, so tests can check that timeouts ignore it.

## Install

```sh
cargo add timebridge-clock
```

## Usage

```rust
use std::time::Duration;

use timebridge::{ExactDuration, MonotonicClock, Timestamp, WallClock};
use timebridge_clock::ManualClock;

let clock = ManualClock::new(Timestamp::from_unix_seconds(1_000));
let start = clock.read();

clock.advance(Duration::from_secs(5));
clock.set_wall(Timestamp::from_unix_seconds(0));

assert_eq!(clock.now(), Timestamp::from_unix_seconds(0));
assert_eq!(clock.elapsed(start), ExactDuration::from_secs(5));
```

## MSRV

Rust 1.88. The MSRV is a signal, not a contract: it may be raised in a minor release.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/sunsided/timebridge/blob/main/LICENSES/Apache-2.0.txt)
or [MIT license](https://github.com/sunsided/timebridge/blob/main/LICENSES/MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
