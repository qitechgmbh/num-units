use typenum::*;

// SI derived unit: meter per second squared
units! {
    MeterPerSecondSquared: "m/s²", "meter per second squared";
}

// Acceleration quantity definition (Length/Time²)
use super::{SI, SIScale};
quantity!(Acceleration, SI<P1, Z0, N2, Z0, Z0, Z0, Z0>, SIScale);
