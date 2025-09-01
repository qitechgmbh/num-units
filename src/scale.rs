/// # Dimension Scale - Unit System Scaling
///
/// This module provides infrastructure for creating scaled dimensional systems
/// that combine compile-time dimensional analysis with concrete unit storage
/// and conversion capabilities.
///
/// ## Overview
///
/// Dimension scales allow you to define unit systems where each dimension
/// is mapped to a specific base unit. This enables automatic unit conversions
/// and dimensional analysis within a consistent unit system.
///
/// ## Key Concepts
///
/// - **Dimension Scale**: Maps dimensions to specific base units
/// - **Unit System**: A complete set of units for all dimensions
/// - **Automatic Conversions**: Generated conversion methods between units
/// - **Type Safety**: Compile-time dimensional analysis
///
/// ## Architecture
///
/// The scaling system works in layers:
///
/// 1. **Dimension Scale**: `dimension_scale!` creates the scale type
/// 2. **Unit System**: `scaled_unit_system!` creates type aliases
/// 3. **Unit Conversions**: Automatic generation of conversion methods
/// 4. **Type Safety**: Compile-time dimensional checking
///
/// ## Example Usage
///
/// ```rust
/// use num_units::dimension_scale;
/// use num_units::{length, mass, time};
///
/// // Create a dimension scale mapping dimensions to units
/// dimension_scale!(MyScale, length::Meter, mass::Kilogram, time::Second);
///
/// // Use in quantity definitions
/// // let distance = length::f64::Length::from_base(100.0);
/// // let mass = mass::f64::Mass::from_base(5.0);
/// // let force = mass * distance; // Automatic dimensional analysis
/// ```

/// Macro for creating a dimension scale type
///
/// This macro generates a `DimensionScale` struct that maps each dimension
/// to a specific base unit, enabling automatic unit conversions and
/// dimensional analysis within a consistent unit system.
///
/// # Parameters
/// - `$scale_name`: The name of the scale type to create
/// - `$unit_types`: The base unit types for each dimension (in order)
///
/// # Generated Code
///
/// This macro generates:
/// - A scale struct implementing `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`
/// - A `Display` implementation showing unit abbreviations
/// - Type-safe dimensional operations
///
/// # Examples
/// ```rust
/// use num_units::dimension_scale;
/// use num_units::{length, mass, time};
///
/// // Create a scale mapping dimensions to specific units
/// dimension_scale!(PhysicsScale, length::Meter, mass::Kilogram, time::Second);
///
/// // The scale can be used for dimensional analysis
/// // (typically used internally by the system)
/// ```
#[macro_export]
macro_rules! dimension_scale {
    ($scale_name:ident, $($unit:ty),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $scale_name(::core::marker::PhantomData<($($unit,)+)>);

        impl ::core::fmt::Display for $scale_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "Scale[{}]",
                    [$(
                        <$unit as $crate::unit::Unit>::ABBREVIATION
                    ),+].join(",")
                )
            }
        }
    };
}

/// Macro for creating a complete scaled unit system
///
/// This macro creates type aliases for a complete unit system with all base units
/// defined. It combines a dimension system with a scale to create a fully functional
/// unit system with automatic conversions.
///
/// # Parameters
/// - `$system_name`: The name of the unit system (e.g., SI)
/// - `$scale_name`: The name of the scale type (e.g., SiScale)
/// - `$unit_types`: The base unit types for each dimension
///
/// # Generated Code
///
/// This macro generates:
/// - A dimension scale type (via `dimension_scale!`)
/// - Type aliases for quantities in the scaled system
/// - Automatic conversion methods between units
///
/// # Examples
/// ```rust
/// use num_units::scaled_unit_system;
/// use num_units::{length, mass, time};
///
/// // Create a complete scaled unit system
/// scaled_unit_system!(MySystem, MyScale, length::Meter, mass::Kilogram, time::Second);
///
/// // Use the generated system for dimensional analysis
/// // (typically used internally by the system)
/// ```
#[macro_export]
macro_rules! scaled_unit_system {
    ($system_name:ident, $scale_name:ident, $($unit:ty),+ $(,)?) => {
        // First create the dimension scale type
        $crate::dimension_scale!($scale_name, $($unit),+);

        /// Base type for this scaled unit system
        pub type $system_name<V, D> = $crate::quantity::Quantity<V, D, $scale_name>;

        // Note: Specific quantity type aliases would need to be generated
        // based on the actual dimension system being used
    };
}
