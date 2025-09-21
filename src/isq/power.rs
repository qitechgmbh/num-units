use typenum::*;

// SI derived unit: watt (kilogram meter squared per second cubed)
units! {
    Watt: "W", "watt";
    Kilowatt: "kW", "kilowatt";
    Megawatt: "MW", "megawatt";
    Horsepower: "hp", "horsepower";
}

// ===== CONVERSION RELATIONSHIPS =====

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Kilowatt => Watt: KILO;            // 1 kW = 1000 W
    Megawatt => Watt: MEGA;            // 1 MW = 1,000,000 W
    Horsepower => Watt: 745.7;         // 1 hp = 745.7 W (mechanical)
}

use crate::prefix::{KILO, MEGA};

// Power quantity definition (Mass×Length²/Time³)
use super::{ISQ, SiScale};
quantity!(Power, ISQ<P2, P1, N3, Z0, Z0, Z0, Z0>, SiScale, Watt);

// Re-export types for convenience
pub use power::Power;
pub use power::*;
