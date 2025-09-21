use crate::prefix::*;
/// # Volume Units - Volume Measurements
///
/// This module defines volume units and their conversions. Volume is measured in
/// cubic meters as the SI derived unit, with various prefixed units and
/// conventional volume units.
///
/// ## Units
///
/// ### SI Derived Unit
/// - **Cubic meter (m³)**: The SI unit of volume
///
/// ### SI Prefixed Cubic Units
/// - All SI prefixes applied to cubic meters
///
/// ### SI Prefixed Liter Units
/// - All SI prefixes applied to liters (1 L = 0.001 m³)
///
/// ### Conventional Volume Units
/// - **Liter (L)**: Common metric volume unit
/// - **Gallon (gal)**: US customary volume unit
/// - **Quart/Cup/Pint**: Fractional gallon units
/// - **Tablespoon/Tablespoon**: Small cooking volume units
/// - **Cubic foot/inch/yard**: Imperial volume units
use typenum::*;

// ===== SI DERIVED UNIT =====
units! {
    // SI prefixed cubic meters (subset of most commonly used)
    CubicTerameter: "Tm³", "cubic terameter";
    CubicGigameter: "Gm³", "cubic gigameter";
    CubicMegameter: "Mm³", "cubic megameter";
    CubicKilometer: "km³", "cubic kilometer";
    CubicHectometer: "hm³", "cubic hectometer";
    CubicDecameter: "dam³", "cubic decameter";
    CubicMeter: "m³", "cubic meter";
    CubicDecimeter: "dm³", "cubic decimeter";
    CubicCentimeter: "cm³", "cubic centimeter";
    CubicMillimeter: "mm³", "cubic millimeter";
    CubicMicrometer: "µm³", "cubic micrometer";
    CubicNanometer: "nm³", "cubic nanometer";
    CubicPicometer: "pm³", "cubic picometer";

    // SI prefixed liters (subset)
    Teraliter: "TL", "teraliter";
    Gigaliter: "GL", "gigaliter";
    Megaliter: "ML", "megaliter";
    Kiloliter: "kL", "kiloliter";
    Hectoliter: "hL", "hectoliter";
    Decaliter: "daL", "decaliter";
    Liter: "L", "liter";
    Deciliter: "dL", "deciliter";
    Centiliter: "cL", "centiliter";
    Milliliter: "mL", "milliliter";
    Microliter: "µL", "microliter";
    Nanoliter: "nL", "nanoliter";
    Picoliter: "pL", "picoliter";

    // Conventional volume units
    CubicFoot: "ft³", "cubic foot";
    CubicInch: "in³", "cubic inch";
    CubicYard: "yd³", "cubic yard";
    Gallon: "gal", "gallon";
    QuartLiquid: "liq qt", "liquid quart";
    PintLiquid: "liq pt", "liquid pint";
    Cup: "cup", "cup";
    Tablespoon: "tbsp", "tablespoon";
    Teaspoon: "tsp", "teaspoon";
}

// ===== CONVERSION RELATIONSHIPS =====

// SI prefixed cubic meters (linear conversions)
convert_linear! {
    CubicTerameter => CubicMeter: TERA * TERA * TERA;
    CubicGigameter => CubicMeter: GIGA * GIGA * GIGA;
    CubicMegameter => CubicMeter: MEGA * MEGA * MEGA;
    CubicKilometer => CubicMeter: KILO * KILO * KILO;
    CubicHectometer => CubicMeter: HECTO * HECTO * HECTO;
    CubicDecameter => CubicMeter: DECA * DECA * DECA;
    CubicDecimeter => CubicMeter: DECI * DECI * DECI;
    CubicCentimeter => CubicMeter: CENTI * CENTI * CENTI;
    CubicMillimeter => CubicMeter: MILLI * MILLI * MILLI;
    CubicMicrometer => CubicMeter: MICRO * MICRO * MICRO;
    CubicNanometer => CubicMeter: NANO * NANO * NANO;
    CubicPicometer => CubicMeter: PICO * PICO * PICO;
}

// SI prefixed liters (1 liter = 0.001 cubic meters)
convert_linear! {
    Teraliter => CubicMeter: TERA * MILLI;
    Gigaliter => CubicMeter: GIGA * MILLI;
    Megaliter => CubicMeter: MEGA * MILLI;
    Kiloliter => CubicMeter: KILO * MILLI;
    Hectoliter => CubicMeter: HECTO * MILLI;
    Decaliter => CubicMeter: DECA * MILLI;
    Liter => CubicMeter: MILLI;
    Deciliter => CubicMeter: DECI * MILLI;
    Centiliter => CubicMeter: CENTI * MILLI;
    Milliliter => CubicMeter: MILLI * MILLI;
    Microliter => CubicMeter: MICRO * MILLI;
    Nanoliter => CubicMeter: NANO * MILLI;
    Picoliter => CubicMeter: PICO * MILLI;
}

// Conventional volume units (exact UOM conversion factors)
convert_linear! {
    CubicFoot => CubicMeter: 2.831685E-2;
    CubicInch => CubicMeter: 1.638706E-5;
    CubicYard => CubicMeter: 7.645549E-1;
    Gallon => CubicMeter: 3.785412E-3;
    QuartLiquid => CubicMeter: 9.463529E-4;
    PintLiquid => CubicMeter: 4.731765E-4;
    Cup => CubicMeter: 2.365882E-4;
    Tablespoon => CubicMeter: 1.478676E-5;
    Teaspoon => CubicMeter: 4.928922E-6;
}

convert_matrix! {
    CubicMeter => CubicTerameter, CubicGigameter, CubicMegameter, CubicKilometer,
        CubicHectometer, CubicDecameter, CubicDecimeter, CubicCentimeter, CubicMillimeter,
        CubicMicrometer, CubicNanometer, CubicPicometer, Teraliter, Gigaliter, Megaliter,
        Kiloliter, Hectoliter, Decaliter, Liter, Deciliter, Centiliter, Milliliter,
        Microliter, Nanoliter, Picoliter, CubicFoot, CubicInch, CubicYard, Gallon,
        QuartLiquid, PintLiquid, Cup, Tablespoon, Teaspoon
}

// Volume quantity definition (Length³)
use super::{ISQ, SiScale};
quantity!(Volume, ISQ<P3, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, CubicMeter);

// Re-export types for convenience
pub use volume::Volume;
pub use volume::*;

#[cfg(test)]
mod tests {
    macro_rules! test_uom_volume {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::volume,
                uom::si::volume,
                Volume,
                Volume,
                CubicMeter,
                $num_units_unit,
                cubic_meter,
                $uom_unit
            );
        };
    }

    // Test SI prefixed cubic meters
    test_uom_volume!(CubicTerameter, cubic_terameter);
    test_uom_volume!(CubicGigameter, cubic_gigameter);
    test_uom_volume!(CubicMegameter, cubic_megameter);
    test_uom_volume!(CubicKilometer, cubic_kilometer);
    test_uom_volume!(CubicHectometer, cubic_hectometer);
    test_uom_volume!(CubicDecameter, cubic_decameter);
    test_uom_volume!(CubicMeter, cubic_meter);
    test_uom_volume!(CubicDecimeter, cubic_decimeter);
    test_uom_volume!(CubicCentimeter, cubic_centimeter);
    test_uom_volume!(CubicMillimeter, cubic_millimeter);
    test_uom_volume!(CubicMicrometer, cubic_micrometer);
    test_uom_volume!(CubicNanometer, cubic_nanometer);
    test_uom_volume!(CubicPicometer, cubic_picometer);

    // Test SI prefixed liters
    test_uom_volume!(Teraliter, teraliter);
    test_uom_volume!(Gigaliter, gigaliter);
    test_uom_volume!(Megaliter, megaliter);
    test_uom_volume!(Kiloliter, kiloliter);
    test_uom_volume!(Hectoliter, hectoliter);
    test_uom_volume!(Decaliter, decaliter);
    test_uom_volume!(Liter, liter);
    test_uom_volume!(Deciliter, deciliter);
    test_uom_volume!(Centiliter, centiliter);
    test_uom_volume!(Milliliter, milliliter);
    test_uom_volume!(Microliter, microliter);
    test_uom_volume!(Nanoliter, nanoliter);
    test_uom_volume!(Picoliter, picoliter);

    // Test conventional volume units
    test_uom_volume!(CubicFoot, cubic_foot);
    test_uom_volume!(CubicInch, cubic_inch);
    test_uom_volume!(CubicYard, cubic_yard);
    test_uom_volume!(Gallon, gallon);
    test_uom_volume!(QuartLiquid, quart_liquid);
    test_uom_volume!(PintLiquid, pint_liquid);
    test_uom_volume!(Cup, cup);
    test_uom_volume!(Tablespoon, tablespoon);
    test_uom_volume!(Teaspoon, teaspoon);
}
