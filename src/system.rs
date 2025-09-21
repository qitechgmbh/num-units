/// # System - Dimensional System Creation
///
/// This module provides macros for creating complete dimensional systems
/// with automatic unit scaling and conversion capabilities.
///
/// ## Overview
///
/// Dimensional systems define the fundamental dimensions of a physical system
/// (like length, mass, time) and map them to specific base units. This enables
/// type-safe dimensional analysis and automatic unit conversions.
///
/// ## Key Features
///
/// - **Type-Safe Dimensions**: Compile-time dimensional analysis
/// - **Automatic Scaling**: Unit conversions based on dimension mappings
/// - **Extensible**: Support for any dimensional system
/// - **SI Compatible**: Full support for SI unit system
///
/// ## Architecture
///
/// The system creation works in two phases:
///
/// 1. **Dimension Definition**: Define fundamental dimensions using typenum
/// 2. **Unit Mapping**: Map dimensions to specific base units
/// 3. **Scale Generation**: Create conversion relationships
/// 4. **Type Safety**: Enable compile-time dimensional checking
///
/// ## Example Usage
///
/// ```rust
/// use num_units::system;
/// use num_units::{length, mass, time};
///
/// // Create a dimensional system with unit mappings
/// system! {
///     Physics,
///     PhysicsScale,
///     L => length::Meter,      // Length dimension -> Meter unit
///     M => mass::Kilogram,     // Mass dimension -> Kilogram unit
///     T => time::Second        // Time dimension -> Second unit
/// }
///
/// // Use for dimensional analysis
/// // let distance = length::Length::from_base(100.0);
/// // let time_val = time::Time::from_base(10.0);
/// // let velocity = distance / time_val; // Automatic dimensional analysis
/// ```

/// Create a dimensional system with default unit scales
///
/// This macro creates a complete dimensional system by combining dimension
/// definitions with unit mappings. It generates the necessary types and
/// conversion relationships for a fully functional unit system.
///
/// # Syntax
/// ```ignore
/// use num_units::system;
///
/// system! {
///     SystemName,
///     SystemScale,
///     DIMENSION1 => UnitType1,
///     DIMENSION2 => UnitType2,
/// }
/// ```
///
/// # Parameters
/// - `SystemName`: Name of the dimension system (e.g., SI, Physics)
/// - `DIMENSION => Unit`: Maps each dimension constant to its base unit type
///
/// # Generated Code
///
/// This macro generates:
/// - A dimension system struct with dimensional operations
/// - A dimension scale mapping dimensions to units
/// - Type aliases for convenient access
/// - Automatic conversion methods between units
///
/// # Examples
/// ```rust,no_run
/// use num_units::system;
///
/// // Create SI system with standard unit mappings
/// // Note: This is just an example of the syntax
/// // system! {
/// //     ISQ,
/// //     SiScale,
/// //     L => length::Meter,
/// //     M => mass::Kilogram,
/// //     T => time::Second,
/// // }
/// ```
/// ```
#[macro_export]
macro_rules! system {
    // Handle the new syntax with dimension => unit mappings
    (
        $system_name:ident,
        $scale_name:ident,
        $($dim:ident => $unit:ty),+ $(,)?
    ) => {
        // First create the dimension system using the existing system! macro pattern
        #[::num_units_macros::system($($dim),+)]
        pub struct $system_name;

        // Then create the scale type using the new dimension_scale! macro
        ::paste::paste! {
            $crate::dimension_scale!([<$scale_name>], $($unit),+);
        }
    };
}
