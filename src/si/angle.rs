/// # Angle Units - Angular Measurements
///
/// This module defines angle units and their conversions. Angles are dimensionless quantities
/// that measure rotation or orientation, with the radian as the SI base unit.
///
/// ## Important Note
///
/// Due to architectural limitations in num-units (lack of "kind" system like UOM),
/// angle units cannot have a separate quantity type. Instead, they are implemented
/// using the existing Unitless base quantity. This means:
/// - Angle quantities are treated as dimensionless scalars
/// - UOM compatibility tests cannot be performed for angle units
/// - All conversions are done through the Unitless base unit
///
/// ## Base Unit
///
/// - **Revolution (r)**: Full circle rotation, the base unit for angles in this implementation
///
/// ## Other Units
///
/// - **Radian (rad)**: The SI unit of angle (1/2π revolutions)
/// - **Degree (°)**: 1/360 of a revolution
/// - **Gon (gon)**: 1/400 of a revolution
/// - **Mil**: 1/6400 of a revolution
/// - **Minute (′)**: 1/60 of a degree (1/21600 revolutions)
/// - **Second (″)**: 1/60 of a minute (1/1296000 revolutions)
///
use super::scalar::Unitless;

// Base angle unit
units! {
    Revolution: "r", "revolution";
}

// Other angle units
units! {
    Radian: "rad", "radian";
    Degree: "°", "degree";
    Gon: "gon", "gon";
    Mil: "mil", "mil";
    Minute: "′", "minute";
    Second: "″", "second";
}

// Angle unit conversions with Revolution as base unit
// All conversions are to Unitless (dimensionless base)
crate::convert_linear! {
    Revolution => Unitless: 1.0;                           // Base unit
    Radian => Unitless: 0.159_154_943_091_895_3;           // 1/(2π)
    Degree => Unitless: 2.777_777_777_777_777_8_E-3;       // 1/360
    Gon => Unitless: 2.5_E-3;                              // 1/400
    Mil => Unitless: 1.562_5_E-4;                          // 1/6400
    Minute => Unitless: 4.629_629_629_629_63_E-5;          // 1/21600
    Second => Unitless: 7.716_049_382_716_049_E-7;         // 1/1296000
}

crate::convert_matrix! {
    Unitless => Revolution, Radian, Degree, Gon, Mil, Minute, Second
}
