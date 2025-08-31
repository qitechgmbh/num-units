/// # Length Units - SI Length Measurements
///
/// This module defines SI length units and their conversions. Length is one of the
/// seven base dimensions in the SI system, with the meter as its base unit.
///
/// ## Base Unit
///
/// - **Meter (m)**: The SI base unit of length
///
/// ## Derived Units
///
/// Common length units include:
/// - **Kilometer (km)**: 1000 meters
/// - **Centimeter (cm)**: 0.01 meters
/// - **Millimeter (mm)**: 0.001 meters
/// - **Micrometer (μm)**: 0.000001 meters
/// - **Nanometer (nm)**: 0.000000001 meters
///
/// ## Usage
///
/// ```rust
/// use num_units::si::length;
///
/// // Create length quantities
/// let distance = length::f64::Length::from_meter(100.0);
/// let height = length::f64::Length::from_kilometer(1.5);
///
/// // Convert between units
/// let distance_km = distance.as_kilometer();     // 0.1 km
/// let height_m = height.as_meter();              // 1500.0 m
///
/// // Arithmetic operations
/// let total_distance = distance + height;        // Automatic conversion
/// let area = distance * height;                  // Creates area quantity
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All length operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
use crate::prefix::{CENTI, KILO, MICRO, MILLI, NANO};
use typenum::*;

// SI base unit
units! {
    Meter: "m", "meter";
}

units! {
    Kilometer: "km", "kilometer";
    Centimeter: "cm", "centimeter";
    Millimeter: "mm", "millimeter";
    Micrometer: "μm", "micrometer";
    Nanometer: "nm", "nanometer";
}

convert_unit! {
    Kilometer: |meter| meter / KILO;
    Meter: |kilometer| kilometer * KILO;
}

convert_unit! {
    Centimeter: |meter| meter / CENTI;
    Meter: |centimeter| centimeter * CENTI;
}

convert_unit! {
    Millimeter: |meter| meter / MILLI;
    Meter: |millimeter| millimeter * MILLI;
}

convert_unit! {
    Micrometer: |meter| meter / MICRO;
    Meter: |micrometer| micrometer * MICRO;
}

convert_unit! {
    Nanometer: |meter| meter / NANO;
    Meter: |nanometer| nanometer * NANO;
}

// Length quantity definition
use super::{SI, SIScale};
quantity!(Length, SI<P1, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale);
