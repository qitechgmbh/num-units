/// Scaled system for combining dimensional analysis with unit conversions
///
/// This module provides the infrastructure for creating scaled dimensions that
/// combine the compile-time dimensional analysis with concrete unit storage
/// and conversion capabilities.

/// Macro for creating a dimension scale type
///
/// This macro generates a DimensionScale struct that can handle any number
/// of dimensions and corresponding unit types.
///
/// # Parameters
/// - `$scale_name`: The name of the scale type to create
/// - `$unit_types`: The unit types for each dimension
///
/// # Example
/// ```ignore
/// dimension_scale!(SIScale, Meter, Kilogram, Second, Ampere, Kelvin, Mole, Candela);
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
                        <$unit as $crate::unit::Unit>::abbreviation()
                    ),+].join(",")
                )
            }
        }
    };
}

/// Macro for creating a complete scaled unit system
///
/// This macro creates type aliases for a complete unit system with all base units defined.
///
/// # Parameters
/// - `$system_name`: The name of the unit system (e.g., SI)
/// - `$scale_name`: The name of the scale type (e.g., SIScale)
/// - `$unit_types`: The base unit types for each dimension
///
/// # Example
/// ```ignore
/// scaled_unit_system!(SI, SIScale, Meter, Kilogram, Second, Ampere, Kelvin, Mole, Candela);
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
