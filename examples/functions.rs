//! Demonstration that the quantity! macro works with any dimension system

use num_traits::Num;
use num_units::{area::Area, length::Length, time::Time, velocity::Velocity, volume::Volume};

fn calc_area<V>(length: Length<V>, width: Length<V>) -> Area<V>
where
    V: Num,
{
    length * width
}

fn calc_volume<V>(length: Length<V>, width: Length<V>, height: Length<V>) -> Volume<V>
where
    V: Num,
{
    length * width * height
}

fn main() {
    println!("quantity! macro works with any dimension system!");

    let l1 = Length::from_base(3.0);
    let l2 = Length::from_base(4.0);
    let t1 = Time::from_base(5.0);

    // Infer dimensions in expressions
    let l3: Length<_> = l1 + l2; // Length + Length = Length
    let a1: Area<_> = l1 * l2; // Length * Length = Area
    let vol1: Volume<_> = a1 * l1; // Area * Length = Volume
    let vel1: Velocity<_> = l3 / t1; // Length / Time = Velocity

    assert_eq!(*l3.base(), 7.0);
    assert_eq!(*a1.base(), 12.0);
    assert_eq!(*vol1.base(), 36.0);
    assert_eq!(*vel1.base(), 7.0 / 5.0);

    // Create generic functions over quantities
    let length: Length<i8> = Length::from_base(1);
    let area = calc_area(length, length);
    let volume = calc_volume(length, length, length);

    assert_eq!(*length.base(), 1);
    assert_eq!(*area.base(), 1);
    assert_eq!(*volume.base(), 1);

    // Test multiplication with defined dimensions
    let area2 = length * length; // Length * Length = Area (defined)
    let volume2 = area * length; // Area * Length = Volume (defined)

    assert_eq!(*area2.base(), 1);
    assert_eq!(*volume2.base(), 1);
}
