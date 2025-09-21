use crate::prefix::{KILO, MICRO, MILLI, NANO, PICO};
use typenum::*;

// ===== SI BASE UNIT =====
units! {
    Second: "s", "second";
}

// ===== METRIC PREFIXES =====
units! {
    Millisecond: "ms", "millisecond";
    Microsecond: "μs", "microsecond";
    Nanosecond: "ns", "nanosecond";
    Picosecond: "ps", "picosecond";
    Kilosecond: "ks", "kilosecond";
}

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Millisecond => Second: MILLI;      // 1 ms = 0.001 s
    Microsecond => Second: MICRO;      // 1 μs = 0.000001 s
    Nanosecond => Second: NANO;        // 1 ns = 0.000000001 s
    Picosecond => Second: PICO;        // 1 ps = 0.000000000001 s
    Kilosecond => Second: KILO;        // 1 ks = 1000 s
}

crate::convert_matrix! {
    Second => Millisecond, Microsecond, Nanosecond, Picosecond, Kilosecond
}

// Time quantity definition
use super::{ISQ, SiScale};
quantity!(Time, ISQ<Z0, Z0, P1, Z0, Z0, Z0, Z0>, SiScale, Second);

// Re-export types for convenience
pub use time::Time;
pub use time::*;
