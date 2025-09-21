use typenum::*;

// ===== SI BASE UNIT =====
units! {
    Kelvin: "K", "kelvin";
    Celsius: "°C", "celsius";
    Fahrenheit: "°F", "fahrenheit";
}

// ===== CONVERSION RELATIONSHIPS =====

// Kelvin is the SI base unit for temperature
// Celsius: C + 273.15 = K
crate::convert_linear! {
    Celsius => Kelvin: 1.0, 273.15;
}

// Fahrenheit: (F - 32) * 5/9 + 273.15 = K
// Or: F * 5/9 - 32 * 5/9 + 273.15 = K
// Or: F * 5/9 + 255.372... = K
// But we need it in the form F * scale + offset = K
// So: F * (5/9) + 255.372... = K
// Let's use the direct conversion for clarity (float only due to fractional expressions)
crate::convert_float! {
    Fahrenheit: |val| (val - 273.15) * 9.0 / 5.0 + 32.0;
    Kelvin: |val| (val - 32.0) * 5.0 / 9.0 + 273.15;
}

crate::convert_matrix! {
    Kelvin => Celsius, Fahrenheit
}

// Temperature quantity definition
use super::{ISQ, SiScale};
quantity!(Temperature, ISQ<Z0, Z0, Z0, Z0, P1, Z0, Z0>, SiScale, Kelvin);

// Re-export types for convenience
pub use temperature::Temperature;
pub use temperature::*;
