/// Tests for angle and scalar unit conversions
use num_units::scalar::{Percent, Radian, Revolution, Scalar, Unitless};

#[test]
fn test_scalar_unit_conversions() {
    let scalar = Scalar::from::<Unitless>(0.75); // base type
    let percent = scalar.to::<Percent>(); // Explicit conversion
    let revolution = scalar.to::<Revolution>(); // Explicit conversion
    let radian = scalar.to::<Radian>(); // Matrix-generated conversion

    assert_eq!(percent, 75.0);
    assert_eq!(revolution, 0.75);
    assert!((radian - 4.71238898038469_f64).abs() < 1e-10); // ~3π/2
}

#[test]
fn test_angle_conversions() {
    let scalar = Scalar::from::<Unitless>(1.0);

    // Test full revolution
    let full_revolution = scalar.to::<Revolution>();
    assert_eq!(full_revolution, 1.0);

    // Test percentage
    let percentage = scalar.to::<Percent>();
    assert_eq!(percentage, 100.0);

    // Test radians (2π for full revolution)
    let radians = scalar.to::<Radian>();
    assert!((radians - 2.0 * std::f64::consts::PI).abs() < 1e-10);
}
