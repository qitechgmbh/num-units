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
/// ```rust
/// use num_units::unit;
/// use num_units::si::length;
///
/// // The unit! macro generates conversion methods automatically
/// let distance = length::f64::Length::from_meter(100.0);
/// let distance_km = distance.as_kilometer(); // Convert to kilometers
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
/// ```rust
/// use num_units::units;
/// use num_units::convert_unit;
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
/// This macro generates unit structs with automatic implementation of the `Unit` trait.
/// Each unit gets compile-time constants for abbreviation, singular name, and plural name.
///
/// # Syntax
/// ```rust
/// units! {
///     UnitName: "abbreviation", "singular name";
///     UnitName: "abbreviation", "singular name", "plural name";
/// }
/// ```
///
/// # Generated Code
/// For each unit, this macro generates:
/// - A unit struct with `Clone`, `Copy`, `Debug`, and `PartialEq` traits
/// - Implementation of the `Unit` trait with proper constants
/// - Documentation comments for the unit
///
/// # Examples
/// ```rust
/// use num_units::units;
///
/// units! {
///     Meter: "m", "meter";
///     Foot: "ft", "foot";
/// }
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
        )+
    };
}

// ===== BASE UNIT CONVERSION TRAITS =====

/// Trait for converting from a base unit to this unit
pub trait FromBaseUnit<From: crate::unit::Unit> {
    fn to_base(value: f64) -> f64;
    fn from_base(base_value: f64) -> f64;
}

/// Trait for converting to a base unit from this unit
pub trait IntoBaseUnit<To: crate::unit::Unit> {
    fn to_base(value: f64) -> f64;
    fn from_base(base_value: f64) -> f64;
}

/// Macro for establishing bidirectional conversion relationships between units
///
/// This macro creates conversion relationships between two units, allowing automatic
/// conversion in both directions. The conversion functions use closures to define
/// the mathematical relationship between units.
///
/// # Syntax
/// ```rust
/// convert_unit! {
///     TargetUnit: |source_param| conversion_expression;
///     SourceUnit: |target_param| reverse_conversion_expression;
/// }
/// ```
///
/// # Parameters
/// - `TargetUnit`: The unit you're converting TO
/// - `SourceUnit`: The unit you're converting FROM
/// - `conversion_expression`: Expression defining how to convert from source to target
/// - `reverse_conversion_expression`: Expression defining how to convert from target to source
///
/// # Examples
/// ```rust
/// use num_units::convert_unit;
/// use num_units::prefix::KILO;
///
/// // Define conversion between meters and kilometers
/// convert_unit! {
///     Kilometer: |meter| meter / KILO;      // km = m / 1000
///     Meter: |kilometer| kilometer * KILO;  // m = km * 1000
/// }
/// ```
#[macro_export]
macro_rules! convert_unit {
    // New syntax: UnitA: |param| expr; UnitB: |param| expr;
    // Process pairs of units
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1
        impl $crate::unit::FromBaseUnit<$unit2> for $unit1 {
            fn to_base(value: f64) -> f64 {
                let $param2 = value;
                $expr2
            }

            fn from_base(base_value: f64) -> f64 {
                let $param1 = base_value;
                $expr1
            }
        }

        // Reverse conversion: $unit1 -> $unit2
        impl $crate::unit::FromBaseUnit<$unit1> for $unit2 {
            fn to_base(value: f64) -> f64 {
                let $param1 = value;
                $expr1
            }

            fn from_base(base_value: f64) -> f64 {
                let $param2 = base_value;
                $expr2
            }
        }

        // Process remaining conversions recursively
        convert_unit! { $($rest)* }
    };

    // Base case: no more conversions to process
    () => {};
}
