use crate::prefix::KILO;
use typenum::*;

// ===== SI/METRIC UNITS =====
// SI base unit
units! {
    Gram: "g", "gram";
    Kilogram: "kg", "kilogram";
    Tonne: "t", "tonne";
}

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Gram => Kilogram: 1.0 / KILO;      // 1 g = 0.001 kg
    Tonne => Kilogram: KILO;            // 1 t = 1000 kg
}

convert_matrix! {
    Kilogram => Gram, Tonne
}

// Mass quantity definition
use super::{ISQ, SiScale};
quantity!(Mass, ISQ<Z0, P1, Z0, Z0, Z0, Z0, Z0>, SiScale, Kilogram);

// Re-export types for convenience
pub use mass::Mass;
pub use mass::*;
