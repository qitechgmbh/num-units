use crate::prefix::{CENTI, MICRO, MILLI, NANO};

// Ratios and percentages
units! {
    Percent: "%", "percent";
    Promille: "‰", "promille";
    PartsPerMillion: "ppm", "parts per million";
    PartsPerBillion: "ppb", "parts per billion";
}

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Percent => Unitless: CENTI;              // 1% = 0.01 unitless
    Promille => Unitless: MILLI;            // 1‰ = 0.001 unitless
    PartsPerMillion => Unitless: MICRO;       // 1 ppm = 0.000001 unitless
    PartsPerBillion => Unitless: NANO;       // 1 ppb = 0.000000001 unitless
}

crate::convert_matrix! {
    Unitless => Percent, Promille, PartsPerMillion, PartsPerBillion
}

// Import Unitless from unitless module
use super::scalar::Unitless;
