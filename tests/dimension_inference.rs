/// Tests for dimensional analysis and cross-dimensional operations
use num_units::si::{
    area::{Area, SquareMeter},
    length::{Length, Meter},
    scalar::{Scalar, Unitless},
    volume::{CubicMeter, Volume},
};

#[test]
fn test_same_dimension_operations() {
    let l1 = Length::from::<Meter>(3.0);
    let l2 = Length::from::<Meter>(4.0);

    // Same-dimension operations
    let length_sum = l1 + l2; // Length + Length = Length
    assert_eq!(length_sum.to::<Meter>(), 7.0);
}

#[test]
fn test_scalar_operations() {
    let l1 = Length::from::<Meter>(3.0);
    let s1 = Scalar::from::<Unitless>(5.0);

    // Scalar multiplication
    let scaled_length: Length<_> = l1 * s1.to::<Unitless>(); // Length * Scalar = Length
    assert_eq!(scaled_length.to::<Meter>(), 15.0);
}

#[test]
fn test_cross_dimensional_operations() {
    let l1 = Length::from::<Meter>(3.0);
    let l2 = Length::from::<Meter>(4.0);

    // Length × Length = Area
    let area: Area<_> = l1 * l2;
    assert_eq!(area.to::<SquareMeter>(), 12.0);

    // Area × Length = Volume
    let volume: Volume<_> = area * l1; // Area * Length = Volume
    assert_eq!(volume.to::<CubicMeter>(), 36.0);
}
