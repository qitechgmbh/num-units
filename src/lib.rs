#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
pub mod prefix;
#[macro_use]
pub mod quantity;
#[macro_use]
pub mod base_units;
#[macro_use]
pub mod scale;
#[macro_use]
pub mod system;
pub mod conversions;
pub mod si;
pub mod unit;

// Re-export num_traits for convenience
pub use num_traits;

// Re-export all quantity types at the root level for compatibility
pub use si::acceleration::acceleration;
pub use si::amount::amount;
pub use si::angle::angle;
pub use si::area::area;
pub use si::current::current;
pub use si::energy::energy;
pub use si::force::force;
pub use si::length::length;
pub use si::luminosity::luminosity;
pub use si::mass::mass;
pub use si::power::power;
pub use si::scalar::scalar;
pub use si::temperature::temperature;
pub use si::time::time;
pub use si::velocity::velocity;
pub use si::volume::volume;
