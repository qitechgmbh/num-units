/// Unit trait for defining units with linear conversions to base units
///
/// Units can have both a multiplication factor and an additive offset,
/// allowing for conversions like temperature scales (Celsius, Fahrenheit).
///
/// The conversion formula is: base_value = factor * unit_value + offset
///
/// # Examples
/// - Meter: factor = 1.0, offset = 0.0 (base unit for length)
/// - Foot: factor = 0.3048, offset = 0.0 (1 foot = 0.3048 meters)  
/// - Celsius: factor = 1.0, offset = 273.15 (°C to Kelvin)
/// - Fahrenheit: factor = 5.0/9.0, offset = 255.372222... (°F to Kelvin)
pub trait Unit {
    /// The dimension this unit measures (Length, Mass, Temperature, etc.)
    type Dimension;

    /// Multiplication factor to convert to base unit
    const FACTOR: f64;

    /// Additive offset to convert to base unit  
    const OFFSET: f64;

    /// Convert a value in this unit to the base unit
    /// base_value = FACTOR * unit_value + OFFSET
    fn to_base<T>(value: T) -> T
    where
        T: num_traits::Num + Copy + From<f64>,
    {
        T::from(Self::FACTOR) * value + T::from(Self::OFFSET)
    }

    /// Convert a value from the base unit to this unit
    /// unit_value = (base_value - OFFSET) / FACTOR
    fn from_base<T>(base_value: T) -> T
    where
        T: num_traits::Num + Copy + From<f64>,
    {
        (base_value - T::from(Self::OFFSET)) / T::from(Self::FACTOR)
    }
}

/// Macro for defining individual units with detailed parameters
///
/// # Syntax
/// ```ignore
/// unit!(UnitStruct, method_name, factor, offset, "Description", "abbreviation", DimensionType);
/// ```
///
/// # Parameters
/// - `UnitStruct`: Singular struct name (e.g., `Meter`, `Foot`)
/// - `method_name`: Lowercase slug for method names (e.g., `meters`, `feet`)
/// - `factor`: Multiplication factor to convert to base unit
/// - `offset`: Additive offset to convert to base unit
/// - `description`: Human-readable description
/// - `abbreviation`: Short symbol (e.g., "m", "ft", "°C")
/// - `dimension`: The dimension type this unit measures
///
/// This generates:
/// - A unit struct implementing the Unit trait
/// - `Quantity::from_{method_name}(value)` - create quantity from this unit
/// - `quantity.as_{method_name}()` - get value in this unit
///
/// # Examples
/// ```ignore
/// # use num_units::unit;
/// # use num_units::si::{Length, ThermodynamicTemperature};
/// unit!(Meter, meters, 1.0, 0.0, "Meter", "m", Length);
/// unit!(Foot, feet, 0.3048, 0.0, "Foot", "ft", Length);
/// unit!(Celsius, celsius, 1.0, 273.15, "Celsius", "°C", ThermodynamicTemperature);
/// ```
#[macro_export]
macro_rules! unit {
    ($unit_name:ident, $method_name:ident, $factor:expr, $offset:expr, $description:expr, $abbreviation:expr, $dimension:ty) => {
        #[doc = concat!($description, " (", $abbreviation, ")")]
        pub struct $unit_name;

        impl $crate::unit::Unit for $unit_name {
            type Dimension = $dimension;
            const FACTOR: f64 = $factor;
            const OFFSET: f64 = $offset;
        }

        paste::paste! {
            impl<V> $crate::quantity::Quantity<V, $dimension>
            where
                V: num_traits::Num + Copy + From<f64>,
            {
                #[doc = concat!(" Create a quantity from a value in ", $description, " (", $abbreviation, ")")]
                pub fn [<from_ $method_name>](value: V) -> Self {
                    if $offset == 0.0 {
                        Self::new(V::from($factor) * value)
                    } else {
                        Self::new(V::from($factor) * value + V::from($offset))
                    }
                }

                #[doc = concat!(" Get the value of this quantity in ", $description, " (", $abbreviation, ")")]
                pub fn [<as_ $method_name>](&self) -> V {
                    (self.value - V::from($offset)) / V::from($factor)
                }
            }
        }

        impl core::fmt::Display for $unit_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", $abbreviation)
            }
        }
    };
}
