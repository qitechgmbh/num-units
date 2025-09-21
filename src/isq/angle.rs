// Angular units (dimensionless)
units! {
    Revolution: "rev", "revolution";
    Radian: "rad", "radian";
    Degree: "°", "degree";
}

// Angular conversions using convert_linear!
crate::convert_linear! {
    Revolution => Unitless: 1.0;                       // 1 revolution = 1 unitless
    Radian => Unitless: 1.0 / (2.0 * std::f64::consts::PI);  // 1 radian = 1/(2π) revolutions
    Degree => Unitless: 1.0 / 360.0;                   // 1 degree = 1/360 revolutions
}

crate::convert_matrix! {
    Unitless => Revolution, Radian, Degree
}

// Import Unitless from unitless module
use super::scalar::Unitless;