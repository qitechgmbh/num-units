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

// Unit conversions using convert_linear!
crate::convert_linear! {
    Millicandela => Candela: MILLI;    // 1 mcd = 0.001 cd
}

// Luminosity quantity definition
use super::{ISQ, SiScale};
quantity!(Luminosity, ISQ<Z0, Z0, Z0, Z0, Z0, Z0, P1>, SiScale, Candela);

// Re-export types for convenience
pub use luminosity::Luminosity;
pub use luminosity::*;
