---
title: Specification and conformance
applyTo: "SPEC.md,crates/*/src/**,crates/*/tests/**"
summary: SPEC.md is normative; every rule has a conformance test named after its ID, and adapters stay lossless.
---

# Specification and conformance

- `SPEC.md` is normative. A behavior change in `timebridge` or an adapter that contradicts a
  rule needs a SPEC.md change in the same PR, with a version note, or it is a bug.
- Every rule ID (`TIMESTAMP-003`, `CONVERSION-001`, `CLOCK-003`, ...) has at least one test in
  `crates/*/tests/conformance.rs` whose name starts with the lowercased ID
  (`timestamp_003_floor_form`). A new rule comes with its tests.
- Adapter conversions are lossless (`CONVERSION-001`): each new conversion gets a proptest
  round trip over the engine's full range plus fixed vectors for the range edges and for
  `(-1, 500_000_000)`.
- Conversions fail instead of clamping, saturating or wrapping (`TIMESTAMP-004`). Reject any
  `saturating_*`, `unwrap_or(MAX)` or silent rounding in a conversion path.
- Values with no `Timestamp` equivalent (Chrono leap seconds, sub-tick `SystemTime` on
  Windows) are rejected with an error, never collapsed (`CONVERSION-003`).
- No direct conversion from offset- or zone-carrying types (`DateTime<FixedOffset>`,
  `OffsetDateTime`, `Zoned`). Callers convert to UTC first (`CONVERSION-002`).
- `MonotonicClock` readings never decrease (`CLOCK-003`). `ManualClock::set_wall` must not
  touch elapsed time.
