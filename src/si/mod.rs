dimension!(SI, L, M, T, I, TH, N, J);

// Dimensionless
pub type Dimensionless = SI<0, 0, 0, 0, 0, 0, 0>;

// Unit modules
pub mod amount;
pub mod current;
pub mod length;
pub mod luminosity;
pub mod mass;
pub mod temperature;
pub mod time;

// Re-export all units
pub use amount::*;
pub use current::*;
pub use length::*;
pub use luminosity::*;
pub use mass::*;
pub use temperature::*;
pub use time::*;

use crate::dimension;
