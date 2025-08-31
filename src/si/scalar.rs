use typenum::*;

// Dimensionless scalar quantities
units! {
    Scalar: "", "scalar";
    Percent: "%", "percent";
    PartsPerMillion: "ppm", "parts per million";
    PartsPerBillion: "ppb", "parts per billion";
}

// ===== CONVERSION RELATIONSHIPS =====

// Percent to Scalar
convert_unit! {
    Percent: |scalar| scalar * HECTO;
    Scalar: |percent| percent / HECTO;
}

// Parts per million to Scalar
convert_unit! {
    PartsPerMillion: |scalar| scalar * MEGA;
    Scalar: |parts_per_million| parts_per_million / MEGA;
}

// Parts per billion to Scalar
convert_unit! {
    PartsPerBillion: |scalar| scalar * GIGA;
    Scalar: |parts_per_billion| parts_per_billion / GIGA;
}

use crate::prefix::{GIGA, HECTO, MEGA};

// Scalar quantity definition (dimensionless)
use super::{SI, SIScale};
quantity!(Scalar, SI<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale);
