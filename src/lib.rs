#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
pub mod prefix;
#[macro_use]
pub mod quantity;
#[macro_use]
pub mod base_units;
pub mod conversions;
pub mod si;
pub mod system;
pub mod unit;

// Re-export num_traits for convenience
pub use num_traits;

// Import typenum types
use typenum::*;

system!(ISQ, L, M, T, I, TH, N, J);

// Define all quantities at crate root level
quantity!(Length, ISQ<P1, Z0, Z0, Z0, Z0, Z0, Z0>); // Length
quantity!(Mass, ISQ<Z0, P1, Z0, Z0, Z0, Z0, Z0>); // Mass
quantity!(Time, ISQ<Z0, Z0, P1, Z0, Z0, Z0, Z0>); // Time
quantity!(Current, ISQ<Z0, Z0, Z0, P1, Z0, Z0, Z0>); // Electric Current
quantity!(Temperature, ISQ<Z0, Z0, Z0, Z0, P1, Z0, Z0>); // Thermodynamic Temperature
quantity!(Amount, ISQ<Z0, Z0, Z0, Z0, Z0, P1, Z0>); // Amount of Substance
quantity!(Luminosity, ISQ<Z0, Z0, Z0, Z0, Z0, Z0, P1>); // Luminous Intensity

// Derived quantities
quantity!(Area, ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>); // Length²
quantity!(Volume, ISQ<P3, Z0, Z0, Z0, Z0, Z0, Z0>); // Length³
quantity!(Velocity, ISQ<P1, Z0, N1, Z0, Z0, Z0, Z0>); // Length/Time
quantity!(Scalar, ISQ<Z0, Z0, Z0, Z0, Z0, Z0, Z0>); // Dimensionless
quantity!(Angle, ISQ<Z0, Z0, Z0, Z0, Z0, Z0, Z0>); // Dimensionless (angular)

// Re-export SI units
pub use si::*;
