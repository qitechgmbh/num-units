use typenum::*;

// SI derived unit: newton (kilogram meter per second squared)
units! {
    Newton: "N", "newton";
    Kilonewton: "kN", "kilonewton";
    Pound: "lbf", "pound-force";
}

// ===== CONVERSION RELATIONSHIPS =====

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Kilonewton => Newton: KILO;        // 1 kN = 1000 N
    Pound => Newton: 4.448222;          // 1 lbf = 4.448222 N
}

use crate::prefix::KILO;

// Force quantity definition (Mass×Length/Time²)
use super::{ISQ, SiScale};
quantity!(Force, ISQ<P1, P1, N2, Z0, Z0, Z0, Z0>, SiScale, Newton);

// Re-export types for convenience
pub use force::Force;
pub use force::*;
