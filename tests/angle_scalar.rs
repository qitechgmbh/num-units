use num_units::si::{
    angle::{Radian, Revolution},
    ratio::Percent,
    scalar::{Scalar, Unitless},
};

/// Tests for angle and scalar unit conversions

#[test]
fn test_scalar_unit_conversions() {
    let unitless = Scalar::from::<Unitless>(0.75); // base type
    let percent = unitless.to::<Percent>(); // Explicit conversion
    let revolution = unitless.to::<Revolution>(); // Explicit conversion
    let radian = unitless.to::<Radian>(); // Matrix-generated conversion

    assert_eq!(percent, 75.0);
    assert_eq!(revolution, 0.75);
    assert!((radian - 4.71238898038469_f64).abs() < 1e-10); // ~3π/2
}

#[test]
fn test_angle_conversions() {
    let unitless = Scalar::from::<Unitless>(1.0);

    // Test full revolution
    let full_revolution = unitless.to::<Revolution>();
    assert_eq!(full_revolution, 1.0);

    // Test percentage
    let percentage = unitless.to::<Percent>();
    assert_eq!(percentage, 100.0);

    // Test radians (2π for full revolution)
    let radians = unitless.to::<Radian>();
    assert!((radians - 2.0 * std::f32::consts::PI).abs() < 1e-10);
}
