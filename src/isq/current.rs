use crate::prefix::{MICRO, MILLI};
use typenum::*;

// ===== SI BASE UNIT =====
units! {
    Ampere: "A", "ampere";
}

// ===== METRIC PREFIXES =====
units! {
    Milliampere: "mA", "milliampere";
    Microampere: "μA", "microampere";
}

// ===== CONVERSION RELATIONSHIPS =====

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Milliampere => Ampere: MILLI;      // 1 mA = 0.001 A
    Microampere => Ampere: MICRO;      // 1 μA = 0.000001 A
}

// Current quantity definition
use super::{ISQ, SiScale};
quantity!(Current, ISQ<Z0, Z0, Z0, P1, Z0, Z0, Z0>, SiScale, Ampere);

// Re-export types for convenience
pub use current::Current;
pub use current::*;
