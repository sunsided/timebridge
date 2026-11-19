# timebridge interchange specification (draft 0.1)

This document is normative for `timebridge` and every adapter crate. The key words MUST,
MUST NOT and MAY are used as in RFC 2119. Conformance tests are named after the rule IDs
(`timestamp_003_floor_form`, `conversion_003_leap_second_rejected`, ...).

## Timestamp

- **TIMESTAMP-001** The epoch is `1970-01-01T00:00:00Z`.
- **TIMESTAMP-002** Resolution is one nanosecond.
- **TIMESTAMP-003** Values use floor-based decomposition: `seconds` is the floor of the signed
  Unix time, `nanoseconds` is in `0..1_000_000_000`. Half a second before the epoch is
  `(-1, 500_000_000)`. Ordering of values is chronological.
- **TIMESTAMP-004** Arithmetic and conversions MUST reject values outside the destination's
  representable range. They MUST NOT clamp, saturate or wrap.
- **TIMESTAMP-005** A `Timestamp` represents Unix time only. It carries no offset, zone,
  calendar or leap-second information and MUST NOT be used for TAI or other time scales.

The representable range is `i64::MIN` seconds to `i64::MAX` seconds plus 999,999,999 ns.

## ExactDuration

- **DURATION-001** An `ExactDuration` is a fixed number of nanoseconds, independent of any
  calendar or zone. Calendar-relative quantities ("one day", "one month") are not
  `ExactDuration`s.
- **DURATION-002** Values use the same floor-based decomposition and range as `Timestamp`.
  Minus half a second is `(-1, 500_000_000)`.

## Conversions

- **CONVERSION-001** A conversion offered by an adapter is lossless: converting there and back
  yields the original value whenever the first conversion succeeds.
- **CONVERSION-002** Lossy operations require explicit caller intent. Adapters MUST NOT offer
  direct conversions from types carrying an offset or zone (`chrono::DateTime<FixedOffset>`,
  `time::OffsetDateTime`, `jiff::Zoned`); the caller first converts to the engine's UTC or
  timestamp type.
- **CONVERSION-003** Source values with no `Timestamp` equivalent, such as Chrono leap seconds,
  MUST be rejected with a distinct error, never collapsed into a neighbouring value.

## Clocks

- **CLOCK-001** Wall time (`WallClock`) and elapsed time (`MonotonicClock`) are separate
  capabilities. Timeouts and elapsed-time measurement MUST use `MonotonicClock`.
- **CLOCK-002** `WallClock` readings MAY jump forwards or backwards between calls.
- **CLOCK-003** Successive `MonotonicClock` readings from one clock MUST NOT decrease. Readings
  from different clock instances are not comparable.

## Reserved

- **ZONE-001** A numeric UTC offset MUST NOT be treated as an IANA zone identifier. Reserved for
  the civil-time and zone-rules layer, which is not part of this draft.
