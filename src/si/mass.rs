use crate::prefix::KILO;
use typenum::*;

// ===== SI/METRIC UNITS =====
// SI base unit
base_units! {
    Gram: "gram", "g";
    Kilogram: "kilogram", "kg";
    Tonne: "tonne", "t";
}

convert_base_unit! {
    Kilogram: |gram| gram * KILO;
    Gram: |kilogram| kilogram / KILO;
}
convert_base_unit! {
    Tonne: |gram| gram / 1_000_000.0;
    Gram: |tonne| tonne * 1_000_000.0;
}

// Mass quantity definition
use super::{SI, SIScale};
quantity!(Mass, SI<Z0, P1, Z0, Z0, Z0, Z0, Z0>, SIScale);
