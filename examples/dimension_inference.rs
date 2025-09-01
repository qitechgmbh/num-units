//! Demonstration of dimensional analysis and cross-dimensional operations

use num_units::si::{
    area::{SquareMeter, area},
    length::{Meter, length},
    scalar::{Unitless, scalar},
    volume::{CubicMeter, volume},
};

fn main() {
    println!("🔬 Dimensional Analysis & Cross-Dimensional Operations");
    println!("====================================================\n");

    let l1 = length::f64::Length::from::<Meter>(3.0);
    let l2 = length::f64::Length::from::<Meter>(4.0);
    let s1 = scalar::f64::Scalar::from::<Unitless>(5.0);
    let t1 = scalar::f64::Scalar::from::<Unitless>(5.0);

    println!("📏 Basic Quantities:");
    println!("  Length 1: {} m", l1.to::<Meter>());
    println!("  Length 2: {} m", l2.to::<Meter>());
    println!("  Scalar: {}", s1.to::<Unitless>());

    // Same-dimension operations
    println!("\n➕ Same-Dimension Operations:");
    let length_sum = l1 + l2; // Length + Length = Length
    println!("  {} m + {} m = {} m", 3.0, 4.0, length_sum.to::<Meter>());

    // Scalar multiplication
    println!("\n✖️  Scalar Operations:");
    let scaled_length: length::f64::Length = l1 * s1.to::<Unitless>(); // Length * Scalar = Length
    println!("  {} m × {} = {} m", 3.0, 5.0, scaled_length.to::<Meter>());

    // Cross-dimensional operations
    println!("\n🌟 Cross-Dimensional Operations:");

    // Length × Length = Area
    let area: area::f64::Area = l1 * l2; // This should work now!
    println!("  {} m × {} m = {} m²", 3.0, 4.0, area.to::<SquareMeter>());

    // Area × Length = Volume
    let volume: volume::f64::Volume = area * l1; // Area * Length = Volume
    println!(
        "  {} m² × {} m = {} m³",
        12.0,
        3.0,
        volume.to::<CubicMeter>()
    );

    // Volume ÷ Time = Volumetric Flow Rate (still Volume since time is scalar)
    let volumetric_flow = volume / t1.to::<Unitless>(); // Volume / Time = Volume/Time
    println!(
        "  {} m³ ÷ {} s = {} m³/s",
        36.0,
        5.0,
        volumetric_flow.to::<CubicMeter>()
    );

    println!("\n✅ Verification:");
    assert_eq!(scaled_length.to::<Meter>(), 15.0);
    assert_eq!(area.to::<SquareMeter>(), 12.0);
    assert_eq!(volume.to::<CubicMeter>(), 36.0);
    assert_eq!(volumetric_flow.to::<CubicMeter>(), 7.2);
    println!("  All calculations verified! ✓");
}
