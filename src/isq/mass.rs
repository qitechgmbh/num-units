use crate::prefix::KILO;
use typenum::*;

// ===== SI/METRIC UNITS =====
// SI base unit
units! {
    Gram: "g", "gram";
    Kilogram: "kg", "kilogram";
    Tonne: "t", "tonne";
}

convert! {
    Gram: |kilogram| kilogram * KILO;
    Kilogram: |gram| gram / KILO;
}
convert! {
    Tonne: |kilogram| kilogram / KILO;
    Kilogram: |tonne| tonne * KILO;
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
