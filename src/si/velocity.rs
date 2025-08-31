use typenum::*;

// SI derived unit: meter per second
base_units! {
    MeterPerSecond: "meter per second", "m/s";
    KilometerPerHour: "kilometer per hour", "km/h";
    MilePerHour: "mile per hour", "mph";
}

// ===== CONVERSION RELATIONSHIPS =====

// Kilometer per hour to meter per second
convert_base_unit! {
    KilometerPerHour: |meter_per_second| meter_per_second * 3.6;
    MeterPerSecond: |kilometer_per_hour| kilometer_per_hour / 3.6;
}

// Mile per hour to meter per second (1 mile = 1609.344 meters)
convert_base_unit! {
    MilePerHour: |meter_per_second| meter_per_second * 2.23694;
    MeterPerSecond: |mile_per_hour| mile_per_hour / 2.23694;
}

// Velocity quantity definition (Length/Time)
use super::{SI, SIScale};
quantity!(Velocity, SI<P1, Z0, N1, Z0, Z0, Z0, Z0>, SIScale);
