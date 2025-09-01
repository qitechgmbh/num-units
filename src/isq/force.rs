use typenum::*;

// SI derived unit: newton (kilogram meter per second squared)
units! {
    Newton: "N", "newton";
    Kilonewton: "kN", "kilonewton";
    Pound: "lbf", "pound-force";
}

// ===== CONVERSION RELATIONSHIPS =====

// Kilonewton to Newton
convert_unit! {
    Kilonewton: |newton| newton / KILO;
    Newton: |kilonewton| kilonewton * KILO;
}

// Pound-force to Newton (1 lbf = 4.448222 N)
convert_unit! {
    Pound: |newton| newton / 4.448222;
    Newton: |pound| pound * 4.448222;
}

use crate::prefix::KILO;

// Force quantity definition (Mass×Length/Time²)
use super::{ISQ, SiScale};
quantity!(Force, ISQ<P1, P1, N2, Z0, Z0, Z0, Z0>, SiScale, Newton);

// Re-export types for convenience
pub use force::Force;
pub use force::*;
