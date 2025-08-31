/// Base unit system for dimensional analysis
///
/// This module defines the core infrastructure for units and conversions.
/// All specific unit definitions are now centralized in the si/ modules.
// Re-export the Unit trait for public use
pub use crate::unit::Unit;

// ===== DIMENSION TYPES =====

/// Temperature dimension
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TemperatureDimension;

/// Length dimension
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct LengthDimension;

/// Mass dimension
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MassDimension;

/// Time dimension
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TimeDimension;

/// Current dimension
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct CurrentDimension;

/// Amount dimension
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AmountDimension;

/// Luminosity dimension
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct LuminosityDimension;

/// Scalar dimension (for dimensionless quantities)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ScalarDimension;

/// Macro for creating new base units
///
/// # Syntax
/// ```ignore
/// base_unit! {
///     dimension: DimensionType;
///     UnitName: "unit name", "abbreviation";
/// }
/// ```
///
/// # Examples
/// ```ignore
/// base_unit! {
///     dimension: LengthDimension;
///     Meter: "meter", "m";
///     Foot: "foot", "ft";
/// }
/// ```
#[macro_export]
macro_rules! base_unit {
    (dimension: $dimension:ty; $($(#[$unit_attr:meta])* $unit:ident: $name:expr, $abbrev:expr;)+) => {
        $(
            $(#[$unit_attr])*
            #[doc = $name]
            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
            pub struct $unit;

            impl Unit for $unit {
                type Dimension = $dimension;
            }
        )+
    };

    // Legacy syntax for backward compatibility (assumes no dimension)
    ($($(#[$unit_attr:meta])* $unit:ident: $name:expr, $abbrev:expr;)+) => {
        $(
            $(#[$unit_attr])*
            #[doc = $name]
            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
            pub struct $unit;

            // Note: Unit trait implementation is minimal now - just a marker trait
            // Actual conversions are handled by FromBaseUnit implementations
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
/// # Syntax
/// ```ignore
/// convert_base_unit! {
///     TargetUnitA: |source_param| { conversion_from_source_to_target };
///     TargetUnitB: |target_param| { conversion_from_target_to_source };
/// }
/// ```
///
/// The first unit is the **target** unit (what you're converting TO).
/// The closure defines how to convert FROM the source unit TO the target unit.
///
/// # Examples
/// ```ignore
/// convert_base_unit! {
///     Celsius: |kelvin| kelvin - 273.15;      // To get Celsius, subtract 273.15 from Kelvin
///     Kelvin: |celsius| celsius + 273.15;     // To get Kelvin, add 273.15 to Celsius
/// }
///
/// convert_base_unit! {
///     Fahrenheit: |kelvin| (kelvin - 273.15) * 9.0 / 5.0 + 32.0;  // Kelvin to Fahrenheit
///     Kelvin: |fahrenheit| (fahrenheit - 32.0) * 5.0 / 9.0 + 273.15;  // Fahrenheit to Kelvin
/// }
///
/// convert_base_unit! {
///     Kilometer: |meter| meter / 1000.0;      // To get Kilometer, divide Meter by 1000
///     Meter: |kilometer| kilometer * 1000.0;  // To get Meter, multiply Kilometer by 1000
/// }
/// ```
#[macro_export]
macro_rules! convert_base_unit {
    // New syntax: UnitA: |param| expr; UnitB: |param| expr;
    // Process pairs of units
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1
        impl FromBaseUnit<$unit2> for $unit1 {
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
        impl FromBaseUnit<$unit1> for $unit2 {
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

    // Base case: empty
    () => {};

    // Legacy syntax patterns (keeping for backward compatibility)

    // |param1: UnitA| { expr }; |param2: UnitB| { expr };
    (|$param1:ident: $from:ident| $forward:expr; |$param2:ident: $to:ident| $reverse:expr;) => {
        // Forward conversion: From -> To
        impl FromBaseUnit<$from> for $to {
            fn to_base(value: f64) -> f64 {
                let $param1 = value;
                $forward
            }

            fn from_base(base_value: f64) -> f64 {
                base_value
            }
        }

        // Reverse conversion: To -> From
        impl FromBaseUnit<$to> for $from {
            fn to_base(value: f64) -> f64 {
                let $param2 = value;
                $reverse
            }

            fn from_base(base_value: f64) -> f64 {
                base_value
            }
        }
    };

    // Old syntax: DerivedUnit = BaseUnit * CONSTANT
    ($($derived:ident = $base:ident * $constant:expr;)+) => {
        $(
            // Forward conversion: Base -> Derived
            impl FromBaseUnit<$base> for $derived {
                fn to_base(value: f64) -> f64 {
                    value * $constant
                }

                fn from_base(base_value: f64) -> f64 {
                    base_value / $constant
                }
            }

            // Reverse conversion: Derived -> Base
            impl FromBaseUnit<$derived> for $base {
                fn to_base(value: f64) -> f64 {
                    value / $constant
                }

                fn from_base(base_value: f64) -> f64 {
                    base_value * $constant
                }
            }
        )+
    };

    // Old syntax with offset: Kelvin = Celsius * 1.0 + 273.15
    ($($derived:ident = $base:ident * $factor:tt + $offset:tt;)+) => {
        $(
            // Forward conversion: Base -> Derived
            impl FromBaseUnit<$base> for $derived {
                fn to_base(value: f64) -> f64 {
                    value * $factor + $offset
                }

                fn from_base(base_value: f64) -> f64 {
                    (base_value - $offset) / $factor
                }
            }

            // Reverse conversion: Derived -> Base
            impl FromBaseUnit<$derived> for $base {
                fn to_base(value: f64) -> f64 {
                    (value - $offset) / $factor
                }

                fn from_base(base_value: f64) -> f64 {
                    value * $factor + $offset
                }
            }
        )+
    };
}
