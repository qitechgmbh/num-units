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

/// Helper macro to define unit structs
#[macro_export]
#[doc(hidden)]
macro_rules! unit_unit {
    ($(#[$unit_attr:meta])+ @$unit:ident $plural:expr) => {
        $(#[$unit_attr])*
        #[doc = $plural]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
    (@$unit:ident $plural:expr) => {
        #[doc = $plural]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
}

/// Helper macro to handle unit constants
#[macro_export]
#[doc(hidden)]
macro_rules! unit_constant {
    ($const:expr) => {
        $const
    };
    () => {
        0.0
    };
}

/// Macro for defining individual units with detailed parameters
///
/// This macro generates unit structs with automatic conversion methods for all
/// supported numeric types (i8, u8, i16, u16, i32, u32, i64, u64, f32, f64).
///
/// # Generated Methods
///
/// For each unit defined with this macro, the following methods are generated:
/// - `from_{unit_name}(value)` - Create quantity from this unit
/// - `as_{unit_name}()` - Get value in this unit
///
/// # Examples
///
/// ```rust
/// use num_units::unit;
/// use num_units::si::length;
///
/// // Define a custom unit (this would typically be done in the si/ modules)
/// // unit! {
/// //     system: crate::si;
/// //     quantity: crate::si::length;
/// //     @kilometer: 1000.0; "km", "kilometer", "kilometers";
/// //     @foot: 0.3048; "ft", "foot", "feet";
/// // }
///
/// // Usage (using existing SI units):
/// let dist = length::f64::Length::from_meter(1000.0);
/// let km = dist.as_kilometer(); // Returns 1.0
/// ```
#[macro_export]
macro_rules! unit {
    // Unit definition syntax with system and quantity paths
    (
        system: $system:path;
        quantity: $quantity:path;

        $($(#[$unit_attr:meta])* @$unit:ident: $coefficient:expr $(, $offset:expr)?;
            $abbreviation:expr, $singular:expr, $plural:expr;)+
    ) => {
        // Import the quantity module
        use $quantity as __quantity_module;

        $(
            $(#[$unit_attr])*
            #[doc = $plural]
            #[allow(non_camel_case_types)]
            pub struct $unit;

            // Generate conversion methods for all numeric types
            paste::paste! {
                // f32 conversions
                impl $crate::quantity::Quantity<f32, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: f32) -> Self {
                        Self::from_raw($coefficient as f32 * value $(+ $offset as f32)?)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> f32 {
                        (self.value $( - $offset as f32)?) / $coefficient as f32
                    }
                }

                // f64 conversions
                impl $crate::quantity::Quantity<f64, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: f64) -> Self {
                        Self::from_raw($coefficient * value $(+ $offset)?)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> f64 {
                        (self.value $( - $offset)?) / $coefficient
                    }
                }

                // i8 conversions
                impl $crate::quantity::Quantity<i8, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: i8) -> Self {
                        Self::from_raw(($coefficient as f64 * value as f64 $(+ $offset)?) as i8)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> i8 {
                        ((self.value as f64 $( - $offset)?) / $coefficient) as i8
                    }
                }

                // u8 conversions
                impl $crate::quantity::Quantity<u8, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: u8) -> Self {
                        Self::from_raw(($coefficient as f64 * value as f64 $(+ $offset)?) as u8)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> u8 {
                        ((self.value as f64 $( - $offset)?) / $coefficient) as u8
                    }
                }

                // i16 conversions
                impl $crate::quantity::Quantity<i16, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: i16) -> Self {
                        Self::from_raw(($coefficient as f64 * value as f64 $(+ $offset)?) as i16)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> i16 {
                        ((self.value as f64 $( - $offset)?) / $coefficient) as i16
                    }
                }

                // u16 conversions
                impl $crate::quantity::Quantity<u16, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: u16) -> Self {
                        Self::from_raw(($coefficient as f64 * value as f64 $(+ $offset)?) as u16)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> u16 {
                        ((self.value as f64 $( - $offset)?) / $coefficient) as u16
                    }
                }

                // i32 conversions
                impl $crate::quantity::Quantity<i32, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: i32) -> Self {
                        Self::from_raw(($coefficient as f64 * value as f64 $(+ $offset)?) as i32)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> i32 {
                        ((self.value as f64 $( - $offset)?) / $coefficient) as i32
                    }
                }

                // u32 conversions
                impl $crate::quantity::Quantity<u32, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: u32) -> Self {
                        Self::from_raw(($coefficient as f64 * value as f64 $(+ $offset)?) as u32)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> u32 {
                        ((self.value as f64 $( - $offset)?) / $coefficient) as u32
                    }
                }

                // i64 conversions
                impl $crate::quantity::Quantity<i64, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: i64) -> Self {
                        Self::from_raw(($coefficient as f64 * value as f64 $(+ $offset)?) as i64)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> i64 {
                        ((self.value as f64 $( - $offset)?) / $coefficient) as i64
                    }
                }

                // u64 conversions
                impl $crate::quantity::Quantity<u64, __quantity_module::Dimension, __quantity_module::Scale> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: u64) -> Self {
                        Self::from_raw(($coefficient as f64 * value as f64 $(+ $offset)?) as u64)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> u64 {
                        ((self.value as f64 $( - $offset)?) / $coefficient) as u64
                    }
                }
            }
        )+
    };
}
