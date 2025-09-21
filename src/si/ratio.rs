/// # Ratio Units - Dimensionless Ratio Measurements
///
/// This module defines dimensionless ratio units and their conversions. Ratios are
/// dimensionless quantities used to express proportions, percentages, and concentrations.
///
/// ## Important Note
///
/// Due to architectural limitations in num-units (lack of "kind" system like UOM),
/// ratio units cannot have a separate quantity type. Instead, they are implemented
/// using the existing Unitless base quantity. This means:
/// - Ratio quantities are treated as dimensionless scalars
/// - UOM compatibility tests cannot be performed for ratio units
/// - All conversions are done through the Unitless base unit
///
/// ## Base Unit
///
/// - **Ratio**: 1.0 (dimensionless unit)
///
/// ## Percentage and Parts-Per Units
///
/// - **PartPerHundred**: 0.01 (equivalent to percent)
/// - **Percent**: 0.01 (%)
/// - **PartPerThousand**: 0.001 (equivalent to per mille)
/// - **PerMille**: 0.001 (‰)
/// - **PartPerTenThousand**: 0.0001 (equivalent to basis point)
/// - **BasisPoint**: 0.0001 (bp)
/// - **PartPerMillion**: 0.000001 (ppm)
/// - **PartPerBillion**: 0.000000001 (ppb)
/// - **PartPerTrillion**: 0.000000000001 (ppt)
/// - **PartPerQuadrillion**: 0.000000000000001 (ppq)
///
/// ## Usage
///
/// ```rust,ignore
/// use num_units::si::scalar::Unitless;
/// use num_units::si::ratio::{Percent, PartsPerMillion, BasisPoint};
///
/// // Create ratio quantities as dimensionless scalars
/// let percentage = Unitless::from::<Percent>(50.0);
/// let concentration = Unitless::from::<PartsPerMillion>(10.0);
/// let basis_points = Unitless::from::<BasisPoint>(25.0);
///
/// // Convert between units
/// let decimal = percentage.to_ratio();     // 0.5
/// let percent_from_ppm = concentration.to::<Percent>(); // 0.001%
/// let ppm_from_bp = basis_points.to::<PartsPerMillion>(); // 0.25 ppm
///
/// // Arithmetic operations
/// let total_ratio = percentage + concentration; // Automatic conversion
/// ```
///
/// ## Architecture
///
/// Ratio units are implemented as dimensionless quantities using Unitless as the base.
/// This is a limitation of the num-units framework which lacks UOM's "kind" system for
/// distinguishing quantities with identical dimensions.
use crate::prefix::*;

// Ratio units (dimensionless)
units! {
    Ratio: "", "ratio";
    PartPerHundred: "parts per hundred", "part per hundred";
    Percent: "%", "percent";
    PartPerThousand: "parts per thousand", "part per thousand";
    PerMille: "‰", "per mille";
    Promille: "‰", "promille";
    PartPerTenThousand: "parts per ten thousand", "part per ten thousand";
    BasisPoint: "bp", "basis point";
    PartPerMillion: "ppm", "part per million";
    PartPerBillion: "ppb", "part per billion";
    PartPerTrillion: "ppt", "part per trillion";
    PartPerQuadrillion: "ppq", "part per quadrillion";
}

// Unit conversions using convert_linear! with exact UOM coefficients
crate::convert_linear! {
    // Exact UOM coefficients for ratio units
    Ratio => Unitless: ONE;
    PartPerHundred => Unitless: CENTI;
    Percent => Unitless: CENTI;
    PartPerThousand => Unitless: MILLI;
    PerMille => Unitless: MILLI;
    Promille => Unitless: MILLI;
    PartPerTenThousand => Unitless: DECIMILLI;
    BasisPoint => Unitless: DECIMILLI;
    PartPerMillion => Unitless: MICRO;
    PartPerBillion => Unitless: NANO;
    PartPerTrillion => Unitless: PICO;
    PartPerQuadrillion => Unitless: FEMTO;
}

crate::convert_matrix! {
    Unitless => Ratio, PartPerHundred, Percent, PartPerThousand, PerMille, PartPerTenThousand, BasisPoint, PartPerMillion, PartPerBillion, PartPerTrillion, PartPerQuadrillion
}

// Import Unitless from scalar module
use super::scalar::Unitless;
