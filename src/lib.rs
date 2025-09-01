#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
pub mod prefix;
#[macro_use]
pub mod quantity;
#[macro_use]
pub mod unit;
#[macro_use]
pub mod scale;
#[macro_use]
pub mod system;
pub mod conversions;
mod isq;

// Re-export num_traits for convenience
pub use num_traits;

// Re-export all quantity types at the root level for compatibility
pub use isq::*;
