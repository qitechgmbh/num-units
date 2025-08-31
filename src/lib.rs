#![cfg_attr(not(feature = "std"), no_std)]
#![feature(generic_const_exprs)]

pub mod prefix;
#[macro_use]
pub mod quantity;
pub mod si;
pub mod system;
pub mod unit;

// Re-export num_traits for convenience
pub use num_traits;

system!(
    ISQ,
    LENGTH,
    MASS,
    TIME,
    CURRENT,
    TEMPERATURE,
    AMOUNT_OF_SUBSTANCE,
    LUMINOSITY
);

pub mod motion {
    crate::system!(Motion, LENGTH, TIME); // Length / Time
    // Basic
    quantity!(Scalar, Motion<0, 0>);
    quantity!(Time, Motion<0, 1>); // Time
    quantity!(Frequency, Motion<0, -1>); // 1 / Time
    quantity!(PerSecondSquared, Motion<0, -2>); // 1 / Time²
    quantity!(PerSecondCubed, Motion<0, -3>); // 1 / Time³
    quantity!(PerSecondToTheFourth, Motion<0, -4>); // 1 / Time⁴
    quantity!(PerSecondToTheFifth, Motion<0, -5>); // 1 / Time⁵
    quantity!(PerSecondToTheSixth, Motion<0, -6>); // 1 / Time⁶

    // 1D Motion
    quantity!(Length, Motion<1, 0>); // Length
    quantity!(Velocity, Motion<1, -1>); // Length / Time
    quantity!(Acceleration, Motion<1, -2>); // Length / Time²
    quantity!(Jerk, Motion<1, -3>); // Length / Time³
    quantity!(Snap, Motion<1, -4>); // Length / Time⁴
    quantity!(Crackle, Motion<1, -5>); // Length / Time⁵
    quantity!(Pop, Motion<1, -6>); // Length / Time⁶

    // Angular Motion
    quantity!(Angle, Motion<0, 0>); // Dimensionless
    quantity!(AngularVelocity, Motion<0, -1>); // 1 / Time
    quantity!(AngularAcceleration, Motion<0, -2>); // 1 / Time²
    quantity!(AngularJerk, Motion<0, -3>); // 1 / Time³
    quantity!(AngularSnap, Motion<0, -4>); // 1 / Time⁴
    quantity!(AngularCrackle, Motion<0, -5>); // 1 / Time⁵
    quantity!(AngularPop, Motion<0, -6>); // 1 / Time⁶

    // 2D Motion
    quantity!(Area, Motion<2, 0>); // Length²
    quantity!(AreaVelocity, Motion<2, -1>); // Length² / Time
    quantity!(AreaAcceleration, Motion<2, -2>); // Length² / Time²
    quantity!(AreaJerk, Motion<2, -3>); // Length² / Time³
    quantity!(AreaSnap, Motion<2, -4>); // Length² / Time⁴
    quantity!(AreaCrackle, Motion<2, -5>); // Length² / Time⁵
    quantity!(AreaPop, Motion<2, -6>); // Length² / Time⁶

    // 3D Motion
    quantity!(Volume, Motion<3, 0>); // Length³
    quantity!(VolumeVelocity, Motion<3, -1>); // Length³ / Time
    quantity!(VolumeAcceleration, Motion<3, -2>); // Length³ / Time²
    quantity!(VolumeJerk, Motion<3, -3>); // Length³ / Time³
    quantity!(VolumeSnap, Motion<3, -4>); // Length³ / Time⁴
    quantity!(VolumeCrackle, Motion<3, -5>); // Length³ / Time⁵
    quantity!(VolumePop, Motion<3, -6>); // Length³ / Time⁶

    // 4D Motion
    quantity!(Hypercube, Motion<4, 0>); // Length⁴
    quantity!(HypercubeVelocity, Motion<4, -1>); // Length⁴ / Time
    quantity!(HypercubeAcceleration, Motion<4, -2>); // Length⁴ / Time²
    quantity!(HypercubeJerk, Motion<4, -3>); // Length⁴ / Time³
    quantity!(HypercubeSnap, Motion<4, -4>); // Length⁴ / Time⁴
    quantity!(HypercubeCrackle, Motion<4, -5>); // Length⁴ / Time⁵
    quantity!(HypercubePop, Motion<4, -6>); // Length⁴ / Time⁶
}
