//! Demonstration that the quantity! macro works with any dimension system

#![feature(generic_const_exprs)]

use num_units::motion::{area::Area, length::Length, scalar::Scalar, volume::Volume};

fn main() {
    println!("quantity! macro works with any dimension system!");

    let l1 = Length::from_raw(3.0);
    let l2 = Length::from_raw(4.0);
    let s1 = Scalar::from_raw(5.0);
    let t1 = Scalar::from_raw(5.0);

    let length: Length<_> = l1 * s1; // Length * Scalar = Length
    let area: Area<_> = l1 * l2; // Length * Length = Area
    let volume: Volume<_> = area * l1; // Area * Length = Volume
    let x = volume * area; // Volume * Area = 5D Cube
    let volumetric_flow = volume / t1; // Volume / Time = Volumetric Flow

    assert_eq!(*length.raw(), 15.0);
    assert_eq!(*area.raw(), 12.0);
    assert_eq!(*volume.raw(), 36.0);
    assert_eq!(*x.raw(), 432.0);
    assert_eq!(*volumetric_flow.raw(), 7.2);
}
