/// Unit trait for defining units with their dimension and basic properties
///
/// This trait is used for unit identification and dimension tracking.
/// Actual conversions are handled by the closure-based FromBaseUnit trait.
pub trait Unit {
    /// The dimension this unit measures (Length, Mass, Temperature, etc.)
    type Dimension;
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
/// # Syntax
/// ```ignore
/// unit! {
///     system: system_path;
///     quantity: quantity_path;
///     @unit_name: coefficient; "abbreviation", "singular", "plural";
///     @unit_with_offset: coefficient, offset; "abbreviation", "singular", "plural";
/// }
/// ```
///
/// # Parameters
/// * `system`: Path to the module where the `system!` macro was run (e.g. `crate::si`)
/// * `quantity`: Path to the module where the `quantity!` macro was run (e.g. `crate::si::length`)
/// * `@unit_name`: Unit identifier (e.g. `@kilometer`, `@foot`)
/// * `coefficient`: Conversion coefficient from the unit to the base unit (e.g. `1.0E-3` for km to m)
/// * `offset`: Optional additive offset (e.g. `273.15` for °C to K)
/// * `"abbreviation"`: Unit abbreviation (e.g. `"km"`)
/// * `"singular"`: Singular unit description (e.g. `"kilometer"`)
/// * `"plural"`: Plural unit description (e.g. `"kilometers"`)
///
/// This generates:
/// - A unit struct implementing the Unit trait
/// - `Quantity::from_{unit_name}(value)` - create quantity from this unit
/// - `quantity.as_{unit_name}()` - get value in this unit
///
/// # Examples
/// ```ignore
/// # use num_units::unit;
/// # use num_units::si::length;
/// unit! {
///     system: crate::si;
///     quantity: crate::si::length;
///     @kilometer: 1000.0; "km", "kilometer", "kilometers";
///     @foot: 0.3048; "ft", "foot", "feet";
///     @celsius: 1.0, 273.15; "°C", "celsius", "celsius";
/// }
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

            impl $crate::unit::Unit for $unit {
                // Access the dimension type from the quantity module
                type Dimension = __quantity_module::Dimension;
                const FACTOR: f64 = $coefficient;
                const OFFSET: f64 = {
                    $($offset ;)?
                    0.0
                };
            }

            // Generate conversion methods for all numeric types
            paste::paste! {
                // f32 conversions
                impl $crate::quantity::Quantity<f32, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<f64, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<i8, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<u8, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<i16, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<u16, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<i32, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<u32, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<i64, __quantity_module::Dimension> {
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
                impl $crate::quantity::Quantity<u64, __quantity_module::Dimension> {
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

/// Macro for defining units with ScaledSystem support
///
/// This macro generates unit structs and conversion methods for ScaledQuantity types.
/// It supports the new dimension tagging syntax: `LENGTH: Meter`, `SCALAR: Radian`, etc.
///
/// # Syntax
/// ```ignore
/// scaled_unit! {
///     system: scaled_system_type;
///     quantity: quantity_module;
///     @unit_name: coefficient; dimension_tag: base_unit, "abbreviation", "singular", "plural";
/// }
/// ```
///
/// # Parameters
/// * `system`: The ScaledSystem type (e.g., `LengthSI`, `AngleRadians`)
/// * `quantity`: The quantity module path
/// * `dimension_tag`: The dimension identifier (e.g., `LENGTH`, `SCALAR`)
/// * `base_unit`: The base unit type (e.g., `Meter`, `Radian`)
///
/// # Examples
/// ```ignore
/// # use num_units::scaled_unit;
/// # use num_units::scaled_system::*;
///
/// scaled_unit! {
///     system: LengthSI;
///     quantity: crate::length;
///     @meter: 1.0; LENGTH: Meter, "m", "meter", "meters";
///     @kilometer: 1000.0; LENGTH: Meter, "km", "kilometer", "kilometers";
/// }
///
/// scaled_unit! {
///     system: AngleRadians;
///     quantity: crate::angle;
///     @radian: 1.0; SCALAR: Radian, "rad", "radian", "radians";
///     @degree: 0.01745329252; SCALAR: Degree, "°", "degree", "degrees";
/// }
/// ```
#[macro_export]
macro_rules! scaled_unit {
    // Unit definition with dimension tagging
    (
        system: $system:ty;
        quantity: $quantity:path;
        $($(#[$unit_attr:meta])* @$unit:ident: $coefficient:expr;
            $dim_tag:ident: $base_unit:ty, $abbreviation:expr, $singular:expr, $plural:expr;)+
    ) => {
        // Import the quantity module
        use $quantity as __quantity_module;

        $(
            $(#[$unit_attr])*
            #[doc = $plural]
            #[allow(non_camel_case_types)]
            pub struct $unit;

            impl $crate::unit::Unit for $unit {
                type Dimension = $system;
                type BaseUnit = $base_unit;
                const FACTOR: f64 = $coefficient;
                const OFFSET: f64 = 0.0;
            }

            // Generate conversion methods for all numeric types
            ::paste::paste! {
                // f32 conversions
                impl $crate::quantity::ScaledQuantity<f32, $system> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: f32) -> Self {
                        Self::from_raw($coefficient as f32 * value)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> f32 {
                        self.value / $coefficient as f32
                    }
                }

                // f64 conversions
                impl $crate::quantity::ScaledQuantity<f64, $system> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: f64) -> Self {
                        Self::from_raw($coefficient * value)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> f64 {
                        self.value / $coefficient
                    }
                }

                // i32 conversions
                impl $crate::quantity::ScaledQuantity<i32, $system> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: i32) -> Self {
                        Self::from_raw(($coefficient * value as f64) as i32)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> i32 {
                        (self.value as f64 / $coefficient) as i32
                    }
                }

                // i64 conversions
                impl $crate::quantity::ScaledQuantity<i64, $system> {
                    #[doc = concat!("Create a quantity from a value in ", stringify!($unit))]
                    pub fn [<from_ $unit:snake>](value: i64) -> Self {
                        Self::from_raw(($coefficient * value as f64) as i64)
                    }

                    #[doc = concat!("Get the value of this quantity in ", stringify!($unit))]
                    pub fn [<as_ $unit:snake>](&self) -> i64 {
                        (self.value as f64 / $coefficient) as i64
                    }
                }
            }
        )+
    };
}
