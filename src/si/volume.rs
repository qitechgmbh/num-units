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

convert_unit! {
    CubicKilometer: |cubicmeter| cubicmeter / (KILO * KILO * KILO);
    CubicMeter: |cubickilometer| cubickilometer * (KILO * KILO * KILO);
}

convert_unit! {
    CubicCentimeter: |cubicmeter| cubicmeter / (CENTI * CENTI * CENTI);
    CubicMeter: |cubiccentimeter| cubiccentimeter * (CENTI * CENTI * CENTI);
}

convert_unit! {
    CubicMillimeter: |cubicmeter| cubicmeter / (MILLI * MILLI * MILLI);
    CubicMeter: |cubicmillimeter| cubicmillimeter * (MILLI * MILLI * MILLI);
}

// Volume quantity definition
use super::{SI, SIScale};
quantity!(Volume, SI<P3, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale, CubicMeter);
