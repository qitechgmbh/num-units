use typenum::*;

// SI derived unit: joule (kilogram meter squared per second squared)
units! {
    Joule: "J", "joule";
    Kilojoule: "kJ", "kilojoule";
    Calorie: "cal", "calorie";
    Kilocalorie: "kcal", "kilocalorie";
    WattHour: "Wh", "watt-hour";
    KilowattHour: "kWh", "kilowatt-hour";
}

// ===== CONVERSION RELATIONSHIPS =====

// Joule is the SI derived unit for energy
// Using convert_linear! with derived units on the left, base unit on the right

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Kilojoule => Joule: KILO;              // 1 kJ = 1000 J
    Calorie => Joule: 4.184;                // 1 cal = 4.184 J
    Kilocalorie => Joule: KILO * 4.184;     // 1 kcal = 4184 J
    WattHour => Joule: HOUR;                // 1 Wh = 3600 J (60 min * 60 sec)
    KilowattHour => Joule: KILO * HOUR;     // 1 kWh = 3,600,000 J
}

convert_matrix! {
    Joule => Kilojoule, Calorie, Kilocalorie, WattHour, KilowattHour
}

use crate::prefix::{HOUR, KILO};

// Energy quantity definition (Mass×Length²/Time²)
use super::{ISQ, SiScale};
quantity!(Energy, ISQ<P2, P1, N2, Z0, Z0, Z0, Z0>, SiScale, Joule);

// Re-export types for convenience
pub use energy::Energy;
pub use energy::*;
