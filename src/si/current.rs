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
convert_unit! {
    Milliampere: |ampere| ampere / MILLI;
    Ampere: |milliampere| milliampere * MILLI;
}

convert_unit! {
    Microampere: |ampere| ampere / MICRO;
    Ampere: |microampere| microampere * MICRO;
}

// Current quantity definition
use super::{SI, SIScale};
quantity!(Current, SI<Z0, Z0, Z0, P1, Z0, Z0, Z0>, SIScale, Ampere);

// Re-export types for convenience
pub use current::Current;
pub use current::*;
