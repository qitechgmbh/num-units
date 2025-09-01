/// Test demonstrating the new generic `from()` and `to()` API
///
/// This shows how the improved API eliminates the need to specify base units
/// while maintaining compile-time dimensional safety.
use num_units::length::{Centimeter, Kilometer, Length, Meter, Micrometer, Millimeter};

#[test]
fn test_generic_api_creation() {
    let distance = Length::from::<Kilometer>(2.5_f32);
    let distance_m = distance.to::<Meter>();
    assert_eq!(distance_m, 2500.0);

    let height = Length::from::<Centimeter>(180.0_f32);
    let height_m = height.to::<Meter>();
    assert_eq!(height_m, 1.8);
}

#[test]
fn test_unit_conversions() {
    let distance = Length::from::<Kilometer>(2.5_f32);

    assert_eq!(distance.to::<Kilometer>(), 2.5);
    assert_eq!(distance.to::<Meter>(), 2500.0);
    assert_eq!(distance.to::<Centimeter>(), 250000.0);
    assert_eq!(distance.to::<Millimeter>(), 2500000.0);
    assert_eq!(distance.to::<Micrometer>(), 2500000000.0);
}

#[test]
fn test_dimensional_safety() {
    let length1 = Length::from::<Kilometer>(1.0);
    let length2 = Length::from::<Centimeter>(50000.0); // 500 m
    let total = length1 + length2; // ✅ Same dimension - works
    assert_eq!(total.to::<Kilometer>(), 1.5);

    // This would be a compile error:
    // let invalid = length + time; // ❌ Would be compile error!
}
