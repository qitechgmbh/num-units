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
/// use num_units::length::{Length, Meter, Kilometer};
///
/// // Create length quantities
/// let distance = Length::from::<Meter>(100.0);
/// let height = Length::from::<Kilometer>(1.5);
///
/// // Convert between units
/// let distance_km = distance.to::<Kilometer>();     // 0.1 km
/// let height_m = height.to::<Meter>();              // 1500.0 m
///
/// // Arithmetic operations
/// let total_distance = distance + height;        // Automatic conversion
/// // let area = distance * height;                  // Creates area quantity
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All length operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
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

convert_unit_int! {
    Kilometer: 1;        // 1 meter = 1/1000 km, so 1 km = 1000 base units
    Meter: 1000;         // 1000 meters = 1000 base units (1000:1 ratio)
}

convert_unit_int! {
    Centimeter: 100;     // 1 meter = 100 cm
    Meter: 1;            // 1 meter = 1 meter (base unit)
}

convert_unit_int! {
    Millimeter: 1000;    // 1 meter = 1000 mm
    Meter: 1;            // 1 meter = 1 meter (base unit)
}

convert_unit_int! {
    Micrometer: 1000000; // 1 meter = 1,000,000 μm
    Meter: 1;            // 1 meter = 1 meter (base unit)
}

convert_unit_int! {
    Nanometer: 1000000000; // 1 meter = 1,000,000,000 nm
    Meter: 1;              // 1 meter = 1 meter (base unit)
}

convert_matrix! {
    Meter => Kilometer, Centimeter, Millimeter, Micrometer, Nanometer
}

// Length quantity definition
use super::{SI, SIScale};
quantity!(Length, SI<P1, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale, Meter);

// Re-export types for convenience
pub use length::Length;
pub use length::*;
