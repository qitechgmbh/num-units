/// # Power Units - SI Power Measurements
///
/// This module defines SI power units and their conversions. Power is a derived quantity
/// in the SI system with dimensions of mass × length² × time⁻³, with the watt as its base unit.
///
/// ## Base Unit
///
/// - **Watt (W)**: The SI derived unit of power (kg·m²/s³)
///
/// ## SI Prefixed Units
///
/// All SI prefixes from yocto- to yotta- are supported for watts:
/// - **Yottawatt (YW)**: 10²⁴ watts
/// - **Zettawatt (ZW)**: 10²¹ watts
/// - **Exawatt (EW)**: 10¹⁸ watts
/// - **Petawatt (PW)**: 10¹⁵ watts
/// - **Terawatt (TW)**: 10¹² watts
/// - **Gigawatt (GW)**: 10⁹ watts
/// - **Megawatt (MW)**: 10⁶ watts
/// - **Kilowatt (kW)**: 10³ watts
/// - **Hectowatt (hW)**: 10² watts
/// - **Decawatt (daW)**: 10 watts
/// - **Watt (W)**: Base unit
/// - **Deciwatt (dW)**: 10⁻¹ watts
/// - **Centiwatt (cW)**: 10⁻² watts
/// - **Milliwatt (mW)**: 10⁻³ watts
/// - **Microwatt (μW)**: 10⁻⁶ watts
/// - **Nanowatt (nW)**: 10⁻⁹ watts
/// - **Picowatt (pW)**: 10⁻¹² watts
/// - **Femtowatt (fW)**: 10⁻¹⁵ watts
/// - **Attowatt (aW)**: 10⁻¹⁸ watts
/// - **Zeptowatt (zW)**: 10⁻²¹ watts
/// - **Yoctowatt (yW)**: 10⁻²⁴ watts
///
/// ## Other Units
///
/// - **ErgPerSecond**: 10⁻⁷ watts
/// - **FootPoundPerHour**: 3.766161111111111 × 10⁻⁴ watts
/// - **FootPoundPerMinute**: 2.2596966666666666 × 10⁻² watts
/// - **FootPoundPerSecond**: 1.355818 watts
/// - **Horsepower**: 745.6999 watts (mechanical horsepower)
/// - **HorsepowerBoiler**: 9809.50 watts (boiler horsepower)
/// - **HorsepowerElectric**: 746 watts (electric horsepower)
/// - **HorsepowerMetric**: 735.4988 watts (metric horsepower)
/// - **HorsepowerImperial**: 745.70 watts (Imperial horsepower)
/// - **HydraulicHorsepower**: 746.043 watts (hydraulic horsepower)
///
/// ## Usage
///
/// ```rust,ignore
/// use num_units::power::Power;
/// use num_units::power::{Watt, Kilowatt, Horsepower};
///
/// // Create power quantities
/// let power = Power::from::<Watt>(1000.0);
/// let large_power = Power::from::<Kilowatt>(5.0);
/// let engine_power = Power::from::<Horsepower>(150.0);
///
/// // Convert between units
/// let power_kw = power.to::<Kilowatt>();        // 1.0 kW
/// let large_power_w = large_power.to::<Watt>(); // 5000.0 W
/// let engine_power_w = engine_power.to::<Watt>(); // 111854.985 W
///
/// // Arithmetic operations
/// let total_power = power + large_power;        // Automatic conversion
/// // let energy = power * time;                   // Creates energy quantity
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All power operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
use crate::prefix::{
    ATTO, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI, NANO, PETA, PICO, TERA,
    YOCTO, YOTTA, ZEPTO, ZETTA,
};
use typenum::*;

// SI base unit
units! {
    Watt: "W", "watt";
}

// SI prefixed watts
units! {
    Yottawatt: "YW", "yottawatt";
    Zettawatt: "ZW", "zettawatt";
    Exawatt: "EW", "exawatt";
    Petawatt: "PW", "petawatt";
    Terawatt: "TW", "terawatt";
    Gigawatt: "GW", "gigawatt";
    Megawatt: "MW", "megawatt";
    Kilowatt: "kW", "kilowatt";
    Hectowatt: "hW", "hectowatt";
    Decawatt: "daW", "decawatt";
    Deciwatt: "dW", "deciwatt";
    Centiwatt: "cW", "centiwatt";
    Milliwatt: "mW", "milliwatt";
    Microwatt: "μW", "microwatt";
    Nanowatt: "nW", "nanowatt";
    Picowatt: "pW", "picowatt";
    Femtowatt: "fW", "femtowatt";
    Attowatt: "aW", "attowatt";
    Zeptowatt: "zW", "zeptowatt";
    Yoctowatt: "yW", "yoctowatt";
}

// Other power units
units! {
    ErgPerSecond: "erg/s", "erg per second";
    FootPoundPerHour: "ft·lbf/h", "foot pound-force per hour";
    FootPoundPerMinute: "ft·lbf/min", "foot pound-force per minute";
    FootPoundPerSecond: "ft·lbf/s", "foot pound-force per second";
    Horsepower: "hp", "horsepower";
    HorsepowerBoiler: "hp (S)", "horsepower (boiler)";
    HorsepowerElectric: "hp (E)", "horsepower (electric)";
    HorsepowerMetric: "hp (M)", "metric horsepower";
    HorsepowerImperial: "hp (I)", "horsepower (Imperial)";
    HydraulicHorsepower: "hp (hydraulic)", "hydraulic horsepower";
}

// Unit conversions using convert_linear! with exact UOM coefficients
crate::convert_linear! {
    // SI prefixed watts
    Yottawatt => Watt: YOTTA;
    Zettawatt => Watt: ZETTA;
    Exawatt => Watt: EXA;
    Petawatt => Watt: PETA;
    Terawatt => Watt: TERA;
    Gigawatt => Watt: GIGA;
    Megawatt => Watt: MEGA;
    Kilowatt => Watt: KILO;
    Hectowatt => Watt: HECTO;
    Decawatt => Watt: DECA;
    Deciwatt => Watt: DECI;
    Centiwatt => Watt: crate::prefix::CENTI;
    Milliwatt => Watt: MILLI;
    Microwatt => Watt: MICRO;
    Nanowatt => Watt: NANO;
    Picowatt => Watt: PICO;
    Femtowatt => Watt: FEMTO;
    Attowatt => Watt: ATTO;
    Zeptowatt => Watt: ZEPTO;
    Yoctowatt => Watt: YOCTO;

    // Other units - exact UOM coefficients
    ErgPerSecond => Watt: 1.0_E-7;
    FootPoundPerHour => Watt: 3.766_161_111_111_111_E-4;
    FootPoundPerMinute => Watt: 2.259_696_666_666_666_6_E-2;
    FootPoundPerSecond => Watt: 1.355_818_E0;
    Horsepower => Watt: 7.456_999_E2;
    HorsepowerBoiler => Watt: 9.809_50_E3;
    HorsepowerElectric => Watt: 7.46_E2;
    HorsepowerMetric => Watt: 7.354_988_E2;
    HorsepowerImperial => Watt: 7.457_0_E2;
    HydraulicHorsepower => Watt: 7.460_43_E2;
}

crate::convert_matrix! {
    Watt => Yottawatt, Zettawatt, Exawatt, Petawatt, Terawatt, Gigawatt, Megawatt, Kilowatt, Hectowatt, Decawatt, Deciwatt, Centiwatt, Milliwatt, Microwatt, Nanowatt, Picowatt, Femtowatt, Attowatt, Zeptowatt, Yoctowatt, ErgPerSecond, FootPoundPerHour, FootPoundPerMinute, FootPoundPerSecond, Horsepower, HorsepowerBoiler, HorsepowerElectric, HorsepowerMetric, HorsepowerImperial, HydraulicHorsepower
}

// Power quantity definition (Mass×Length²/Time³)
use super::{ISQ, SiScale};
quantity!(Power, ISQ<P2, P1, N3, Z0, Z0, Z0, Z0>, SiScale, Watt);

// UOM compatibility tests
#[cfg(test)]
mod tests {

    macro_rules! test_uom_power {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::power,
                uom::si::power,
                Power,
                Power,
                Watt,
                $num_units_unit,
                watt,
                $uom_unit
            );
        };
    }

    // Test SI prefixed watts
    test_uom_power!(Yottawatt, yottawatt);
    test_uom_power!(Zettawatt, zettawatt);
    test_uom_power!(Exawatt, exawatt);
    test_uom_power!(Petawatt, petawatt);
    test_uom_power!(Terawatt, terawatt);
    test_uom_power!(Gigawatt, gigawatt);
    test_uom_power!(Megawatt, megawatt);
    test_uom_power!(Kilowatt, kilowatt);
    test_uom_power!(Hectowatt, hectowatt);
    test_uom_power!(Decawatt, decawatt);
    test_uom_power!(Watt, watt);
    test_uom_power!(Deciwatt, deciwatt);
    test_uom_power!(Centiwatt, centiwatt);
    test_uom_power!(Milliwatt, milliwatt);
    test_uom_power!(Microwatt, microwatt);
    test_uom_power!(Nanowatt, nanowatt);
    test_uom_power!(Picowatt, picowatt);
    test_uom_power!(Femtowatt, femtowatt);
    test_uom_power!(Attowatt, attowatt);
    test_uom_power!(Zeptowatt, zeptowatt);
    test_uom_power!(Yoctowatt, yoctowatt);

    // Test other units
    test_uom_power!(ErgPerSecond, erg_per_second);
    test_uom_power!(FootPoundPerHour, foot_pound_per_hour);
    test_uom_power!(FootPoundPerMinute, foot_pound_per_minute);
    test_uom_power!(FootPoundPerSecond, foot_pound_per_second);
    test_uom_power!(Horsepower, horsepower);
    test_uom_power!(HorsepowerBoiler, horsepower_boiler);
    test_uom_power!(HorsepowerElectric, horsepower_electric);
    test_uom_power!(HorsepowerMetric, horsepower_metric);
    test_uom_power!(HorsepowerImperial, horsepower_imperial);
    test_uom_power!(HydraulicHorsepower, hydraulic_horsepower);
}

// Re-export types for convenience
pub use power::Power;
pub use power::*;
