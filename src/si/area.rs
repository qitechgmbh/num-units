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
/// - **Hectare (ha)**: 10,000 square meters
/// - **Acre (ac)**: 4,046.873 square meters
/// - **Square Foot (ft²)**: 0.09290304 square meters
/// - **Square Inch (in²)**: 0.00064516 square meters
/// - **Square Yard (yd²)**: 0.8361274 square meters
/// - **Square Mile (mi²)**: 2,589,988 square meters
///
/// ## Usage
///
/// ```rust,ignore
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
use crate::prefix::{
    ATTO, CENTI, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI, NANO, PETA, PICO,
    TERA, YOCTO, YOTTA, ZEPTO, ZETTA,
};
use typenum::*;

// SI base unit
units! {
    SquareMeter: "m²", "square meter";
}

// SI prefixed units
units! {
    SquareYottameter: "Ym²", "square yottameter";
    SquareZettameter: "Zm²", "square zettameter";
    SquareExameter: "Em²", "square exameter";
    SquarePetameter: "Pm²", "square petameter";
    SquareTerameter: "Tm²", "square terameter";
    SquareGigameter: "Gm²", "square gigameter";
    SquareMegameter: "Mm²", "square megameter";
    SquareKilometer: "km²", "square kilometer";
    SquareHectometer: "hm²", "square hectometer";
    SquareDecameter: "dam²", "square decameter";
    SquareDecimeter: "dm²", "square decimeter";
    SquareCentimeter: "cm²", "square centimeter";
    SquareMillimeter: "mm²", "square millimeter";
    SquareMicrometer: "μm²", "square micrometer";
    SquareNanometer: "nm²", "square nanometer";
    SquarePicometer: "pm²", "square picometer";
    SquareFemtometer: "fm²", "square femtometer";
    SquareAttometer: "am²", "square attometer";
    SquareZeptometer: "zm²", "square zeptometer";
    SquareYoctometer: "ym²", "square yoctometer";
}

// Imperial/US customary units
units! {
    Acre: "ac", "acre";
    Are: "a", "are";
    Barn: "b", "barn";
    CircularMil: "cmil", "circular mil";
    Hectare: "ha", "hectare";
    SquareFoot: "ft²", "square foot";
    SquareInch: "in²", "square inch";
    SquareMile: "mi²", "square mile";
    SquareYard: "yd²", "square yard";
}

// Unit conversions using convert_linear! with exact UOM coefficients
crate::convert_linear! {
    // SI prefixed units - using squared prefixes
    SquareYottameter => SquareMeter: YOTTA * YOTTA;
    SquareZettameter => SquareMeter: ZETTA * ZETTA;
    SquareExameter => SquareMeter: EXA * EXA;
    SquarePetameter => SquareMeter: PETA * PETA;
    SquareTerameter => SquareMeter: TERA * TERA;
    SquareGigameter => SquareMeter: GIGA * GIGA;
    SquareMegameter => SquareMeter: MEGA * MEGA;
    SquareKilometer => SquareMeter: KILO * KILO;
    SquareHectometer => SquareMeter: HECTO * HECTO;
    SquareDecameter => SquareMeter: DECA * DECA;
    SquareDecimeter => SquareMeter: DECI * DECI;
    SquareCentimeter => SquareMeter: CENTI * CENTI;
    SquareMillimeter => SquareMeter: MILLI * MILLI;
    SquareMicrometer => SquareMeter: MICRO * MICRO;
    SquareNanometer => SquareMeter: NANO * NANO;
    SquarePicometer => SquareMeter: PICO * PICO;
    SquareFemtometer => SquareMeter: FEMTO * FEMTO;
    SquareAttometer => SquareMeter: ATTO * ATTO;
    SquareZeptometer => SquareMeter: ZEPTO * ZEPTO;
    SquareYoctometer => SquareMeter: YOCTO * YOCTO;

    // Imperial/US customary units - exact UOM coefficients
    Acre => SquareMeter: 4.046_873_E3;
    Are => SquareMeter: 1.0_E2;
    Barn => SquareMeter: 1.0_E-28;
    CircularMil => SquareMeter: 5.067_075_E-10;
    Hectare => SquareMeter: 1.0_E4;
    SquareFoot => SquareMeter: 9.290_304_E-2;
    SquareInch => SquareMeter: 6.451_6_E-4;
    SquareMile => SquareMeter: 2.589_988_E6;
    SquareYard => SquareMeter: 8.361_274_E-1;
}

crate::convert_matrix! {
    SquareMeter => SquareYottameter, SquareZettameter, SquareExameter, SquarePetameter, SquareTerameter, SquareGigameter, SquareMegameter, SquareKilometer, SquareHectometer, SquareDecameter, SquareDecimeter, SquareCentimeter, SquareMillimeter, SquareMicrometer, SquareNanometer, SquarePicometer, SquareFemtometer, SquareAttometer, SquareZeptometer, SquareYoctometer, Acre, Are, Barn, CircularMil, Hectare, SquareFoot, SquareInch, SquareMile, SquareYard
}

// Area quantity definition
use super::{ISQ, SiScale};
quantity!(Area, ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, SquareMeter);

// UOM compatibility tests
#[cfg(test)]
mod tests {

    macro_rules! test_uom_area {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::area,
                uom::si::area,
                Area,
                Area,
                SquareMeter,
                $num_units_unit,
                square_meter,
                $uom_unit
            );
        };
    }

    // Test SI prefixed units
    test_uom_area!(SquareYottameter, square_yottameter);
    test_uom_area!(SquareZettameter, square_zettameter);
    test_uom_area!(SquareExameter, square_exameter);
    test_uom_area!(SquarePetameter, square_petameter);
    test_uom_area!(SquareTerameter, square_terameter);
    test_uom_area!(SquareGigameter, square_gigameter);
    test_uom_area!(SquareMegameter, square_megameter);
    test_uom_area!(SquareKilometer, square_kilometer);
    test_uom_area!(SquareHectometer, square_hectometer);
    test_uom_area!(SquareDecameter, square_decameter);
    test_uom_area!(SquareMeter, square_meter);
    test_uom_area!(SquareDecimeter, square_decimeter);
    test_uom_area!(SquareCentimeter, square_centimeter);
    test_uom_area!(SquareMillimeter, square_millimeter);
    test_uom_area!(SquareMicrometer, square_micrometer);
    test_uom_area!(SquareNanometer, square_nanometer);
    test_uom_area!(SquarePicometer, square_picometer);
    test_uom_area!(SquareFemtometer, square_femtometer);
    test_uom_area!(SquareAttometer, square_attometer);
    test_uom_area!(SquareZeptometer, square_zeptometer);
    test_uom_area!(SquareYoctometer, square_yoctometer);

    // Test Imperial/US customary units
    test_uom_area!(Acre, acre);
    test_uom_area!(Are, are);
    test_uom_area!(Barn, barn);
    test_uom_area!(CircularMil, circular_mil);
    test_uom_area!(Hectare, hectare);
    test_uom_area!(SquareFoot, square_foot);
    test_uom_area!(SquareInch, square_inch);
    test_uom_area!(SquareMile, square_mile);
    test_uom_area!(SquareYard, square_yard);
}

// Re-export types for convenience
pub use area::Area;
pub use area::*;
