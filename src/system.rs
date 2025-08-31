/// Unified system for creating complete unit systems with default scales
///
/// This module provides a macro for creating a dimensional system
/// with default unit scales for each dimension.

/// Create a dimensional system with default unit scales
///
/// This macro creates:
/// 1. A dimension system (like ISQ)
/// 2. A dimension scale type with specified base units mapped to dimensions
/// 3. Type aliases for convenient access
///
/// # Syntax
/// ```ignore
/// system! {
///     SystemName,
///     DIMENSION1 => Unit1,
///     DIMENSION2 => Unit2,
///     ...
/// }
/// ```
///
/// # Parameters
/// - `SystemName`: Name of the dimension system (e.g., ISQ)
/// - `DIMENSION => Unit`: Maps each dimension constant to its default unit type
///
/// # Example
/// ```ignore
/// system! {
///     ISQ,
///     L => Meter,
///     M => Kilogram,
///     T => Second,
///     I => Ampere,
///     TH => Kelvin,
///     N => Mole,
///     J => Candela
/// }
/// ```
#[macro_export]
macro_rules! system {
    // Handle the new syntax with dimension => unit mappings
    (
        $system_name:ident,
        $($dim:ident => $unit:ty),+ $(,)?
    ) => {
        // First create the dimension system using the existing system! macro pattern
        #[::num_units_macros::system($($dim),+)]
        pub struct $system_name;

        // Then create the scale type using the new dimension_scale! macro
        ::paste::paste! {
            $crate::dimension_scale!([<$system_name Scale>], $($unit),+);
        }
    };

    // Keep the old syntax for backward compatibility
    ($system_name:ident, $($dim:ident),+ $(,)?) => {
        #[::num_units_macros::system($($dim),+)]
        pub struct $system_name;
    };
}
