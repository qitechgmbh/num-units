use typenum::*;

// SI derived unit: watt (kilogram meter squared per second cubed)
base_units! {
    Watt: "watt", "W";
    Kilowatt: "kilowatt", "kW";
    Megawatt: "megawatt", "MW";
    Horsepower: "horsepower", "hp";
}

// ===== CONVERSION RELATIONSHIPS =====

// Kilowatt to Watt
convert_base_unit! {
    Kilowatt: |watt| watt / 1000.0;
    Watt: |kilowatt| kilowatt * 1000.0;
}

// Megawatt to Watt
convert_base_unit! {
    Megawatt: |watt| watt / 1_000_000.0;
    Watt: |megawatt| megawatt * 1_000_000.0;
}

// Horsepower to Watt (mechanical horsepower: 1 hp = 745.7 W)
convert_base_unit! {
    Horsepower: |watt| watt / 745.7;
    Watt: |horsepower| horsepower * 745.7;
}

// Power quantity definition (Mass×Length²/Time³)
use super::{SI, SIScale};
quantity!(Power, SI<P2, P1, N3, Z0, Z0, Z0, Z0>, SIScale);