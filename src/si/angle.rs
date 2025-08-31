use typenum::*;

// Base angle unit: radian (dimensionless but in its own dimension)
units! {
    Radian: "rad", "radian";
    Degree: "°", "degree";
    Gradian: "gon", "gradian";
    Turn: "rev", "turn";
}

// ===== CONVERSION RELATIONSHIPS =====

// Degree to Radian
convert_unit! {
    Degree: |radian| radian * 180.0 / std::f64::consts::PI;
    Radian: |degree| degree * std::f64::consts::PI / 180.0;
}

// Gradian to Radian
convert_unit! {
    Gradian: |radian| radian * 200.0 / std::f64::consts::PI;
    Radian: |gradian| gradian * std::f64::consts::PI / 200.0;
}

// Turn to Radian
convert_unit! {
    Turn: |radian| radian / (2.0 * std::f64::consts::PI);
    Radian: |turn| turn * 2.0 * std::f64::consts::PI;
}

// Angle quantity definition (dimensionless)
use super::{SI, SIScale};
quantity!(Angle, SI<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale);
