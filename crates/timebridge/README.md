# timebridge

[![crates.io](https://img.shields.io/crates/v/timebridge.svg)](https://crates.io/crates/timebridge)
[![docs.rs](https://img.shields.io/docsrs/timebridge)](https://docs.rs/timebridge)
[![CI](https://github.com/sunsided/timebridge/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/timebridge/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/timebridge/graph/badge.svg)](https://codecov.io/gh/sunsided/timebridge)
[![MSRV](https://img.shields.io/crates/msrv/timebridge)](https://crates.io/crates/timebridge)
[![License](https://img.shields.io/crates/l/timebridge)](https://github.com/sunsided/timebridge#license)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Engine-neutral temporal vocabulary for Rust library boundaries: `Timestamp`, `ExactDuration`, and the `WallClock` / `MonotonicClock` capability traits. `no_std`, zero dependencies.

Libraries expose these types instead of choosing Chrono, Jiff or `time` for their users.
Applications convert at their edge with [`timebridge-jiff`](https://crates.io/crates/timebridge-jiff),
[`timebridge-chrono`](https://crates.io/crates/timebridge-chrono) or
[`timebridge-time`](https://crates.io/crates/timebridge-time). The semantics are normative and
specified in [SPEC.md](https://github.com/sunsided/timebridge/blob/main/SPEC.md).

## Install

```sh
cargo add timebridge
```

## Usage

```rust
use timebridge::{ExactDuration, Timestamp, WallClock};

/// A library function that needs the current time, but no datetime engine.
fn is_expired(clock: &dyn WallClock, expires_at: Timestamp) -> bool {
    clock.now() >= expires_at
}

struct Fixed(Timestamp);

impl WallClock for Fixed {
    fn now(&self) -> Timestamp {
        self.0
    }
}

let issued = Timestamp::from_unix_seconds(1_700_000_000);
let expires = issued.checked_add(ExactDuration::from_secs(3_600)).unwrap();

assert!(!is_expired(&Fixed(issued), expires));
assert!(is_expired(&Fixed(expires), expires));
```

## Features

- `std` (default): conversions to and from `std::time::SystemTime`. Without it the crate is
  `no_std` and allocation-free.

## MSRV

Rust 1.88. The MSRV is a signal, not a contract: it may be raised in a minor release.

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/sunsided/timebridge/blob/main/LICENSES/Apache-2.0.txt)
or [MIT license](https://github.com/sunsided/timebridge/blob/main/LICENSES/MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
