use crate::prefix::MILLI;
use typenum::*;

// ===== SI BASE UNIT =====
base_units! {
    Candela: "candela", "cd";
}

// ===== METRIC PREFIXES =====
base_units! {
    Millicandela: "millicandela", "mcd";
}

convert_base_unit! {
    Millicandela: |candela| candela * MILLI;
    Candela: |millicandela| millicandela / MILLI;
}

// Luminosity quantity definition
use super::{SI, SIScale};
quantity!(Luminosity, SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>, SIScale);
