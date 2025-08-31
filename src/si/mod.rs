// Unit modules - define unit types like Meter, Kilogram, etc.
pub mod acceleration;
pub mod amount;
pub mod angle;
pub mod area;
pub mod current;
pub mod energy;
pub mod force;
pub mod length;
pub mod luminosity;
pub mod mass;
pub mod power;
pub mod scalar;
pub mod temperature;
pub mod time;
pub mod velocity;
pub mod volume;

// Create the SI system with unit scaling using the new syntax
system! {
    SI,
    L => length::Meter,
    M => mass::Kilogram,
    T => time::Second,
    I => current::Ampere,
    TH => temperature::Kelvin,
    N => amount::Mole,
    J => luminosity::Candela
}
