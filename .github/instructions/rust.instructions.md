---
applyTo: "**/*.rs"
---

<!-- generated from docs/review/rust.md by scripts/review-index.py; edit the source instead -->

# Rust code

- Every module (`//!`), type, field, variant, function, constant and macro has a doc
  comment, private and `pub(crate)` items included. Docs state intent, invariants, units and
  error/panic conditions (`# Errors`, `# Panics`). They do not restate the name.
- No `mod.rs`: modules with children are `foo.rs` plus `foo/` (see `timestamp.rs` plus
  `timestamp/system_time.rs`).
- No separator comments (`// -----`, `// =====`). If a file needs sections, split it into
  submodules.
- Leave a blank line after the closing `}` of a block statement when another statement
  follows, and between logical phases of a function.
- One level of abstraction per function. If a block needs a "what" comment, extract it into
  a function named after that comment.
- `clippy::pedantic` clean. Suppression only with a scoped
  `#[expect(lint, reason = "...")]`. Never `#[allow]`, and never crate-level suppression.
- Every crate root has `#![forbid(unsafe_code)]`. `timebridge` and the three adapter crates
  are `#![no_std]`; keep them that way, and gate `std` use in `timebridge` behind the `std`
  feature.
- `timebridge` has zero dependencies. Adding one needs a strong reason in the PR.
- Arithmetic on `Timestamp` and `ExactDuration` goes through `i128` total nanoseconds and the
  single range check in `nanos::split`. No `as` casts, no wrapping or saturating arithmetic.
- `WallClock` and `MonotonicClock` stay dyn-compatible where they are today, and keep their
  distinct method names (`now` vs `read`).
- The mermaid diagram in `crates/timebridge/src/lib.rs` must match the crate graph.
