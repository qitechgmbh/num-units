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

convert! {
    Millisecond: |second| second / MILLI;
    Second: |millisecond| millisecond * MILLI;
}

convert! {
    Microsecond: |second| second / MICRO;
    Second: |microsecond| microsecond * MICRO;
}

convert! {
    Nanosecond: |second| second / NANO;
    Second: |nanosecond| nanosecond * NANO;
}

convert! {
    Picosecond: |second| second / PICO;
    Second: |picosecond| picosecond * PICO;
}

convert! {
    Kilosecond: |second| second / KILO;
    Second: |kilosecond| kilosecond * KILO;
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
