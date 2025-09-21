use crate::prefix::*;
/// # Temperature Units - Temperature Interval Measurements
///
/// This module defines temperature interval units and their conversions. Temperature intervals
/// represent differences in temperature rather than absolute temperatures. All units are
/// linearly convertible to the base kelvin unit.
///
/// ## Units
///
/// ### SI Base Unit
/// - **Kelvin (K)**: The SI unit of temperature interval
///
/// ### SI Prefixed Units
/// - All SI prefixes from yocto to yotta applied to kelvin
///
/// ### Temperature Scales
/// - **Degree Celsius (°C)**: Equal to kelvin for intervals
/// - **Degree Fahrenheit (°F)**: 5/9 of kelvin
/// - **Degree Rankine (°R)**: 5/9 of kelvin
///
/// ## Implementation Notes
///
/// This implementation follows UOM's temperature_interval quantity, which uses linear
/// conversions without offsets. This differs from absolute temperature scales that require
/// affine transformations (scaling + offset).
use typenum::*;

// ===== SI BASE UNIT =====
units! {
    // SI prefixed kelvin units
    Yottakelvin: "YK", "yottakelvin";
    Zettakelvin: "ZK", "zettakelvin";
    Exakelvin: "EK", "exakelvin";
    Petakelvin: "PK", "petakelvin";
    Terakelvin: "TK", "terakelvin";
    Gigakelvin: "GK", "gigakelvin";
    Megakelvin: "MK", "megakelvin";
    Kilokelvin: "kK", "kilokelvin";
    Hectokelvin: "hK", "hectokelvin";
    Decakelvin: "daK", "decakelvin";
    Kelvin: "K", "kelvin";
    Decikelvin: "dK", "decikelvin";
    Centikelvin: "cK", "centikelvin";
    Millikelvin: "mK", "millikelvin";
    Microkelvin: "µK", "microkelvin";
    Nanokelvin: "nK", "nanokelvin";
    Picokelvin: "pK", "picokelvin";
    Femtokelvin: "fK", "femtokelvin";
    Attokelvin: "aK", "attokelvin";
    Zeptokelvin: "zK", "zeptokelvin";
    Yoctokelvin: "yK", "yoctokelvin";

    // Temperature scales (linear conversions only)
    DegreeCelsius: "°C", "degree Celsius";
    DegreeFahrenheit: "°F", "degree Fahrenheit";
    DegreeRankine: "°R", "degree Rankine";
}

// ===== CONVERSION RELATIONSHIPS =====

// SI prefixed units (linear conversions)
convert_linear! {
    Yottakelvin => Kelvin: YOTTA;
    Zettakelvin => Kelvin: ZETTA;
    Exakelvin => Kelvin: EXA;
    Petakelvin => Kelvin: PETA;
    Terakelvin => Kelvin: TERA;
    Gigakelvin => Kelvin: GIGA;
    Megakelvin => Kelvin: MEGA;
    Kilokelvin => Kelvin: KILO;
    Hectokelvin => Kelvin: HECTO;
    Decakelvin => Kelvin: DECA;
    Decikelvin => Kelvin: DECI;
    Centikelvin => Kelvin: CENTI;
    Millikelvin => Kelvin: MILLI;
    Microkelvin => Kelvin: MICRO;
    Nanokelvin => Kelvin: NANO;
    Picokelvin => Kelvin: PICO;
    Femtokelvin => Kelvin: FEMTO;
    Attokelvin => Kelvin: ATTO;
    Zeptokelvin => Kelvin: ZEPTO;
    Yoctokelvin => Kelvin: YOCTO;
}

// Temperature scales (linear conversions for intervals)
convert_linear! {
    DegreeCelsius => Kelvin: 1.0;
    DegreeFahrenheit => Kelvin: 5.0 / 9.0;
    DegreeRankine => Kelvin: 5.0 / 9.0;
}

convert_matrix! {
    Kelvin => Yottakelvin, Zettakelvin, Exakelvin, Petakelvin, Terakelvin, Gigakelvin,
        Megakelvin, Kilokelvin, Hectokelvin, Decakelvin, Decikelvin, Centikelvin,
        Millikelvin, Microkelvin, Nanokelvin, Picokelvin, Femtokelvin, Attokelvin,
        Zeptokelvin, Yoctokelvin, DegreeCelsius, DegreeFahrenheit, DegreeRankine
}

// Temperature quantity definition
use super::{ISQ, SiScale};
quantity!(Temperature, ISQ<Z0, Z0, Z0, Z0, P1, Z0, Z0>, SiScale, Kelvin);

// Re-export types for convenience
pub use temperature::Temperature;
pub use temperature::*;

#[cfg(test)]
mod tests {
    macro_rules! test_uom_temperature {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::temperature,
                uom::si::temperature_interval,
                Temperature,
                TemperatureInterval,
                Kelvin,
                $num_units_unit,
                kelvin,
                $uom_unit
            );
        };
    }

    // Test SI prefixed kelvin units
    test_uom_temperature!(Yottakelvin, yottakelvin);
    test_uom_temperature!(Zettakelvin, zettakelvin);
    test_uom_temperature!(Exakelvin, exakelvin);
    test_uom_temperature!(Petakelvin, petakelvin);
    test_uom_temperature!(Terakelvin, terakelvin);
    test_uom_temperature!(Gigakelvin, gigakelvin);
    test_uom_temperature!(Megakelvin, megakelvin);
    test_uom_temperature!(Kilokelvin, kilokelvin);
    test_uom_temperature!(Hectokelvin, hectokelvin);
    test_uom_temperature!(Decakelvin, decakelvin);
    test_uom_temperature!(Kelvin, kelvin);
    test_uom_temperature!(Decikelvin, decikelvin);
    test_uom_temperature!(Centikelvin, centikelvin);
    test_uom_temperature!(Millikelvin, millikelvin);
    test_uom_temperature!(Microkelvin, microkelvin);
    test_uom_temperature!(Nanokelvin, nanokelvin);
    test_uom_temperature!(Picokelvin, picokelvin);
    test_uom_temperature!(Femtokelvin, femtokelvin);
    test_uom_temperature!(Attokelvin, attokelvin);
    test_uom_temperature!(Zeptokelvin, zeptokelvin);
    test_uom_temperature!(Yoctokelvin, yoctokelvin);

    // Test temperature scales
    test_uom_temperature!(DegreeCelsius, degree_celsius);
    test_uom_temperature!(DegreeFahrenheit, degree_fahrenheit);
    test_uom_temperature!(DegreeRankine, degree_rankine);
}
