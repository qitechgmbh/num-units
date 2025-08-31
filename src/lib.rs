#![cfg_attr(not(feature = "std"), no_std)]

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
    use typenum::*;

    crate::system!(Motion, LENGTH, TIME); // Length / Time

    // Basic
    quantity!(Scalar, Motion<Z0, Z0>);
    quantity!(Time, Motion<Z0, P1>); // Time
    quantity!(Frequency, Motion<Z0, N1>); // 1 / Time
    quantity!(PerSecondSquared, Motion<Z0, N2>); // 1 / Time²
    quantity!(PerSecondCubed, Motion<Z0, N3>); // 1 / Time³
    quantity!(PerSecondToTheFourth, Motion<Z0, N4>); // 1 / Time⁴
    quantity!(PerSecondToTheFifth, Motion<Z0, N5>); // 1 / Time⁵
    quantity!(PerSecondToTheSixth, Motion<Z0, N6>); // 1 / Time⁶

    // 1D Motion
    quantity!(Length, Motion<P1, Z0>); // Length
    quantity!(Velocity, Motion<P1, N1>); // Length / Time
    quantity!(Acceleration, Motion<P1, N2>); // Length / Time²
    quantity!(Jerk, Motion<P1, N3>); // Length / Time³
    quantity!(Snap, Motion<P1, N4>); // Length / Time⁴
    quantity!(Crackle, Motion<P1, N5>); // Length / Time⁵
    quantity!(Pop, Motion<P1, N6>); // Length / Time⁶

    // Angular Motion
    quantity!(Angle, Motion<Z0, Z0>); // Dimensionless
    quantity!(AngularVelocity, Motion<Z0, N1>); // 1 / Time
    quantity!(AngularAcceleration, Motion<Z0, N2>); // 1 / Time²
    quantity!(AngularJerk, Motion<Z0, N3>); // 1 / Time³
    quantity!(AngularSnap, Motion<Z0, N4>); // 1 / Time⁴
    quantity!(AngularCrackle, Motion<Z0, N5>); // 1 / Time⁵
    quantity!(AngularPop, Motion<Z0, N6>); // 1 / Time⁶

    // 2D Motion
    quantity!(Area, Motion<P2, Z0>); // Length²
    quantity!(AreaVelocity, Motion<P2, N1>); // Length² / Time
    quantity!(AreaAcceleration, Motion<P2, N2>); // Length² / Time²
    quantity!(AreaJerk, Motion<P2, N3>); // Length² / Time³
    quantity!(AreaSnap, Motion<P2, N4>); // Length² / Time⁴
    quantity!(AreaCrackle, Motion<P2, N5>); // Length² / Time⁵
    quantity!(AreaPop, Motion<P2, N6>); // Length² / Time⁶

    // 3D Motion
    quantity!(Volume, Motion<P3, Z0>); // Length³
    quantity!(VolumeVelocity, Motion<P3, N1>); // Length³ / Time
    quantity!(VolumeAcceleration, Motion<P3, N2>); // Length³ / Time²
    quantity!(VolumeJerk, Motion<P3, N3>); // Length³ / Time³
    quantity!(VolumeSnap, Motion<P3, N4>); // Length³ / Time⁴
    quantity!(VolumeCrackle, Motion<P3, N5>); // Length³ / Time⁵
    quantity!(VolumePop, Motion<P3, N6>); // Length³ / Time⁶

    // 4D Motion
    quantity!(Hypercube, Motion<P4, Z0>); // Length⁴
    quantity!(HypercubeVelocity, Motion<P4, N1>); // Length⁴ / Time
    quantity!(HypercubeAcceleration, Motion<P4, N2>); // Length⁴ / Time²
    quantity!(HypercubeJerk, Motion<P4, N3>); // Length⁴ / Time³
    quantity!(HypercubeSnap, Motion<P4, N4>); // Length⁴ / Time⁴
    quantity!(HypercubeCrackle, Motion<P4, N5>); // Length⁴ / Time⁵
    quantity!(HypercubePop, Motion<P4, N6>); // Length⁴ / Time⁶
}
