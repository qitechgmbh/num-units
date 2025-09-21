/// # Unit System - Core Unit Definitions and Conversions
///
/// This module defines the fundamental traits and macros for working with units
/// in the dimensional analysis system. It provides the building blocks for
/// type-safe unit conversions and dimensional tracking.
///
/// ## Core Concepts
///
/// - **Unit Trait**: Defines basic unit properties (abbreviation, names)
/// - **Unit Macro**: Generates unit structs with conversion methods
/// - **Base Units**: Fundamental units that form the basis of a unit system
/// - **Derived Units**: Units created from combinations of base units
///
/// ## Architecture
///
/// The unit system is built around three key components:
///
/// 1. **Unit Trait**: Provides compile-time unit metadata
/// 2. **Unit Macro**: Generates conversion methods for all numeric types
/// 3. **Base Units**: Define fundamental units like meter, kilogram, second
///
/// ## Example Usage
///
/// ```rust,no_run
/// use num_units::unit;
/// use num_units::length::{Length, Meter, Kilometer};
///
/// // The unit! macro generates conversion methods automatically
/// let distance = Length::from::<Meter>(100.0);
/// let distance_km = distance.to::<Kilometer>(); // Convert to kilometers
/// ```
pub trait Unit {
    /// The abbreviation for this unit (e.g., "m", "kg", "s")
    const ABBREVIATION: &'static str = "unit";

    /// The singular name for this unit (e.g., "meter", "kilogram", "second")
    const SINGULAR: &'static str = "unit";

    /// The plural name for this unit (e.g., "meters", "kilograms", "seconds")
    const PLURAL: &'static str = "units";
}

/// # Base Units - Fundamental Unit Definitions
///
/// This module provides the core infrastructure for defining base units and their
/// conversion relationships. It includes macros for creating unit structs and
/// establishing bidirectional conversion relationships between units.
///
/// ## Key Components
///
/// - **Base Units**: Fundamental units that cannot be expressed in terms of other units
/// - **Derived Units**: Units created from combinations of base units
/// - **Conversion Traits**: Bidirectional conversion between related units
/// - **Unit Macros**: Generate unit structs with proper trait implementations
///
/// ## Architecture
///
/// The base units system uses a layered approach:
///
/// 1. **Unit Definition**: `units!` macro creates unit structs
/// 2. **Trait Implementation**: Automatic implementation of `Unit` trait
/// 3. **Conversion Relationships**: `convert_unit!` establishes conversions
/// 4. **Type Safety**: Compile-time dimensional analysis prevents errors
///
/// ## Example Usage
///
/// ```ignore
/// use num_units::{units, convert_unit, convert_unit_float};
/// use num_units::prefix::KILO;
///
/// // Define base units
/// units! {
///     Meter: "m", "meter";
///     Kilogram: "kg", "kilogram";
/// }
///
/// // Establish conversion relationships
/// convert_unit! {
///     Kilometer: |meter| meter / KILO;
///     Meter: |kilometer| kilometer * KILO;
/// }
/// ```

/// Macro for creating new base units
///
/// This macro generates unit structs with automatic implementation of the `Unit` trait
/// and identity `FromUnit` conversions. Each unit gets compile-time constants for
/// abbreviation, singular name, and plural name, plus the ability to convert to itself.
///
/// # Syntax
/// ```ignore
/// use num_units::units;
///
/// units! {
///     UnitName: "abbreviation", "singular name";
///     // Or with explicit plural:
///     UnitName2: "abbreviation2", "singular name2", "plural name2";
/// }
/// ```
///
/// # Generated Code
/// For each unit, this macro generates:
/// - A unit struct with `Clone`, `Copy`, `Debug`, and `PartialEq` traits
/// - Implementation of the `Unit` trait with proper constants
/// - Identity `FromUnit<UnitName> for UnitName` implementation (unit to itself)
/// - Documentation comments for the unit
///
/// # Examples
/// ```rust,no_run
/// use num_units::units;
///
/// units! {
///     Meter: "m", "meter", "meters";
///     Kilogram: "kg", "kilogram", "kilograms";
///     Foot: "ft", "foot", "feet";
/// }
///
/// // Now Meter automatically implements FromUnit<Meter> for identity conversions
/// // This enables: Length::from::<Meter>(value) and distance.to::<Meter>()
/// ```
#[macro_export]
macro_rules! units {
    // New syntax: abbreviation first, then singular, then optional plural
    ($($(#[$unit_attr:meta])* $unit:ident: $abbrev:expr, $singular:expr, $plural:expr;)+) => {
        $(
            $(#[$unit_attr])*
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, PartialEq)]
            #[doc = $abbrev]
            pub struct $unit;

            impl $crate::unit::Unit for $unit {
                const ABBREVIATION: &'static str = $abbrev;
                const SINGULAR: &'static str = $singular;
                const PLURAL: &'static str = $plural;
            }

            // Automatic identity conversion - unit to itself (generic over any type)
            impl<V: num_traits::Num + Copy> $crate::unit::FromUnit<$unit, V> for $unit {
                fn to_base(value: V) -> V { value }
                fn from_base(base_value: V) -> V { base_value }
            }
        )+
    };

    // New syntax: abbreviation first, then singular (plural auto-generated)
    ($($(#[$unit_attr:meta])* $unit:ident: $abbrev:expr, $singular:expr;)+) => {
        $(
            $(#[$unit_attr])*
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, PartialEq)]
            #[doc = $abbrev]
            pub struct $unit;

            impl $crate::unit::Unit for $unit {
                const ABBREVIATION: &'static str = $abbrev;
                const SINGULAR: &'static str = $singular;
                const PLURAL: &'static str = concat!($singular, "s");
            }

            // Automatic identity conversion - unit to itself (generic over any type)
            impl<V: num_traits::Num + Copy> $crate::unit::FromUnit<$unit, V> for $unit {
                fn to_base(value: V) -> V { value }
                fn from_base(base_value: V) -> V { base_value }
            }
        )+
    };
}

// ===== BASE UNIT CONVERSION TRAITS =====

/// Trait for converting from a base unit to this unit (generic over any numeric type)
pub trait FromUnit<From: crate::unit::Unit, V: num_traits::Num = f64> {
    fn to_base(value: V) -> V;
    fn from_base(base_value: V) -> V;
}

/// Trait for converting to a base unit from this unit (generic over any numeric type)
pub trait IntoUnit<To: crate::unit::Unit, V: num_traits::Num = f64> {
    fn to_base(value: V) -> V;
    fn from_base(base_value: V) -> V;
}

