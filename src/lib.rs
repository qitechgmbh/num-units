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
#[macro_use]
pub mod conversions;

pub mod si;

// Re-export num_traits for convenience
pub use num_traits;
