use crate::prefix::{CENTI, KILO, MILLI};
use typenum::*;

// SI base unit
units! {
    CubicMeter: "m³", "cubic meter";
}

units! {
    CubicKilometer: "km³", "cubic kilometer";
    CubicCentimeter: "cm³", "cubic centimeter";
    CubicMillimeter: "mm³", "cubic millimeter";
}

// CubicMeter is the SI derived unit for volume
// Using convert_linear! with derived units on the left, base unit on the right

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    CubicKilometer => CubicMeter: KILO * KILO * KILO;          // 1 km³ = 1,000,000,000 m³
    CubicCentimeter => CubicMeter: CENTI * CENTI * CENTI;      // 1 cm³ = 0.000001 m³
    CubicMillimeter => CubicMeter: MILLI * MILLI * MILLI;      // 1 mm³ = 0.000000001 m³
}

// Volume quantity definition
use super::{ISQ, SiScale};
quantity!(Volume, ISQ<P3, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, CubicMeter);

// Re-export types for convenience
pub use volume::Volume;
pub use volume::*;
