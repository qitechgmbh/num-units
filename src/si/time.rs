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

convert_unit! {
    Millisecond: |second| second / MILLI;
    Second: |millisecond| millisecond * MILLI;
}

convert_unit! {
    Microsecond: |second| second / MICRO;
    Second: |microsecond| microsecond * MICRO;
}

convert_unit! {
    Nanosecond: |second| second / NANO;
    Second: |nanosecond| nanosecond * NANO;
}

convert_unit! {
    Picosecond: |second| second / PICO;
    Second: |picosecond| picosecond * PICO;
}

convert_unit! {
    Kilosecond: |second| second / KILO;
    Second: |kilosecond| kilosecond * KILO;
}

crate::convert_matrix! {
    Second => Millisecond, Microsecond, Nanosecond, Picosecond, Kilosecond
}

// Time quantity definition
use super::{SI, SIScale};
quantity!(Time, SI<Z0, Z0, P1, Z0, Z0, Z0, Z0>, SIScale, Second);

// Re-export types for convenience
pub use time::Time;
pub use time::*;
