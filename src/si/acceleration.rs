use typenum::*;

// SI derived unit: meter per second squared
units! {
    MeterPerSecondSquared: "m/s²", "meter per second squared";
    StandardGravity: "G", "standard gravity";
}

convert_linear! {
    StandardGravity => MeterPerSecondSquared: 9.80665; // 1 G = 9.80665 m/s²
}

// Acceleration quantity definition (Length/Time²)
use super::{ISQ, SiScale};
quantity!(Acceleration, ISQ<P1, Z0, N2, Z0, Z0, Z0, Z0>, SiScale, MeterPerSecondSquared);

// Re-export types for convenience
pub use acceleration::Acceleration;
pub use acceleration::*;
