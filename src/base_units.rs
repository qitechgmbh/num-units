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
/// 1. **Unit Definition**: `base_units!` macro creates unit structs
/// 2. **Trait Implementation**: Automatic implementation of `Unit` trait
/// 3. **Conversion Relationships**: `convert_base_unit!` establishes conversions
/// 4. **Type Safety**: Compile-time dimensional analysis prevents errors
///
/// ## Example Usage
///
/// ```rust
/// use num_units::base_units;
/// use num_units::convert_base_unit;
/// use num_units::prefix::KILO;
///
/// // Define base units
/// base_units! {
///     Meter: "meter", "m";
///     Kilogram: "kilogram", "kg";
/// }
///
/// // Establish conversion relationships
/// convert_base_unit! {
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
/// base_units! {
///     UnitName: "singular name", "abbreviation";
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
/// use num_units::base_units;
///
/// base_units! {
///     Meter: "meter", "m";
///     Kilogram: "kilogram", "kg";
/// }
/// ```
#[macro_export]
macro_rules! base_units {


    // Legacy syntax for backward compatibility
    ($($(#[$unit_attr:meta])* $unit:ident: $name:expr, $abbrev:expr;)+) => {
        $(
            $(#[$unit_attr])*
            #[doc = $name]
            #[doc = concat!("Abbreviation: `", $abbrev, "`")]
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, PartialEq)]
            pub struct $unit;

            impl $crate::unit::Unit for $unit {
                const ABBREVIATION: &'static str = $abbrev;
                const SINGULAR: &'static str = $name;
                const PLURAL: &'static str = concat!($name, "s");
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
/// convert_base_unit! {
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
/// use num_units::convert_base_unit;
/// use num_units::prefix::KILO;
///
/// // Define conversion between meters and kilometers
/// convert_base_unit! {
///     Kilometer: |meter| meter / KILO;      // km = m / 1000
///     Meter: |kilometer| kilometer * KILO;  // m = km * 1000
/// }
/// ```
#[macro_export]
macro_rules! convert_base_unit {
    // New syntax: UnitA: |param| expr; UnitB: |param| expr;
    // Process pairs of units
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1
        impl $crate::base_units::FromBaseUnit<$unit2> for $unit1 {
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
        impl $crate::base_units::FromBaseUnit<$unit1> for $unit2 {
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
        convert_base_unit! { $($rest)* }
    };

    // Base case: no more conversions to process
    () => {};
}
