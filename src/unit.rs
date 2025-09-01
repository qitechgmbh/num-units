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
/// use num_units::length::Length;
/// use num_units::si::length::{Meter, Kilometer};
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
/// ```rust,no_run
/// use num_units::{units, convert_unit};
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
/// ```rust,no_run
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
/// - Identity `FromUnit<UnitName> for UnitName` implementation (unit to itself)
/// - Documentation comments for the unit
///
/// # Examples
/// ```rust,no_run
/// use num_units::units;
///
/// units! {
///     Meter: "m", "meter";
///     Foot: "ft", "foot";
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

/// Macro for establishing bidirectional conversion relationships between units (floating-point)
///
/// This macro creates conversion relationships between two units using floating-point
/// arithmetic. Use this for conversions that involve fractional factors. This is the
/// top-level macro that calls the hierarchical conversion macros.
///
/// # Syntax
/// ```rust,no_run
/// use num_units::convert_unit;
/// 
/// convert_unit! {
///     TargetUnit: |source_param| conversion_expression;
///     SourceUnit: |target_param| reverse_conversion_expression;
/// }
/// ```
///
/// # Examples
/// ```rust,no_run
/// use num_units::convert_unit;
/// use num_units::prefix::KILO;
///
/// // Define conversion with floating-point factors
/// convert_unit! {
///     Kilometer: |meter| meter / KILO;      // km = m / 1000.0
///     Meter: |kilometer| kilometer * KILO;  // m = km * 1000.0
/// }
/// ```
///
/// This automatically generates conversions for f32 and f64 types.
#[macro_export]
macro_rules! convert_unit {
    ($($input:tt)*) => {
        convert_unit_float! { $($input)* }
    };
}

// ===== HIERARCHICAL CONVERSION MACROS =====

/// Macro for specific integer type conversions (i8)
#[macro_export]
macro_rules! convert_unit_i8 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (i8 version)
        impl $crate::unit::FromUnit<$unit2, i8> for $unit1 {
            fn to_base(value: i8) -> i8 {
                let $param2 = value as f64;
                ($expr2).round() as i8
            }

            fn from_base(base_value: i8) -> i8 {
                let $param1 = base_value as f64;
                ($expr1).round() as i8
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (i8 version)
        impl $crate::unit::FromUnit<$unit1, i8> for $unit2 {
            fn to_base(value: i8) -> i8 {
                let $param1 = value as f64;
                ($expr1).round() as i8
            }

            fn from_base(base_value: i8) -> i8 {
                let $param2 = base_value as f64;
                ($expr2).round() as i8
            }
        }

        convert_unit_i8! { $($rest)* }
    };
    () => {};
}

/// Macro for specific integer type conversions (i16)
#[macro_export]
macro_rules! convert_unit_i16 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (i16 version)
        impl $crate::unit::FromUnit<$unit2, i16> for $unit1 {
            fn to_base(value: i16) -> i16 {
                let $param2 = value as f64;
                ($expr2).round() as i16
            }

            fn from_base(base_value: i16) -> i16 {
                let $param1 = base_value as f64;
                ($expr1).round() as i16
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (i16 version)
        impl $crate::unit::FromUnit<$unit1, i16> for $unit2 {
            fn to_base(value: i16) -> i16 {
                let $param1 = value as f64;
                ($expr1).round() as i16
            }

            fn from_base(base_value: i16) -> i16 {
                let $param2 = base_value as f64;
                ($expr2).round() as i16
            }
        }

        convert_unit_i16! { $($rest)* }
    };
    () => {};
}

/// Macro for specific integer type conversions (i32)
#[macro_export]
macro_rules! convert_unit_i32 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (i32 version)
        impl $crate::unit::FromUnit<$unit2, i32> for $unit1 {
            fn to_base(value: i32) -> i32 {
                let $param2 = value as f64;
                ($expr2).round() as i32
            }

            fn from_base(base_value: i32) -> i32 {
                let $param1 = base_value as f64;
                ($expr1).round() as i32
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (i32 version)
        impl $crate::unit::FromUnit<$unit1, i32> for $unit2 {
            fn to_base(value: i32) -> i32 {
                let $param1 = value as f64;
                ($expr1).round() as i32
            }

            fn from_base(base_value: i32) -> i32 {
                let $param2 = base_value as f64;
                ($expr2).round() as i32
            }
        }

        convert_unit_i32! { $($rest)* }
    };
    () => {};
}

/// Macro for specific integer type conversions (i64)
#[macro_export]
macro_rules! convert_unit_i64 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (i64 version)
        impl $crate::unit::FromUnit<$unit2, i64> for $unit1 {
            fn to_base(value: i64) -> i64 {
                let $param2 = value;
                $expr2
            }

            fn from_base(base_value: i64) -> i64 {
                let $param1 = base_value;
                $expr1
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (i64 version)
        impl $crate::unit::FromUnit<$unit1, i64> for $unit2 {
            fn to_base(value: i64) -> i64 {
                let $param1 = value;
                $expr1
            }

            fn from_base(base_value: i64) -> i64 {
                let $param2 = base_value;
                $expr2
            }
        }

        convert_unit_i64! { $($rest)* }
    };
    () => {};
}

/// Macro for specific integer type conversions (i128)
#[macro_export]
macro_rules! convert_unit_i128 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (i128 version)
        impl $crate::unit::FromUnit<$unit2, i128> for $unit1 {
            fn to_base(value: i128) -> i128 {
                let $param2 = value;
                $expr2
            }

            fn from_base(base_value: i128) -> i128 {
                let $param1 = base_value;
                $expr1
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (i128 version)
        impl $crate::unit::FromUnit<$unit1, i128> for $unit2 {
            fn to_base(value: i128) -> i128 {
                let $param1 = value;
                $expr1
            }

            fn from_base(base_value: i128) -> i128 {
                let $param2 = base_value;
                $expr2
            }
        }

        convert_unit_i128! { $($rest)* }
    };
    () => {};
}

/// Macro for unsigned integer type conversions (u8)
#[macro_export]
macro_rules! convert_unit_u8 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (u8 version)
        impl $crate::unit::FromUnit<$unit2, u8> for $unit1 {
            fn to_base(value: u8) -> u8 {
                let $param2 = value as f64;
                ($expr2).round() as u8
            }

            fn from_base(base_value: u8) -> u8 {
                let $param1 = base_value as f64;
                ($expr1).round() as u8
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (u8 version)
        impl $crate::unit::FromUnit<$unit1, u8> for $unit2 {
            fn to_base(value: u8) -> u8 {
                let $param1 = value as f64;
                ($expr1).round() as u8
            }

            fn from_base(base_value: u8) -> u8 {
                let $param2 = base_value as f64;
                ($expr2).round() as u8
            }
        }

        convert_unit_u8! { $($rest)* }
    };
    () => {};
}

/// Macro for unsigned integer type conversions (u16)
#[macro_export]
macro_rules! convert_unit_u16 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (u16 version)
        impl $crate::unit::FromUnit<$unit2, u16> for $unit1 {
            fn to_base(value: u16) -> u16 {
                let $param2 = value as f64;
                ($expr2).round() as u16
            }

            fn from_base(base_value: u16) -> u16 {
                let $param1 = base_value as f64;
                ($expr1).round() as u16
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (u16 version)
        impl $crate::unit::FromUnit<$unit1, u16> for $unit2 {
            fn to_base(value: u16) -> u16 {
                let $param1 = value as f64;
                ($expr1).round() as u16
            }

            fn from_base(base_value: u16) -> u16 {
                let $param2 = base_value as f64;
                ($expr2).round() as u16
            }
        }

        convert_unit_u16! { $($rest)* }
    };
    () => {};
}

/// Macro for unsigned integer type conversions (u32)
#[macro_export]
macro_rules! convert_unit_u32 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (u32 version)
        impl $crate::unit::FromUnit<$unit2, u32> for $unit1 {
            fn to_base(value: u32) -> u32 {
                let $param2 = value as f64;
                ($expr2).round() as u32
            }

            fn from_base(base_value: u32) -> u32 {
                let $param1 = base_value as f64;
                ($expr1).round() as u32
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (u32 version)
        impl $crate::unit::FromUnit<$unit1, u32> for $unit2 {
            fn to_base(value: u32) -> u32 {
                let $param1 = value as f64;
                ($expr1).round() as u32
            }

            fn from_base(base_value: u32) -> u32 {
                let $param2 = base_value as f64;
                ($expr2).round() as u32
            }
        }

        convert_unit_u32! { $($rest)* }
    };
    () => {};
}

/// Macro for unsigned integer type conversions (u64)
#[macro_export]
macro_rules! convert_unit_u64 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (u64 version)
        impl $crate::unit::FromUnit<$unit2, u64> for $unit1 {
            fn to_base(value: u64) -> u64 {
                let $param2 = value;
                $expr2
            }

            fn from_base(base_value: u64) -> u64 {
                let $param1 = base_value;
                $expr1
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (u64 version)
        impl $crate::unit::FromUnit<$unit1, u64> for $unit2 {
            fn to_base(value: u64) -> u64 {
                let $param1 = value;
                $expr1
            }

            fn from_base(base_value: u64) -> u64 {
                let $param2 = base_value;
                $expr2
            }
        }

        convert_unit_u64! { $($rest)* }
    };
    () => {};
}

/// Macro for unsigned integer type conversions (u128)
#[macro_export]
macro_rules! convert_unit_u128 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (u128 version)
        impl $crate::unit::FromUnit<$unit2, u128> for $unit1 {
            fn to_base(value: u128) -> u128 {
                let $param2 = value;
                $expr2
            }

            fn from_base(base_value: u128) -> u128 {
                let $param1 = base_value;
                $expr1
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (u128 version)
        impl $crate::unit::FromUnit<$unit1, u128> for $unit2 {
            fn to_base(value: u128) -> u128 {
                let $param1 = value;
                $expr1
            }

            fn from_base(base_value: u128) -> u128 {
                let $param2 = base_value;
                $expr2
            }
        }

        convert_unit_u128! { $($rest)* }
    };
    () => {};
}

/// Macro for floating-point type conversions (f32)
#[macro_export]
macro_rules! convert_unit_f32 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (f32 version)
        impl $crate::unit::FromUnit<$unit2, f32> for $unit1 {
            fn to_base(value: f32) -> f32 {
                let $param2 = value as f64;
                ($expr2) as f32
            }

            fn from_base(base_value: f32) -> f32 {
                let $param1 = base_value as f64;
                ($expr1) as f32
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (f32 version)
        impl $crate::unit::FromUnit<$unit1, f32> for $unit2 {
            fn to_base(value: f32) -> f32 {
                let $param1 = value as f64;
                ($expr1) as f32
            }

            fn from_base(base_value: f32) -> f32 {
                let $param2 = base_value as f64;
                ($expr2) as f32
            }
        }

        convert_unit_f32! { $($rest)* }
    };
    () => {};
}

/// Macro for floating-point type conversions (f64)
#[macro_export]
macro_rules! convert_unit_f64 {
    ($unit1:ident: |$param1:ident| $expr1:expr; $unit2:ident: |$param2:ident| $expr2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (f64 generic version)
        impl $crate::unit::FromUnit<$unit2, f64> for $unit1 {
            fn to_base(value: f64) -> f64 {
                let $param2 = value;
                $expr2
            }

            fn from_base(base_value: f64) -> f64 {
                let $param1 = base_value;
                $expr1
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (f64 generic version)
        impl $crate::unit::FromUnit<$unit1, f64> for $unit2 {
            fn to_base(value: f64) -> f64 {
                let $param1 = value;
                $expr1
            }

            fn from_base(base_value: f64) -> f64 {
                let $param2 = base_value;
                $expr2
            }
        }

        convert_unit_f64! { $($rest)* }
    };
    () => {};
}

/// Macro for all signed integer type conversions
#[macro_export]
macro_rules! convert_unit_signed {
    ($($input:tt)*) => {
        convert_unit_i8! { $($input)* }
        convert_unit_i16! { $($input)* }
        convert_unit_i32! { $($input)* }
        convert_unit_i64! { $($input)* }
        convert_unit_i128! { $($input)* }
    };
}

/// Macro for all unsigned integer type conversions
#[macro_export]
macro_rules! convert_unit_unsigned {
    ($($input:tt)*) => {
        convert_unit_u8! { $($input)* }
        convert_unit_u16! { $($input)* }
        convert_unit_u32! { $($input)* }
        convert_unit_u64! { $($input)* }
        convert_unit_u128! { $($input)* }
    };
}

/// Macro for all floating-point type conversions
#[macro_export]
macro_rules! convert_unit_float {
    ($($input:tt)*) => {
        convert_unit_f32! { $($input)* }
        convert_unit_f64! { $($input)* }
    };
}

/// Macro for establishing bidirectional conversion relationships between units (integer)
///
/// This macro creates conversion relationships between two units using integer
/// arithmetic for exact conversions. Use this for conversions with exact integer factors.
///
/// # Syntax
/// ```rust,no_run
/// use num_units::convert_unit_int;
/// 
/// convert_unit_int! {
///     TargetUnit: factor_to_target;
///     SourceUnit: factor_to_source;
/// }
/// ```
///
/// # Examples
/// ```rust,no_run
/// use num_units::convert_unit_int;
///
/// // Define conversion with exact integer factors
/// convert_unit_int! {
///     Millimeter: 1000;  // 1 meter = 1000 millimeters
///     Meter: 1;          // 1 meter = 1 meter (base unit)
/// }
/// ```
///
/// This generates both integer and floating-point implementations, with integer
/// implementations using exact arithmetic when possible.
#[macro_export]
macro_rules! convert_unit_int {
    // Process pairs of units with integer conversion factors
    ($unit1:ident: $factor1:expr; $unit2:ident: $factor2:expr; $($rest:tt)*) => {
        // Forward conversion: $unit2 -> $unit1 (f64 version)
        impl $crate::unit::FromUnit<$unit2, f64> for $unit1 {
            fn to_base(value: f64) -> f64 {
                value * ($factor2 as f64) / ($factor1 as f64)
            }

            fn from_base(base_value: f64) -> f64 {
                base_value * ($factor1 as f64) / ($factor2 as f64)
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (f64 version)
        impl $crate::unit::FromUnit<$unit1, f64> for $unit2 {
            fn to_base(value: f64) -> f64 {
                value * ($factor1 as f64) / ($factor2 as f64)
            }

            fn from_base(base_value: f64) -> f64 {
                base_value * ($factor2 as f64) / ($factor1 as f64)
            }
        }

        // Forward conversion: $unit2 -> $unit1 (f32 version)
        impl $crate::unit::FromUnit<$unit2, f32> for $unit1 {
            fn to_base(value: f32) -> f32 {
                value * ($factor2 as f32) / ($factor1 as f32)
            }

            fn from_base(base_value: f32) -> f32 {
                base_value * ($factor1 as f32) / ($factor2 as f32)
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (f32 version)
        impl $crate::unit::FromUnit<$unit1, f32> for $unit2 {
            fn to_base(value: f32) -> f32 {
                value * ($factor1 as f32) / ($factor2 as f32)
            }

            fn from_base(base_value: f32) -> f32 {
                base_value * ($factor2 as f32) / ($factor1 as f32)
            }
        }

        // Forward conversion: $unit2 -> $unit1 (i32 version)
        impl $crate::unit::FromUnit<$unit2, i32> for $unit1 {
            fn to_base(value: i32) -> i32 {
                // Use integer arithmetic when possible
                if $factor2 % $factor1 == 0 {
                    value * ($factor2 / $factor1) as i32
                } else {
                    ((value as i64) * ($factor2 as i64) / ($factor1 as i64)) as i32
                }
            }

            fn from_base(base_value: i32) -> i32 {
                if $factor1 % $factor2 == 0 {
                    base_value * ($factor1 / $factor2) as i32
                } else {
                    ((base_value as i64) * ($factor1 as i64) / ($factor2 as i64)) as i32
                }
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (i32 version)
        impl $crate::unit::FromUnit<$unit1, i32> for $unit2 {
            fn to_base(value: i32) -> i32 {
                if $factor1 % $factor2 == 0 {
                    value * ($factor1 / $factor2) as i32
                } else {
                    ((value as i64) * ($factor1 as i64) / ($factor2 as i64)) as i32
                }
            }

            fn from_base(base_value: i32) -> i32 {
                if $factor2 % $factor1 == 0 {
                    base_value * ($factor2 / $factor1) as i32
                } else {
                    ((base_value as i64) * ($factor2 as i64) / ($factor1 as i64)) as i32
                }
            }
        }

        // Forward conversion: $unit2 -> $unit1 (i64 version)
        impl $crate::unit::FromUnit<$unit2, i64> for $unit1 {
            fn to_base(value: i64) -> i64 {
                value * ($factor2 as i64) / ($factor1 as i64)
            }

            fn from_base(base_value: i64) -> i64 {
                base_value * ($factor1 as i64) / ($factor2 as i64)
            }
        }

        // Reverse conversion: $unit1 -> $unit2 (i64 version)
        impl $crate::unit::FromUnit<$unit1, i64> for $unit2 {
            fn to_base(value: i64) -> i64 {
                value * ($factor1 as i64) / ($factor2 as i64)
            }

            fn from_base(base_value: i64) -> i64 {
                base_value * ($factor2 as i64) / ($factor1 as i64)
            }
        }

        // Process remaining conversions recursively
        convert_unit_int! { $($rest)* }
    };

    // Base case: no more conversions to process
    () => {};
}

// 🎩✨ CONVERSION MATRIX MAGIC ✨🎩
// Define conversions from one base unit to all others, then generate all combinations!

/// Generate a single bidirectional transitive conversion pair for i8
#[macro_export]
macro_rules! convert_matrix_generate_pair_i8 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - i8 version)
        impl $crate::unit::FromUnit<$from, i8> for $to {
            fn to_base(value: i8) -> i8 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, i8>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, i8>>::to_base(base_value)
            }

            fn from_base(base_value: i8) -> i8 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, i8>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, i8>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - i8 version)
        impl $crate::unit::FromUnit<$to, i8> for $from {
            fn to_base(value: i8) -> i8 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, i8>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, i8>>::to_base(base_value)
            }

            fn from_base(base_value: i8) -> i8 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, i8>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, i8>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for i16
#[macro_export]
macro_rules! convert_matrix_generate_pair_i16 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - i16 version)
        impl $crate::unit::FromUnit<$from, i16> for $to {
            fn to_base(value: i16) -> i16 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, i16>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, i16>>::to_base(base_value)
            }

            fn from_base(base_value: i16) -> i16 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, i16>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, i16>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - i16 version)
        impl $crate::unit::FromUnit<$to, i16> for $from {
            fn to_base(value: i16) -> i16 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, i16>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, i16>>::to_base(base_value)
            }

            fn from_base(base_value: i16) -> i16 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, i16>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, i16>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for i32
#[macro_export]
macro_rules! convert_matrix_generate_pair_i32 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - i32 version)
        impl $crate::unit::FromUnit<$from, i32> for $to {
            fn to_base(value: i32) -> i32 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, i32>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, i32>>::to_base(base_value)
            }

            fn from_base(base_value: i32) -> i32 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, i32>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, i32>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - i32 version)
        impl $crate::unit::FromUnit<$to, i32> for $from {
            fn to_base(value: i32) -> i32 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, i32>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, i32>>::to_base(base_value)
            }

            fn from_base(base_value: i32) -> i32 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, i32>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, i32>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for i64
#[macro_export]
macro_rules! convert_matrix_generate_pair_i64 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - i64 version)
        impl $crate::unit::FromUnit<$from, i64> for $to {
            fn to_base(value: i64) -> i64 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, i64>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, i64>>::to_base(base_value)
            }

            fn from_base(base_value: i64) -> i64 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, i64>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, i64>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - i64 version)
        impl $crate::unit::FromUnit<$to, i64> for $from {
            fn to_base(value: i64) -> i64 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, i64>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, i64>>::to_base(base_value)
            }

            fn from_base(base_value: i64) -> i64 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, i64>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, i64>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for i128
#[macro_export]
macro_rules! convert_matrix_generate_pair_i128 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - i128 version)
        impl $crate::unit::FromUnit<$from, i128> for $to {
            fn to_base(value: i128) -> i128 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, i128>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, i128>>::to_base(base_value)
            }

            fn from_base(base_value: i128) -> i128 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, i128>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, i128>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - i128 version)
        impl $crate::unit::FromUnit<$to, i128> for $from {
            fn to_base(value: i128) -> i128 {
                // Chain: $to -> $base -> $from
                let base_value =
                    <$base as $crate::unit::FromUnit<$to, i128>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, i128>>::to_base(base_value)
            }

            fn from_base(base_value: i128) -> i128 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, i128>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, i128>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for u8
#[macro_export]
macro_rules! convert_matrix_generate_pair_u8 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - u8 version)
        impl $crate::unit::FromUnit<$from, u8> for $to {
            fn to_base(value: u8) -> u8 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, u8>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, u8>>::to_base(base_value)
            }

            fn from_base(base_value: u8) -> u8 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, u8>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, u8>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - u8 version)
        impl $crate::unit::FromUnit<$to, u8> for $from {
            fn to_base(value: u8) -> u8 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, u8>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, u8>>::to_base(base_value)
            }

            fn from_base(base_value: u8) -> u8 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, u8>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, u8>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for u16
#[macro_export]
macro_rules! convert_matrix_generate_pair_u16 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - u16 version)
        impl $crate::unit::FromUnit<$from, u16> for $to {
            fn to_base(value: u16) -> u16 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, u16>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, u16>>::to_base(base_value)
            }

            fn from_base(base_value: u16) -> u16 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, u16>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, u16>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - u16 version)
        impl $crate::unit::FromUnit<$to, u16> for $from {
            fn to_base(value: u16) -> u16 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, u16>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, u16>>::to_base(base_value)
            }

            fn from_base(base_value: u16) -> u16 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, u16>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, u16>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for u32
#[macro_export]
macro_rules! convert_matrix_generate_pair_u32 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - u32 version)
        impl $crate::unit::FromUnit<$from, u32> for $to {
            fn to_base(value: u32) -> u32 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, u32>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, u32>>::to_base(base_value)
            }

            fn from_base(base_value: u32) -> u32 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, u32>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, u32>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - u32 version)
        impl $crate::unit::FromUnit<$to, u32> for $from {
            fn to_base(value: u32) -> u32 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, u32>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, u32>>::to_base(base_value)
            }

            fn from_base(base_value: u32) -> u32 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, u32>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, u32>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for u64
#[macro_export]
macro_rules! convert_matrix_generate_pair_u64 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - u64 version)
        impl $crate::unit::FromUnit<$from, u64> for $to {
            fn to_base(value: u64) -> u64 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, u64>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, u64>>::to_base(base_value)
            }

            fn from_base(base_value: u64) -> u64 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, u64>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, u64>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - u64 version)
        impl $crate::unit::FromUnit<$to, u64> for $from {
            fn to_base(value: u64) -> u64 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, u64>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, u64>>::to_base(base_value)
            }

            fn from_base(base_value: u64) -> u64 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, u64>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, u64>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for u128
#[macro_export]
macro_rules! convert_matrix_generate_pair_u128 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - u128 version)
        impl $crate::unit::FromUnit<$from, u128> for $to {
            fn to_base(value: u128) -> u128 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, u128>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, u128>>::to_base(base_value)
            }

            fn from_base(base_value: u128) -> u128 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, u128>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, u128>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - u128 version)
        impl $crate::unit::FromUnit<$to, u128> for $from {
            fn to_base(value: u128) -> u128 {
                // Chain: $to -> $base -> $from
                let base_value =
                    <$base as $crate::unit::FromUnit<$to, u128>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, u128>>::to_base(base_value)
            }

            fn from_base(base_value: u128) -> u128 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, u128>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, u128>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for f32
#[macro_export]
macro_rules! convert_matrix_generate_pair_f32 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - f32 version)
        impl $crate::unit::FromUnit<$from, f32> for $to {
            fn to_base(value: f32) -> f32 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, f32>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, f32>>::to_base(base_value)
            }

            fn from_base(base_value: f32) -> f32 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, f32>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, f32>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - f32 version)
        impl $crate::unit::FromUnit<$to, f32> for $from {
            fn to_base(value: f32) -> f32 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, f32>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, f32>>::to_base(base_value)
            }

            fn from_base(base_value: f32) -> f32 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, f32>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, f32>>::from_base(base_intermediate)
            }
        }
    };
}

/// Generate a single bidirectional transitive conversion pair for f64
#[macro_export]
macro_rules! convert_matrix_generate_pair_f64 {
    ($from:ty, $base:ty, $to:ty) => {
        // Generate $from -> $to via $base (transitive conversion - f64 generic version)
        impl $crate::unit::FromUnit<$from, f64> for $to {
            fn to_base(value: f64) -> f64 {
                // Chain: $from -> $base -> $to
                let base_value =
                    <$base as $crate::unit::FromUnit<$from, f64>>::to_base(value);
                <$to as $crate::unit::FromUnit<$base, f64>>::to_base(base_value)
            }

            fn from_base(base_value: f64) -> f64 {
                // Chain: $to -> $base -> $from
                let base_intermediate =
                    <$to as $crate::unit::FromUnit<$base, f64>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$from, f64>>::from_base(base_intermediate)
            }
        }

        // Generate $to -> $from via $base (reverse transitive conversion - f64 generic version)
        impl $crate::unit::FromUnit<$to, f64> for $from {
            fn to_base(value: f64) -> f64 {
                // Chain: $to -> $base -> $from
                let base_value = <$base as $crate::unit::FromUnit<$to, f64>>::to_base(value);
                <$from as $crate::unit::FromUnit<$base, f64>>::to_base(base_value)
            }

            fn from_base(base_value: f64) -> f64 {
                // Chain: $from -> $base -> $to
                let base_intermediate =
                    <$from as $crate::unit::FromUnit<$base, f64>>::from_base(base_value);
                <$base as $crate::unit::FromUnit<$to, f64>>::from_base(base_intermediate)
            }
        }
    };
}

/// Helper macro to generate all pairwise transitive conversions for i8
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_i8 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_i8!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_i8!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for i16
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_i16 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_i16!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_i16!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for i32
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_i32 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_i32!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_i32!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for i64
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_i64 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_i64!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_i64!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for i128
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_i128 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_i128!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_i128!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for u8
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_u8 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_u8!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_u8!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for u16
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_u16 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_u16!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_u16!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for u32
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_u32 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_u32!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_u32!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for u64
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_u64 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_u64!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_u64!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for u128
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_u128 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_u128!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_u128!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for f32
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_f32 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_f32!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_f32!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Helper macro to generate all pairwise transitive conversions for f64
#[macro_export]
macro_rules! convert_matrix_generate_all_pairs_f64 {
    ($base:ty; $first:ty $(, $rest:ty)*) => {
        // Generate conversions from $first to all remaining targets
        $(
            convert_matrix_generate_pair_f64!($first, $base, $rest);
        )*

        // Recursively process remaining targets (but don't include $first again)
        convert_matrix_generate_all_pairs_f64!($base; $($rest),*);
    };

    // Base case: only one or no targets left
    ($base:ty; $single:ty) => {};
    ($base:ty;) => {};
}

/// Conversion matrix macro for signed integers - generates all possible unit conversions from a base unit
#[macro_export]
macro_rules! convert_matrix_signed {
    ($base:ty => $($target:ty),* $(,)?) => {
        // Generate all transitive conversions: Target1 ↔ Target2 via Base
        convert_matrix_generate_all_pairs_i8!($base; $($target),*);
        convert_matrix_generate_all_pairs_i16!($base; $($target),*);
        convert_matrix_generate_all_pairs_i32!($base; $($target),*);
        convert_matrix_generate_all_pairs_i64!($base; $($target),*);
        convert_matrix_generate_all_pairs_i128!($base; $($target),*);
    };
}

/// Conversion matrix macro for unsigned integers - generates all possible unit conversions from a base unit
#[macro_export]
macro_rules! convert_matrix_unsigned {
    ($base:ty => $($target:ty),* $(,)?) => {
        // Generate all transitive conversions: Target1 ↔ Target2 via Base
        convert_matrix_generate_all_pairs_u8!($base; $($target),*);
        convert_matrix_generate_all_pairs_u16!($base; $($target),*);
        convert_matrix_generate_all_pairs_u32!($base; $($target),*);
        convert_matrix_generate_all_pairs_u64!($base; $($target),*);
        convert_matrix_generate_all_pairs_u128!($base; $($target),*);
    };
}

/// Conversion matrix macro for floating-point types - generates all possible unit conversions from a base unit
#[macro_export]
macro_rules! convert_matrix_float {
    ($base:ty => $($target:ty),* $(,)?) => {
        // Generate all transitive conversions: Target1 ↔ Target2 via Base
        convert_matrix_generate_all_pairs_f32!($base; $($target),*);
        convert_matrix_generate_all_pairs_f64!($base; $($target),*);
    };
}

/// Conversion matrix macro - generates all possible unit conversions from a base unit
///
/// This macro takes a base unit and a list of target units, then automatically
/// generates all possible transitive conversions between every pair via the base unit.
/// You must define the direct base ↔ target conversions using `convert_unit!` first.
///
/// This is the top-level macro that calls the floating-point matrix generation.
///
/// # Syntax
/// ```rust,no_run
/// use num_units::convert_matrix;
/// 
/// convert_matrix! {
///     BaseUnit => TargetUnit1, TargetUnit2, TargetUnit3
/// }
/// ```
///
/// # Requirements
/// Before using the matrix, you must define direct conversions using `convert_unit!`:
/// ```rust,no_run
/// convert_unit! {
///     TargetUnit1: |base| conversion_expr;
///     BaseUnit: |target1| reverse_conversion_expr;
/// }
/// ```
///
/// # Generated Conversions
/// The matrix automatically generates transitive conversions:
/// - TargetUnit1 ↔ TargetUnit2 (via BaseUnit)
/// - TargetUnit1 ↔ TargetUnit3 (via BaseUnit)
/// - TargetUnit2 ↔ TargetUnit3 (via BaseUnit)
///
/// This generates conversions for f32 and f64 types by default.
/// Use `convert_matrix_signed!`, `convert_matrix_unsigned!`, etc. for specific type sets.
///
/// # Example
/// ```rust,no_run
/// // First define direct conversions
/// convert_unit! {
///     Revolution: |unitless| unitless;
///     Unitless: |revolution| revolution;
/// }
/// convert_unit! {
///     Radian: |unitless| unitless * 2.0 * PI;
///     Unitless: |radian| radian / (2.0 * PI);
/// }
///
/// // Then generate all transitive conversions
/// convert_matrix! {
///     Unitless => Revolution, Radian, Degree
/// }
/// ```
/// This generates Revolution ↔ Radian, Revolution ↔ Degree, Radian ↔ Degree automatically!
#[macro_export]
macro_rules! convert_matrix {
    ($base:ty => $($target:ty),* $(,)?) => {
        convert_matrix_float! { $base => $($target),* }
    };
}
