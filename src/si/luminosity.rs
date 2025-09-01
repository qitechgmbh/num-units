use crate::prefix::MILLI;
use typenum::*;

// ===== SI BASE UNIT =====
units! {
    Candela: "cd", "candela";
}

// ===== METRIC PREFIXES =====
units! {
    Millicandela: "mcd", "millicandela";
}

convert_unit! {
    Millicandela: |candela| candela * MILLI;
    Candela: |millicandela| millicandela / MILLI;
}

// Luminosity quantity definition
use super::{SI, SIScale};
quantity!(Luminosity, SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>, SIScale, Candela);

// Re-export types for convenience
pub use luminosity::Luminosity;
pub use luminosity::*;
