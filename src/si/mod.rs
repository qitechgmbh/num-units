/// # SI Units - International System of Units
///
/// This module provides a complete implementation of the International System of Units (SI),
/// the modern form of the metric system. It includes all base units, derived units, and
/// their conversions.
///
/// ## Overview
///
/// The SI system is the world's most widely used system of measurement. It is based on
/// seven base units and uses decimal prefixes for powers of ten. This implementation
/// provides type-safe dimensional analysis with automatic unit conversions.
///
/// ## Base Units
///
/// The seven SI base units are:
///
/// - **Length**: Meter (m)
/// - **Mass**: Kilogram (kg)
/// - **Time**: Second (s)
/// - **Electric Current**: Ampere (A)
/// - **Temperature**: Kelvin (K)
/// - **Amount of Substance**: Mole (mol)
/// - **Luminous Intensity**: Candela (cd)
///
/// ## Derived Units
///
/// Common derived units include:
///
/// - **Area**: Square meter (m²)
/// - **Volume**: Cubic meter (m³)
/// - **Velocity**: Meter per second (m/s)
/// - **Acceleration**: Meter per second squared (m/s²)
/// - **Force**: Newton (N)
/// - **Energy**: Joule (J)
/// - **Power**: Watt (W)
///
/// ## Usage
///
/// ```rust
/// use num_units::{length, time, mass};
/// use num_units::{length as length_units, time as time_units, mass as mass_units};
///
/// // Create quantities with automatic unit tracking
/// let distance = length::Length::from::<length_units::Meter>(100.0);
/// let time_val = time::Time::from::<time_units::Second>(10.0);
/// let mass_val = mass::Mass::from::<mass_units::Kilogram>(5.0);
///
/// // Automatic dimensional analysis
/// let velocity = distance / time_val;        // m/s
/// let momentum = mass_val * velocity;        // kg⋅m/s
/// let force = mass_val * distance / (time_val * time_val); // N
///
/// // Easy unit conversions
/// let distance_km = distance.to::<length_units::Kilometer>(); // Convert to km
/// let time_ms = time_val.to::<time_units::Millisecond>();     // Convert to ms
/// ```
///
/// ## Architecture
///
/// The SI module is organized as follows:
///
/// - **Base Units**: Fundamental units in each dimension
/// - **Derived Units**: Units created from base unit combinations
/// - **Conversions**: Automatic conversion between related units
/// - **Type Safety**: Compile-time dimensional analysis
///
/// ## Features
///
/// - ✅ Complete SI unit system
/// - ✅ Automatic unit conversions
/// - ✅ Type-safe dimensional analysis
/// - ✅ Support for all numeric types
/// - ✅ No runtime overhead
/// - ✅ Extensive documentation
// Unit modules - define unit types like Meter, Kilogram, etc.
pub mod acceleration;
pub mod amount;
pub mod area;
pub mod current;
pub mod energy;
pub mod force;
pub mod length;
pub mod luminosity;
pub mod mass;
pub mod power;
pub mod scalar;
pub mod temperature;
pub mod time;
pub mod velocity;
pub mod volume;

// Create the SI system with unit scaling using the new syntax
system! {
    SI,
    L => length::Meter,
    M => mass::Kilogram,
    T => time::Second,
    I => current::Ampere,
    TH => temperature::Kelvin,
    N => amount::Mole,
    J => luminosity::Candela
}
