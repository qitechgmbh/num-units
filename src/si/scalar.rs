use typenum::*;

// Dimensionless scalar base unit
units! {
    Unitless: "", "unitless";
}

// Scalar quantity definition (dimensionless)
use super::{ISQ, SiScale};
quantity!(Scalar, ISQ<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, Unitless);

// Re-export types for convenience
pub use scalar::Scalar;
pub use scalar::*;