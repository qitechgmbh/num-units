use typenum::*;

// SI derived unit: meter per second squared
base_units! {
    MeterPerSecondSquared: "meter per second squared", "m/s²";
}

// Acceleration quantity definition (Length/Time²)
use super::{SI, SIScale};
quantity!(Acceleration, SI<P1, Z0, N2, Z0, Z0, Z0, Z0>, SIScale);
