use typenum::*;

// SI derived unit: joule (kilogram meter squared per second squared)
base_units! {
    Joule: "joule", "J";
    Kilojoule: "kilojoule", "kJ";
    Calorie: "calorie", "cal";
    Kilocalorie: "kilocalorie", "kcal";
    WattHour: "watt-hour", "Wh";
    KilowattHour: "kilowatt-hour", "kWh";
}

// ===== CONVERSION RELATIONSHIPS =====

// Kilojoule to Joule
convert_base_unit! {
    Kilojoule: |joule| joule / 1000.0;
    Joule: |kilojoule| kilojoule * 1000.0;
}

// Calorie to Joule (thermochemical calorie: 1 cal = 4.184 J)
convert_base_unit! {
    Calorie: |joule| joule / 4.184;
    Joule: |calorie| calorie * 4.184;
}

// Kilocalorie to Joule
convert_base_unit! {
    Kilocalorie: |joule| joule / 4184.0;
    Joule: |kilocalorie| kilocalorie * 4184.0;
}

// Watt-hour to Joule (1 Wh = 3600 J)
convert_base_unit! {
    WattHour: |joule| joule / 3600.0;
    Joule: |watt_hour| watt_hour * 3600.0;
}

// Kilowatt-hour to Joule
convert_base_unit! {
    KilowattHour: |joule| joule / 3_600_000.0;
    Joule: |kilowatt_hour| kilowatt_hour * 3_600_000.0;
}

// Energy quantity definition (Mass×Length²/Time²)
use super::{SI, SIScale};
quantity!(Energy, SI<P2, P1, N2, Z0, Z0, Z0, Z0>, SIScale);