use crate::prefix::KILO;
use typenum::*;

// ===== SI/METRIC UNITS =====
// SI base unit
units! {
    Gram: "g", "gram";
    Kilogram: "kg", "kilogram";
    Tonne: "t", "tonne";
}

convert_unit! {
    Gram: |kilogram| kilogram * KILO;
    Kilogram: |gram| gram / KILO;
}
convert_unit! {
    Tonne: |kilogram| kilogram / KILO;
    Kilogram: |tonne| tonne * KILO;
}

convert_matrix! {
    Kilogram => Gram, Tonne
}

// Mass quantity definition
use super::{SI, SIScale};
quantity!(Mass, SI<Z0, P1, Z0, Z0, Z0, Z0, Z0>, SIScale, Kilogram);

// Re-export types for convenience
pub use mass::Mass;
pub use mass::*;
