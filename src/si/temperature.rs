use typenum::*;

// ===== SI BASE UNIT =====
units! {
    Kelvin: "K", "kelvin";
    Celsius: "°C", "celsius";
    Fahrenheit: "°F", "fahrenheit";
}

// ===== CONVERSION RELATIONSHIPS =====

// Celsius to Kelvin (with offset)
convert_unit! {
    Kelvin: |kelvin| kelvin - 273.15;
    Celsius: |celsius| celsius + 273.15;
}

// Fahrenheit to Kelvin (with offset)
convert_unit! {
    Fahrenheit: |kelvin| (kelvin - 273.15) * 9.0 / 5.0 + 32.0;
    Kelvin: |fahrenheit| (fahrenheit - 32.0) * 5.0 / 9.0 + 273.15;
}

// Temperature quantity definition
use super::{SI, SIScale};
quantity!(Temperature, SI<Z0, Z0, Z0, Z0, P1, Z0, Z0>, SIScale);
