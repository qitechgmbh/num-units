use typenum::*;

// Dimensionless scalar quantities
base_units! {
    Scalar: "scalar", "";
    Percent: "percent", "%";
    PartsPerMillion: "parts per million", "ppm";
    PartsPerBillion: "parts per billion", "ppb";
}

// ===== CONVERSION RELATIONSHIPS =====

// Percent to Scalar
convert_base_unit! {
    Percent: |scalar| scalar * 100.0;
    Scalar: |percent| percent / 100.0;
}

// Parts per million to Scalar
convert_base_unit! {
    PartsPerMillion: |scalar| scalar * 1_000_000.0;
    Scalar: |parts_per_million| parts_per_million / 1_000_000.0;
}

// Parts per billion to Scalar
convert_base_unit! {
    PartsPerBillion: |scalar| scalar * 1_000_000_000.0;
    Scalar: |parts_per_billion| parts_per_billion / 1_000_000_000.0;
}

// Scalar quantity definition (dimensionless)
use super::{SI, SIScale};
quantity!(Scalar, SI<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale);