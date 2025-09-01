use typenum::*;

// SI derived unit: watt (kilogram meter squared per second cubed)
units! {
    Watt: "W", "watt";
    Kilowatt: "kW", "kilowatt";
    Megawatt: "MW", "megawatt";
    Horsepower: "hp", "horsepower";
}

// ===== CONVERSION RELATIONSHIPS =====

// Kilowatt to Watt
convert_unit! {
    Kilowatt: |watt| watt / KILO;
    Watt: |kilowatt| kilowatt * KILO;
}

// Megawatt to Watt
convert_unit! {
    Megawatt: |watt| watt / MEGA;
    Watt: |megawatt| megawatt * MEGA;
}

// Horsepower to Watt (mechanical horsepower: 1 hp = 745.7 W)
convert_unit! {
    Horsepower: |watt| watt / 745.7;
    Watt: |horsepower| horsepower * 745.7;
}

use crate::prefix::{KILO, MEGA};

// Power quantity definition (Mass×Length²/Time³)
use super::{ISQ, SiScale};
quantity!(Power, ISQ<P2, P1, N3, Z0, Z0, Z0, Z0>, SiScale, Watt);

// Re-export types for convenience
pub use power::Power;
pub use power::*;
