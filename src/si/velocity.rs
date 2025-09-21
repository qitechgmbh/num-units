use crate::prefix::*;
/// # Velocity Units - Speed Measurements
///
/// This module defines velocity (speed) units and their conversions. Velocity is
/// measured in meters per second as the SI derived unit, with various prefixed
/// units and conventional speed units.
///
/// ## Units
///
/// ### SI Derived Unit
/// - **Meter per second (m/s)**: The SI unit of velocity
///
/// ### SI Prefixed Units
/// - All SI prefixes from yocto to yotta applied to meters per second
///
/// ### Conventional Velocity Units
/// - **Foot per hour/second/minute**: Imperial/US customary velocity units
/// - **Inch per second/minute**: Small imperial velocity units
/// - **Kilometer per hour**: Common metric speed unit
/// - **Mile per hour/minute/second**: Imperial speed units
/// - **Knot**: Nautical speed unit
/// - **Millimeter per minute**: Very slow velocity unit
///
/// ### Special Units
/// - **Speed of light in vacuum (c)**: 299,792,458 m/s
/// - **Natural unit of velocity**: Same as speed of light
/// - **Atomic unit of velocity**: Hartree atomic unit
use typenum::*;

// ===== SI DERIVED UNIT =====
units! {
    // SI prefixed meters per second
    YottameterPerSecond: "Ym/s", "yottameter per second";
    ZettameterPerSecond: "Zm/s", "zettameter per second";
    ExameterPerSecond: "Em/s", "exameter per second";
    PetameterPerSecond: "Pm/s", "petameter per second";
    TerameterPerSecond: "Tm/s", "terameter per second";
    GigameterPerSecond: "Gm/s", "gigameter per second";
    MegameterPerSecond: "Mm/s", "megameter per second";
    KilometerPerSecond: "km/s", "kilometer per second";
    HectometerPerSecond: "hm/s", "hectometer per second";
    DecameterPerSecond: "dam/s", "decameter per second";
    MeterPerSecond: "m/s", "meter per second";
    DecimeterPerSecond: "dm/s", "decimeter per second";
    CentimeterPerSecond: "cm/s", "centimeter per second";
    MillimeterPerSecond: "mm/s", "millimeter per second";
    MicrometerPerSecond: "µm/s", "micrometer per second";
    NanometerPerSecond: "nm/s", "nanometer per second";
    PicometerPerSecond: "pm/s", "picometer per second";
    FemtometerPerSecond: "fm/s", "femtometer per second";
    AttometerPerSecond: "am/s", "attometer per second";
    ZeptometerPerSecond: "zm/s", "zeptometer per second";
    YoctometerPerSecond: "ym/s", "yoctometer per second";

    // Conventional velocity units
    FootPerHour: "ft/h", "foot per hour";
    FootPerMinute: "ft/min", "foot per minute";
    FootPerSecond: "ft/s", "foot per second";
    InchPerSecond: "in/s", "inch per second";
    InchPerMinute: "in/min", "inch per minute";
    KilometerPerHour: "km/h", "kilometer per hour";
    Knot: "kn", "knot";
    MilePerHour: "mi/h", "mile per hour";
    MilePerMinute: "mi/min", "mile per minute";
    MilePerSecond: "mi/s", "mile per second";
    MillimeterPerMinute: "mm/min", "millimeter per minute";

    // Special velocity units
    AtomicUnitOfVelocity: "a₀ · Eₕ/ħ", "atomic unit of velocity";
    NaturalUnitOfVelocity: "c", "natural unit of velocity";
    SpeedOfLightInVacuum: "c", "speed of light in vacuum";
}

// ===== CONVERSION RELATIONSHIPS =====

// SI prefixed units (linear conversions)
convert_linear! {
    YottameterPerSecond => MeterPerSecond: YOTTA;
    ZettameterPerSecond => MeterPerSecond: ZETTA;
    ExameterPerSecond => MeterPerSecond: EXA;
    PetameterPerSecond => MeterPerSecond: PETA;
    TerameterPerSecond => MeterPerSecond: TERA;
    GigameterPerSecond => MeterPerSecond: GIGA;
    MegameterPerSecond => MeterPerSecond: MEGA;
    KilometerPerSecond => MeterPerSecond: KILO;
    HectometerPerSecond => MeterPerSecond: HECTO;
    DecameterPerSecond => MeterPerSecond: DECA;
    DecimeterPerSecond => MeterPerSecond: DECI;
    CentimeterPerSecond => MeterPerSecond: CENTI;
    MillimeterPerSecond => MeterPerSecond: MILLI;
    MicrometerPerSecond => MeterPerSecond: MICRO;
    NanometerPerSecond => MeterPerSecond: NANO;
    PicometerPerSecond => MeterPerSecond: PICO;
    FemtometerPerSecond => MeterPerSecond: FEMTO;
    AttometerPerSecond => MeterPerSecond: ATTO;
    ZeptometerPerSecond => MeterPerSecond: ZEPTO;
    YoctometerPerSecond => MeterPerSecond: YOCTO;
}

// Conventional velocity units (exact UOM conversion factors)
convert_linear! {
    FootPerHour => MeterPerSecond: 8.466666666666667E-5;
    FootPerMinute => MeterPerSecond: 5.08E-3;
    FootPerSecond => MeterPerSecond: 3.048E-1;
    InchPerSecond => MeterPerSecond: 2.54E-2;
    InchPerMinute => MeterPerSecond: 4.233333333333333334E-4;
    KilometerPerHour => MeterPerSecond: 2.777777777777778E-1;
    Knot => MeterPerSecond: 5.144444444444445E-1;
    MilePerHour => MeterPerSecond: 4.4704E-1;
    MilePerMinute => MeterPerSecond: 2.68224E1;
    MilePerSecond => MeterPerSecond: 1.609344E3;
    MillimeterPerMinute => MeterPerSecond: 1.666666666666666667E-5;
}

// Special velocity units
convert_linear! {
    AtomicUnitOfVelocity => MeterPerSecond: 2.18769126364E6;
    NaturalUnitOfVelocity => MeterPerSecond: 299792458.0;
    SpeedOfLightInVacuum => MeterPerSecond: 299792458.0;
}

convert_matrix! {
    MeterPerSecond => YottameterPerSecond, ZettameterPerSecond, ExameterPerSecond,
        PetameterPerSecond, TerameterPerSecond, GigameterPerSecond, MegameterPerSecond,
        KilometerPerSecond, HectometerPerSecond, DecameterPerSecond, DecimeterPerSecond,
        CentimeterPerSecond, MillimeterPerSecond, MicrometerPerSecond, NanometerPerSecond,
        PicometerPerSecond, FemtometerPerSecond, AttometerPerSecond, ZeptometerPerSecond,
        YoctometerPerSecond, FootPerHour, FootPerMinute, FootPerSecond, InchPerSecond,
        InchPerMinute, KilometerPerHour, Knot, MilePerHour, MilePerMinute, MilePerSecond,
        MillimeterPerMinute, AtomicUnitOfVelocity, NaturalUnitOfVelocity, SpeedOfLightInVacuum
}

use crate::prefix::{DECA, EXA, GIGA, HECTO, KILO, MEGA, PETA, TERA, YOTTA, ZETTA};

// Velocity quantity definition (Length/Time)
use super::{ISQ, SiScale};
quantity!(Velocity, ISQ<P1, Z0, N1, Z0, Z0, Z0, Z0>, SiScale, MeterPerSecond);

// Re-export types for convenience
pub use velocity::Velocity;
pub use velocity::*;

#[cfg(test)]
mod tests {

    macro_rules! test_uom_velocity {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::velocity,
                uom::si::velocity,
                Velocity,
                Velocity,
                MeterPerSecond,
                $num_units_unit,
                meter_per_second,
                $uom_unit
            );
        };
    }

    // Test SI prefixed meters per second
    test_uom_velocity!(YottameterPerSecond, yottameter_per_second);
    test_uom_velocity!(ZettameterPerSecond, zettameter_per_second);
    test_uom_velocity!(ExameterPerSecond, exameter_per_second);
    test_uom_velocity!(PetameterPerSecond, petameter_per_second);
    test_uom_velocity!(TerameterPerSecond, terameter_per_second);
    test_uom_velocity!(GigameterPerSecond, gigameter_per_second);
    test_uom_velocity!(MegameterPerSecond, megameter_per_second);
    test_uom_velocity!(KilometerPerSecond, kilometer_per_second);
    test_uom_velocity!(HectometerPerSecond, hectometer_per_second);
    test_uom_velocity!(DecameterPerSecond, decameter_per_second);
    test_uom_velocity!(MeterPerSecond, meter_per_second);
    test_uom_velocity!(DecimeterPerSecond, decimeter_per_second);
    test_uom_velocity!(CentimeterPerSecond, centimeter_per_second);
    test_uom_velocity!(MillimeterPerSecond, millimeter_per_second);
    test_uom_velocity!(MicrometerPerSecond, micrometer_per_second);
    test_uom_velocity!(NanometerPerSecond, nanometer_per_second);
    test_uom_velocity!(PicometerPerSecond, picometer_per_second);
    test_uom_velocity!(FemtometerPerSecond, femtometer_per_second);
    test_uom_velocity!(AttometerPerSecond, attometer_per_second);
    test_uom_velocity!(ZeptometerPerSecond, zeptometer_per_second);
    test_uom_velocity!(YoctometerPerSecond, yoctometer_per_second);

    // Test conventional velocity units
    test_uom_velocity!(FootPerHour, foot_per_hour);
    test_uom_velocity!(FootPerMinute, foot_per_minute);
    test_uom_velocity!(FootPerSecond, foot_per_second);
    test_uom_velocity!(InchPerSecond, inch_per_second);
    test_uom_velocity!(InchPerMinute, inch_per_minute);
    test_uom_velocity!(KilometerPerHour, kilometer_per_hour);
    test_uom_velocity!(Knot, knot);
    test_uom_velocity!(MilePerHour, mile_per_hour);
    test_uom_velocity!(MilePerMinute, mile_per_minute);
    test_uom_velocity!(MilePerSecond, mile_per_second);
    test_uom_velocity!(MillimeterPerMinute, millimeter_per_minute);

    // Test special velocity units
    test_uom_velocity!(AtomicUnitOfVelocity, atomic_unit_of_velocity);
    test_uom_velocity!(NaturalUnitOfVelocity, natural_unit_of_velocity);
    test_uom_velocity!(SpeedOfLightInVacuum, speed_of_light_in_vacuum);
}
