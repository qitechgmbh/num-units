/// Tests for dimensional analysis and cross-dimensional operations
use num_units::{
    area::{SquareMeter, area},
    length::{Meter, length},
    scalar::{Unitless, scalar},
    volume::{CubicMeter, volume},
};

#[test]
fn test_same_dimension_operations() {
    let l1 = length::f64::Length::from::<Meter>(3.0);
    let l2 = length::f64::Length::from::<Meter>(4.0);

    // Same-dimension operations
    let length_sum = l1 + l2; // Length + Length = Length
    assert_eq!(length_sum.to::<Meter>(), 7.0);
}

#[test]
fn test_scalar_operations() {
    let l1 = length::f64::Length::from::<Meter>(3.0);
    let s1 = scalar::f64::Scalar::from::<Unitless>(5.0);

    // Scalar multiplication
    let scaled_length: length::f64::Length = l1 * s1.to::<Unitless>(); // Length * Scalar = Length
    assert_eq!(scaled_length.to::<Meter>(), 15.0);
}

#[test]
fn test_cross_dimensional_operations() {
    let l1 = length::f64::Length::from::<Meter>(3.0);
    let l2 = length::f64::Length::from::<Meter>(4.0);
    let s1 = scalar::f64::Scalar::from::<Unitless>(5.0);

    // Length × Length = Area
    let area: area::f64::Area = l1 * l2;
    assert_eq!(area.to::<SquareMeter>(), 12.0);

    // Area × Length = Volume
    let volume: volume::f64::Volume = area * l1; // Area * Length = Volume
    assert_eq!(volume.to::<CubicMeter>(), 36.0);

    // Volume ÷ Time = Volumetric Flow Rate (still Volume since time is scalar)
    let volumetric_flow = volume / s1.to::<Unitless>(); // Volume / Time = Volume/Time
    assert_eq!(volumetric_flow.to::<CubicMeter>(), 7.2);
}
