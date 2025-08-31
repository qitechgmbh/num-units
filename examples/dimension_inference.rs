//! Demonstration that the quantity! macro works with any dimension system

use num_units::{area::f64::Area, length::f64::Length, scalar::f64::Scalar, volume::f64::Volume};

fn main() {
    println!("quantity! macro works with any dimension system!");

    let l1 = Length::from_base(3.0);
    let l2 = Length::from_base(4.0);
    let s1 = Scalar::from_base(5.0);
    let t1 = Scalar::from_base(5.0);

    let length: Length = l1 * s1; // Length * Scalar = Length
    let area: Area = l1 * l2; // Length * Length = Area
    let volume: Volume = area * l1; // Area * Length = Volume
    let x = volume * area; // Volume * Area = 5D Cube
    let volumetric_flow = volume / t1; // Volume / Time = Volumetric Flow

    assert_eq!(*length.base(), 15.0);
    assert_eq!(*area.base(), 12.0);
    assert_eq!(*volume.base(), 36.0);
    assert_eq!(*x.base(), 432.0);
    assert_eq!(*volumetric_flow.base(), 7.2);
}
