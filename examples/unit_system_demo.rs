#![feature(generic_const_exprs)]

use nom::quantity::Quantity;
use nom::si::length::LengthDimension;
use nom::si::mass::MassDimension;
use nom::si::temperature::ThermodynamicTemperatureDimension;

fn main() {
    println!("🎯 Unit System Demo with Individual Unit Definitions!");
    println!("=====================================================");

    // ============================================================================
    // Length conversions with proper method names
    // ============================================================================
    println!("\n📏 Length Conversions:");

    // Create lengths using unit-specific methods
    let meter_length = Quantity::<f64, LengthDimension>::from_meters(1.0);
    let foot_length = Quantity::<f64, LengthDimension>::from_feet(3.28084);
    let cm_length = Quantity::<f64, LengthDimension>::from_centimeters(100.0);

    println!("1 meter = {} meters", meter_length.as_meters());
    println!("1 meter = {} feet", meter_length.as_feet());
    println!("1 meter = {} centimeters", meter_length.as_centimeters());
    println!("1 meter = {} inches", meter_length.as_inches());

    // Show they're all equivalent when converted to base units
    println!("\nAll these should be approximately equal in base units:");
    println!("1 meter: {} (base units)", meter_length.into_base());
    println!("3.28084 feet: {} (base units)", foot_length.into_base());
    println!("100 cm: {} (base units)", cm_length.into_base());

    // ============================================================================
    // Mass conversions with proper method names
    // ============================================================================
    println!("\n⚖️  Mass Conversions:");

    let kg_mass = Quantity::<f64, MassDimension>::from_kilograms(1.0);
    let lb_mass = Quantity::<f64, MassDimension>::from_pounds(2.20462);

    println!("1 kg = {} kilograms", kg_mass.as_kilograms());
    println!("1 kg = {} pounds", kg_mass.as_pounds());
    println!("1 kg = {} grams", kg_mass.as_grams());

    println!("2.20462 lbs = {} kg", lb_mass.as_kilograms());

    // ============================================================================
    // Temperature conversions (with offset for Celsius and Fahrenheit)
    // ============================================================================
    println!("\n🌡️  Temperature Conversions:");
    println!("Freezing point of water:");

    let celsius_freezing = Quantity::<f64, ThermodynamicTemperatureDimension>::from_celsius(0.0);
    let kelvin_freezing = Quantity::<f64, ThermodynamicTemperatureDimension>::from_kelvins(273.15);
    let fahrenheit_freezing =
        Quantity::<f64, ThermodynamicTemperatureDimension>::from_fahrenheit(32.0);

    println!("  {} °C", celsius_freezing.as_celsius());
    println!("  {} K", celsius_freezing.as_kelvins());
    println!("  {} °F", celsius_freezing.as_fahrenheit());

    println!("\nAll should be equal in base units (Kelvin):");
    println!("0°C: {} K (base)", celsius_freezing.into_base());
    println!("273.15 K: {} K (base)", kelvin_freezing.into_base());
    println!("32°F: {} K (base)", fahrenheit_freezing.into_base());

    println!("\nBoiling point of water:");
    let celsius_boiling = Quantity::<f64, ThermodynamicTemperatureDimension>::from_celsius(100.0);

    println!("  {} °C", celsius_boiling.as_celsius());
    println!("  {} K", celsius_boiling.as_kelvins());
    println!("  {} °F", celsius_boiling.as_fahrenheit());

    // ============================================================================
    // Dimensional arithmetic
    // ============================================================================
    println!("\n🧮 Dimensional Arithmetic:");

    let length1 = Quantity::<f64, LengthDimension>::from_meters(5.0);
    let length2 = Quantity::<f64, LengthDimension>::from_feet(6.0); // ~1.83 meters
    let total_length = length1 + length2;

    println!("5 meters + 6 feet = {} meters", total_length.as_meters());
    println!("5 meters + 6 feet = {} feet", total_length.as_feet());

    // Show dimensional multiplication
    let width = Quantity::<f64, LengthDimension>::from_meters(3.0);
    let height = Quantity::<f64, LengthDimension>::from_meters(4.0);
    let area = width * height; // Results in Length² (area)

    println!(
        "3m × 4m = {} square meters (in base units)",
        area.into_base()
    );

    println!("\n✅ Demo complete!");
}
