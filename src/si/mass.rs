use crate::prefix::{KILO, MEGA};
use typenum::*;

// ===== SI/METRIC UNITS =====
// SI base unit
units! {
    Gram: "g", "gram";
    Kilogram: "kg", "kilogram";
    Tonne: "t", "tonne";
}

convert_unit! {
    Kilogram: |gram| gram * KILO;
    Gram: |kilogram| kilogram / KILO;
}
convert_unit! {
    Tonne: |gram| gram / MEGA;
    Gram: |tonne| tonne * MEGA;
}

// Mass quantity definition
use super::{SI, SIScale};
quantity!(Mass, SI<Z0, P1, Z0, Z0, Z0, Z0, Z0>, SIScale);
