/// Tests for generic functions with SI quantities
use num_traits::Num;
use num_units::{
    length::{Kilometer, Meter, length},
    mass::{Gram, Kilogram, mass},
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

#[test]
fn test_generic_length_operations() {
    // Test with f64
    let length1_f64 = length::f64::Length::from::<Meter>(3.0);
    let length2_f64 = length::f64::Length::from::<Meter>(4.0);
    let total_length_f64 = add_lengths(length1_f64, length2_f64);
    assert_eq!(total_length_f64.to::<Meter>(), 7.0);

    // Test with i32
    let length1_i32 = length::i32::Length::from::<Meter>(3);
    let length2_i32 = length::i32::Length::from::<Meter>(4);
    let total_length_i32 = add_lengths(length1_i32, length2_i32);
    assert_eq!(total_length_i32.to::<Meter>(), 7);
}

#[test]
fn test_unit_conversions_in_functions() {
    let km_length = length::f64::Length::from::<Kilometer>(2.5);
    let m_length = length::f64::Length::from::<Meter>(500.0);
    let total_km = add_lengths(km_length, m_length);

    assert_eq!(total_km.to::<Kilometer>(), 3.0);
    assert_eq!(total_km.to::<Meter>(), 3000.0);
}

#[test]
fn test_mass_operations() {
    let mass1 = mass::f64::Mass::from::<Kilogram>(2.5);
    let mass2 = mass::f64::Mass::from::<Gram>(1500.0);
    let total_mass = add_masses(mass1, mass2);

    assert_eq!(total_mass.to::<Kilogram>(), 4.0);
    assert_eq!(total_mass.to::<Gram>(), 4000.0);
}

#[test]
fn test_pure_numeric_calculation() {
    let speed = 65.0; // km/h
    let time = 2.5; // hours
    let distance = calculate_distance(speed, time);

    assert_eq!(distance, 162.5);
}
