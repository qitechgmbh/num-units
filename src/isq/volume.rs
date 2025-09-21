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

convert! {
    CubicKilometer: |cubicmeter| cubicmeter / (KILO * KILO * KILO);
    CubicMeter: |cubickilometer| cubickilometer * (KILO * KILO * KILO);
}

convert! {
    CubicCentimeter: |cubicmeter| cubicmeter / (CENTI * CENTI * CENTI);
    CubicMeter: |cubiccentimeter| cubiccentimeter * (CENTI * CENTI * CENTI);
}

convert! {
    CubicMillimeter: |cubicmeter| cubicmeter / (MILLI * MILLI * MILLI);
    CubicMeter: |cubicmillimeter| cubicmillimeter * (MILLI * MILLI * MILLI);
}

// Volume quantity definition
use super::{ISQ, SiScale};
quantity!(Volume, ISQ<P3, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, CubicMeter);

// Re-export types for convenience
pub use volume::Volume;
pub use volume::*;
