//! Conversion between total nanoseconds and the floor-based `(seconds, nanoseconds)` form.
//!
//! Both [`crate::Timestamp`] and [`crate::ExactDuration`] store their value as a floored
//! second count plus a nonnegative sub-second remainder. All arithmetic goes through `i128`
//! total nanoseconds, which can hold every representable value without overflow, so the
//! range check happens in exactly one place: [`split`].

/// Number of nanoseconds in one second.
pub(crate) const PER_SECOND: u32 = 1_000_000_000;

/// Splits a total nanosecond count into floor-based `(seconds, nanoseconds)` parts.
///
/// Returns `None` when the floored second count does not fit in an `i64`. The nanosecond part
/// is always in `0..1_000_000_000`, so `-500_000_000` becomes `(-1, 500_000_000)`.
pub(crate) fn split(total: i128) -> Option<(i64, u32)> {
    let per_second = i128::from(PER_SECOND);
    let seconds = i64::try_from(total.div_euclid(per_second)).ok()?;
    let nanoseconds =
        u32::try_from(total.rem_euclid(per_second)).expect("rem_euclid by 1e9 is in 0..1e9");

    Some((seconds, nanoseconds))
}

/// Joins floor-based `(seconds, nanoseconds)` parts into a total nanosecond count.
///
/// Never overflows: `|seconds| * 1e9 + nanoseconds` is far below `i128::MAX`.
pub(crate) fn join(seconds: i64, nanoseconds: u32) -> i128 {
    i128::from(seconds) * i128::from(PER_SECOND) + i128::from(nanoseconds)
}
