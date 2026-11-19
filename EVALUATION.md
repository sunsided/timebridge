# Phase 4 evaluation: neutral core vs. Jiff directly

Two implementations of the same token-expiry library:

- `examples/token-expiry`: public API uses `timebridge::{Timestamp, WallClock}`.
- `examples/token-expiry-jiff`: public API uses `jiff::{Timestamp, SignedDuration}`.

Measured on 2026-11-19, rustc 1.100.0, Jiff 1.0.1 with `default-features = false`.

| Criterion                      | Neutral core                               | Jiff directly                         |
| ------------------------------ | ------------------------------------------ | ------------------------------------- |
| Normal dependencies            | `timebridge` (0 transitive deps)        | `jiff`, `jiff-core`                   |
| Clean debug build of the lib   | 0.2 s                                      | 1.9 s                                 |
| Clock injection                | `&dyn WallClock`, shared trait             | none; caller passes `now`             |
| Consumer on Jiff               | `.to_bridge()` / `.to_jiff()` at the edge | no conversion                         |
| Consumer on Chrono or `time`   | `.to_bridge()` / `.to_chrono()` etc.     | adds Jiff; converts via third-party crates (`jiff-chrono-conversions`, `timetraveler`) or by hand |

## Findings

- **Dependency weight is a weak argument.** Jiff without default features is two crates and
  about two seconds of clean build. That cost vanishes for applications that already use Jiff.
- **Conversion friction is real but small.** Each consumer writes one conversion per value at
  the boundary, plus a five-line `WallClock` impl. Range errors surface as `Result`s that are
  unreachable in practice for wall-clock values.
- **The clock trait is the distinctive part.** Jiff 1.0 only offers the global `Timestamp::now()`
  and `Zoned::now()`, with no injectable clock abstraction, so
  the Jiff version moves `now` into every call signature. A shared `WallClock` /
  `MonotonicClock` contract is useful whichever value type wins.
- **The value types are only clearly better for engine-neutral libraries** whose users are split
  across Chrono, `time` and Jiff. For a Jiff-only ecosystem, exposing `jiff::Timestamp` is
  simpler.
- **`MonotonicClock::read` vs `WallClock::now`.** Both methods were originally `now()`, which made
  calls ambiguous on types implementing both traits (`SystemClock`, `ManualClock`). The
  monotonic method was renamed to `read`.

## Not yet measured

Compile times inside a real application graph, and adoption cost for an existing library
(for example migrating a JWT crate). Civil time and zone rules are out of scope for this
iteration.
