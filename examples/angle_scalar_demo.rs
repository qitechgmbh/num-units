use num_units::si::scalar::{Percent, Radian, Revolution, Unitless, scalar::f64::Scalar};

fn main() {
    let scalar = Scalar::from::<Unitless>(0.75); // base type
    let percent = scalar.to::<Percent>(); // Explicit conversion
    let revolution = scalar.to::<Revolution>(); // Explicit conversion
    let radian = scalar.to::<Radian>(); // Matrix-generated conversion

    assert_eq!(percent, 75.0);
    println!("0.75 unitless is {}%", percent);
    assert_eq!(revolution, 0.75);
    println!("0.75 unitless is {} revolutions", revolution);
    assert!((radian - 4.71238898038469).abs() < 1e-10); // ~3π/2
    println!("0.75 unitless is {} radians", radian);
}
