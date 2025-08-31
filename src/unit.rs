/// Unit trait for defining units with their dimension and basic properties
///
/// This trait is used for unit identification and dimension tracking.
/// Actual conversions are handled by the closure-based FromBaseUnit trait.
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
