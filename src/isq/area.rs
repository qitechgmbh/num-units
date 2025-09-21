/// # Area Units - SI Area Measurements
///
/// This module defines SI area units and their conversions. Area is a derived
/// dimension in the SI system, with the square meter as its base unit.
///
/// ## Base Unit
///
/// - **Square Meter (m²)**: The SI base unit of area
///
/// ## Derived Units
///
/// Common area units include:
/// - **Square Kilometer (km²)**: 1,000,000 square meters
/// - **Square Centimeter (cm²)**: 0.0001 square meters
/// - **Square Millimeter (mm²)**: 0.000001 square meters
/// - **Square Micrometer (μm²)**: 0.000000000001 square meters
/// - **Square Nanometer (nm²)**: 0.000000000000000001 square meters
///
/// ## Usage
///
/// ```rust
/// use num_units::area::Area;
/// use num_units::area::{SquareMeter, SquareKilometer};
///
/// // Create area quantities
/// let surface = Area::from::<SquareMeter>(100.0);
/// let field = Area::from::<SquareKilometer>(1.5);
///
/// // Convert between units
/// let surface_km2 = surface.to::<SquareKilometer>();   // 0.0001 km²
/// let field_m2 = field.to::<SquareMeter>();            // 1,500,000.0 m²
///
/// // Arithmetic operations
/// let total_area = surface + field;                  // Automatic conversion
/// // let volume = surface * height;                     // Creates volume quantity
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All area operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
use crate::prefix::{CENTI, KILO, MICRO, MILLI, NANO};
use typenum::*;

// SI base unit
units! {
    SquareMeter: "m²", "square meter";
}

units! {
    SquareKilometer: "km²", "square kilometer";
    SquareCentimeter: "cm²", "square centimeter";
    SquareMillimeter: "mm²", "square millimeter";
    SquareMicrometer: "μm²", "square micrometer";
    SquareNanometer: "nm²", "square nanometer";
}

convert! {
    SquareKilometer: |square_meter| square_meter / (KILO * KILO);
    SquareMeter: |square_kilometer| square_kilometer * (KILO * KILO);
}

convert! {
    SquareCentimeter: |square_meter| square_meter / (CENTI * CENTI);
    SquareMeter: |square_centimeter| square_centimeter * (CENTI * CENTI);
}

convert! {
    SquareMillimeter: |square_meter| square_meter / (MILLI * MILLI);
    SquareMeter: |square_millimeter| square_millimeter * (MILLI * MILLI);
}

convert! {
    SquareMicrometer: |square_meter| square_meter / (MICRO * MICRO);
    SquareMeter: |square_micrometer| square_micrometer * (MICRO * MICRO);
}

convert! {
    SquareNanometer: |square_meter| square_meter / (NANO * NANO);
    SquareMeter: |square_nanometer| square_nanometer * (NANO * NANO);
}

crate::convert_matrix! {
    SquareMeter => SquareKilometer, SquareCentimeter, SquareMillimeter, SquareMicrometer, SquareNanometer
}

// Area quantity definition
use super::{ISQ, SiScale};
quantity!(Area, ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, SquareMeter);

// Re-export types for convenience
pub use area::Area;
pub use area::*;
