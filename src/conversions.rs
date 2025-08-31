/// Comprehensive conversion system for units and quantities
///
/// This module provides a unified trait-based conversion system that works
/// for both base units and derived units, using the FromBaseUnit/IntoBaseUnit
/// approach as the foundation.
use num_traits::Num;
/// Trait for units that can be converted to/from base units
///
/// This trait extends the Unit trait with conversion capabilities
/// using the FromBaseUnit/IntoBaseUnit system.
pub trait ConvertibleUnit: crate::base_units::Unit {
    /// The base unit for this unit's dimension
    type BaseUnit: crate::base_units::Unit;

    /// Convert a value in this unit to its base unit
    fn to_base_unit<V>(value: V) -> V
    where
        V: Num + Copy + From<f64> + num_traits::NumCast;

    /// Convert a value from this unit's base unit to this unit
    fn from_base_unit<V>(base_value: V) -> V
    where
        V: Num + Copy + From<f64> + num_traits::NumCast;
}
