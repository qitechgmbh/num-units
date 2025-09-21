use typenum::*;

// SI derived unit: meter per second
units! {
    MeterPerSecond: "m/s", "meter per second";
    KilometerPerHour: "km/h", "kilometer per hour";
    MilePerHour: "mph", "mile per hour";
}

// ===== CONVERSION RELATIONSHIPS =====

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    KilometerPerHour => MeterPerSecond: 1.0 / 3.6;     // 1 km/h = 0.2778 m/s
    MilePerHour => MeterPerSecond: 1.0 / 2.23694;      // 1 mph = 0.44704 m/s
}

// Velocity quantity definition (Length/Time)
use super::{ISQ, SiScale};
quantity!(Velocity, ISQ<P1, Z0, N1, Z0, Z0, Z0, Z0>, SiScale, MeterPerSecond);

// Re-export types for convenience
pub use velocity::Velocity;
pub use velocity::*;
