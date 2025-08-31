/// Base unit system for dimensional analysis
///
/// This module defines the core infrastructure for units and conversions.
/// All specific unit definitions are now centralized in the si/ modules.
// Re-export the Unit trait for public use
pub use crate::unit::Unit;

/// Macro for creating new base units
///
/// # Syntax
/// ```ignore
/// base_units! {
///     dimension: DimensionType;
///     UnitName: "unit name", "abbreviation";
/// }
/// ```
///
/// # Examples
/// ```ignore
/// base_units! {
///     dimension: LengthDimension;
///     Meter: "meter", "m";
///     Foot: "foot", "ft";
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
pub trait FromBaseUnit<From: Unit> {
    fn to_base(value: f64) -> f64;
    fn from_base(base_value: f64) -> f64;
}

/// Trait for converting to a base unit from this unit
pub trait IntoBaseUnit<To: Unit> {
    fn to_base(value: f64) -> f64;
    fn from_base(base_value: f64) -> f64;
}

/// Macro for establishing bidirectional conversion relationships between units
///
/// The first unit is the **target** unit (what you're converting TO).
/// The closure defines how to convert FROM the source unit TO the target unit.
///
/// # Examples
/// ```ignore
///
/// convert_base_unit! {
///     Kilometer: |meter| meter * KILO;      // To get Kilometer, divide Meter by 1000
///     Meter: |kilometer| kilometer / KILO;  // To get Meter, multiply Kilometer by 1000
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
