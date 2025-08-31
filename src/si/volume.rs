use crate::prefix::{CENTI, KILO, MILLI};
use typenum::*;

// SI base unit
base_units! {
    CubicMeter: "cubicmeter", "m³";
}

base_units! {
    CubicKilometer: "cubickilometer", "km³";
    CubicCentimeter: "cubiccentimeter", "cm³";
    CubicMillimeter: "cubicmillimeter", "mm³";
}

convert_base_unit! {
    CubicKilometer: |cubicmeter| cubicmeter / (KILO * KILO * KILO);
    CubicMeter: |cubickilometer| cubickilometer * (KILO * KILO * KILO);
}

convert_base_unit! {
    CubicCentimeter: |cubicmeter| cubicmeter / (CENTI * CENTI * CENTI);
    CubicMeter: |cubiccentimeter| cubiccentimeter * (CENTI * CENTI * CENTI);
}

convert_base_unit! {
    CubicMillimeter: |cubicmeter| cubicmeter / (MILLI * MILLI * MILLI);
    CubicMeter: |cubicmillimeter| cubicmillimeter * (MILLI * MILLI * MILLI);
}

// Volume quantity definition
use super::{SI, SIScale};
quantity!(Volume, SI<P3, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale);
