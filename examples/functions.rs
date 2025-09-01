//! Demonstration of generic functions with SI quantities

use num_traits::Num;
use num_units::si::{
    length::{length, Meter, Kilometer},
    mass::{mass, Kilogram, Gram},
};

// Generic function that works with any numeric type
fn calculate_distance<V>(speed: V, time: V) -> V
where
    V: Num + Copy,
{
    speed * time
}

// Function working with length quantities
fn add_lengths<V>(length1: length::Length<V>, length2: length::Length<V>) -> length::Length<V>
where
    V: Num,
{
    length1 + length2
}

// Function working with mass quantities
fn add_masses<V>(mass1: mass::Mass<V>, mass2: mass::Mass<V>) -> mass::Mass<V>
where
    V: Num,
{
    mass1 + mass2
}

fn main() {
    println!("🔧 Generic Functions with SI Quantities Demo");
    println!("==========================================\n");

    // Test generic functions with different numeric types
    println!("📏 Length Operations:");
    
    let length1_f64 = length::f64::Length::from::<Meter>(3.0);
    let length2_f64 = length::f64::Length::from::<Meter>(4.0);
    let total_length_f64 = add_lengths(length1_f64, length2_f64);
    
    println!("f64: {} m + {} m = {} m", 3.0, 4.0, total_length_f64.to::<Meter>());
    
    let length1_i32 = length::i32::Length::from::<Meter>(3);
    let length2_i32 = length::i32::Length::from::<Meter>(4);
    let total_length_i32 = add_lengths(length1_i32, length2_i32);
    
    println!("i32: {} m + {} m = {} m", 3, 4, total_length_i32.to::<Meter>());

    // Test unit conversions within functions
    println!("\n🔄 Unit Conversions:");
    
    let km_length = length::f64::Length::from::<Kilometer>(2.5);
    let m_length = length::f64::Length::from::<Meter>(500.0);
    let total_km = add_lengths(km_length, m_length);
    
    println!("2.5 km + 500 m = {} km", total_km.to::<Kilometer>());
    println!("2.5 km + 500 m = {} m", total_km.to::<Meter>());

    // Test mass operations
    println!("\n⚖️  Mass Operations:");
    
    let mass1 = mass::f64::Mass::from::<Kilogram>(2.5);
    let mass2 = mass::f64::Mass::from::<Gram>(1500.0);
    let total_mass = add_masses(mass1, mass2);
    
    println!("2.5 kg + 1500 g = {} kg", total_mass.to::<Kilogram>());
    println!("2.5 kg + 1500 g = {} g", total_mass.to::<Gram>());

    // Test pure numeric calculation
    println!("\n🧮 Pure Numeric Calculation:");
    
    let speed = 65.0; // km/h
    let time = 2.5;   // hours
    let distance = calculate_distance(speed, time);
    
    println!("{} km/h × {} hours = {} km", speed, time, distance);

    println!("\n✅ Generic functions work seamlessly with:");
    println!("   • Different numeric types (i32, f64, etc.)");
    println!("   • Automatic unit conversions");
    println!("   • Type-safe dimensional analysis");
    println!("   • Zero runtime overhead");
}
