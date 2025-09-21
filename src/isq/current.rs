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

// Metric prefix conversions
convert! {
    Milliampere: |ampere| ampere / MILLI;
    Ampere: |milliampere| milliampere * MILLI;
}

convert! {
    Microampere: |ampere| ampere / MICRO;
    Ampere: |microampere| microampere * MICRO;
}

// Current quantity definition
use super::{ISQ, SiScale};
quantity!(Current, ISQ<Z0, Z0, Z0, P1, Z0, Z0, Z0>, SiScale, Ampere);

// Re-export types for convenience
pub use current::Current;
pub use current::*;
