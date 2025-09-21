/// Shared utilities for comparing num-units with UOM library results
///
/// This module provides common macros and helper functions for testing that
/// our unit conversion results match those from the UOM library.

/// Macro to generate a test that compares unit conversion results between num-units and UOM
///
/// This macro creates a test function that:
/// 1. Creates 1 unit of the base unit in both systems
/// 2. Converts to the target unit in both systems  
/// 3. Compares the results using exact equality (assert_eq!)
///
/// # Arguments
/// * `$num_units_module` - The num-units module path (e.g., `crate::si::length`)
/// * `$uom_module` - The UOM module path (e.g., `uom::si::length`)
/// * `$num_units_quantity` - The quantity type in num-units (e.g., `Length`)
/// * `$uom_quantity` - The UOM quantity type (e.g., `Length`)
/// * `$num_units_base` - The base unit type in num-units (e.g., `Meter`)
/// * `$num_units_target` - The target unit type in num-units (e.g., `Kilometer`)
/// * `$uom_base` - The base unit in UOM (e.g., `meter`)
/// * `$uom_target` - The target unit in UOM (e.g., `kilometer`)
#[macro_export]
macro_rules! test_uom_compatibility {
    (
        $num_units_module:path,
        $uom_module:path,
        $num_units_quantity:ident,
        $uom_quantity:ident,
        $num_units_base:ident,
        $num_units_target:ty,
        $uom_base:ident,
        $uom_target:ident
    ) => {
        paste::paste! {
            #[test]
            fn [<test_ $uom_target>]() {
                use uom::si::f64::$uom_quantity as [<Uom $uom_quantity>];
                use $num_units_module::*;
                use $uom_module as uom_units;

                // Create 1 base unit in both systems
                let num_units_base = $num_units_quantity::from::<$num_units_base>(1.0);
                let uom_base = [<Uom $uom_quantity>]::new::<uom_units::$uom_base>(1.0);

                // Convert to target unit
                let num_units_result = num_units_base.to::<$num_units_target>();
                let uom_result = uom_base.get::<uom_units::$uom_target>();

                // Compare results with exact equality
                assert_eq!(
                    num_units_result, uom_result,
                    "Conversion mismatch for {}: num-units = {}, UOM = {}",
                    stringify!($uom_target), num_units_result, uom_result
                );
            }
        }
    };
}
