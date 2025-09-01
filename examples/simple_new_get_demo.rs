/// Example demonstrating the new generic `from()` and `to()` API
///
/// This shows how the improved API eliminates the need to specify base units
/// while maintaining compile-time dimensional safety.
use num_units::length::Length;
use num_units::si::length::{Centimeter, Kilometer, Meter, Micrometer, Millimeter};

fn main() {
    println!("=== New Generic API Demo ===\n");

    println!("--- Creating Quantities ---");
    let distance = Length::from::<Kilometer>(2.5_f32);
    let distance_m = distance.to::<Meter>();
    println!("Length::from::<Kilometer>(2.5) → {} m", distance_m);

    let height = Length::from::<Centimeter>(180.0_f32);
    let height_m = height.to::<Meter>();
    println!("Length::from::<Centimeter>(180.0) → {} m", height_m);
    println!();

    println!("--- Converting Units ---");
    println!("distance (2.5 km) in different units:");
    println!("  {} km", distance.to::<Kilometer>());
    println!("  {} m ", distance.to::<Meter>());
    println!("  {} cm", distance.to::<Centimeter>());
    println!("  {} mm", distance.to::<Millimeter>());
    println!("  {} μm", distance.to::<Micrometer>());
    println!();

    println!("--- Dimensional Safety ---");
    let length1 = Length::from::<Kilometer>(1.0);
    let length2 = Length::from::<Centimeter>(50000.0); // 500 m
    let total = length1 + length2; // ✅ Same dimension - works
    println!("1 km + 500 m = {} km", total.to::<Kilometer>());

    // let invalid = length + time; // ❌ Would be compile error!
}
