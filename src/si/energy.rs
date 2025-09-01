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

// Kilojoule to Joule
convert_unit! {
    Kilojoule: |joule| joule / KILO;
    Joule: |kilojoule| kilojoule * KILO;
}

// Calorie to Joule (thermochemical calorie: 1 cal = 4.184 J)
convert_unit! {
    Calorie: |joule| joule / 4.184;
    Joule: |calorie| calorie * 4.184;
}

// Kilocalorie to Joule
convert_unit! {
    Kilocalorie: |joule| joule / 4184.0;
    Joule: |kilocalorie| kilocalorie * 4184.0;
}

// Watt-hour to Joule (1 Wh = 3600 J)
convert_unit! {
    WattHour: |joule| joule / 3600.0;
    Joule: |watt_hour| watt_hour * 3600.0;
}

// Kilowatt-hour to Joule
convert_unit! {
    KilowattHour: |joule| joule / 3_600_000.0;
    Joule: |kilowatt_hour| kilowatt_hour * 3_600_000.0;
}

convert_matrix! {
    Joule => Kilojoule, Calorie, Kilocalorie, WattHour, KilowattHour
}

use crate::prefix::KILO;

// Energy quantity definition (Mass×Length²/Time²)
use super::{SI, SIScale};
quantity!(Energy, SI<P2, P1, N2, Z0, Z0, Z0, Z0>, SIScale, Joule);

// Re-export types for convenience
pub use energy::Energy;
pub use energy::*;
