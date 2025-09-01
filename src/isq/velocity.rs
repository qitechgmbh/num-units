use typenum::*;

// SI derived unit: meter per second
units! {
    MeterPerSecond: "m/s", "meter per second";
    KilometerPerHour: "km/h", "kilometer per hour";
    MilePerHour: "mph", "mile per hour";
}

// ===== CONVERSION RELATIONSHIPS =====

// Kilometer per hour to meter per second
convert_unit! {
    KilometerPerHour: |meter_per_second| meter_per_second * 3.6;
    MeterPerSecond: |kilometer_per_hour| kilometer_per_hour / 3.6;
}

// Mile per hour to meter per second (1 mile = 1609.344 meters)
convert_unit! {
    MilePerHour: |meter_per_second| meter_per_second * 2.23694;
    MeterPerSecond: |mile_per_hour| mile_per_hour / 2.23694;
}

// Velocity quantity definition (Length/Time)
use super::{ISQ, SiScale};
quantity!(Velocity, ISQ<P1, Z0, N1, Z0, Z0, Z0, Z0>, SiScale, MeterPerSecond);

// Re-export types for convenience
pub use velocity::Velocity;
pub use velocity::*;
