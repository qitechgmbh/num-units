// Base angle unit: radian (dimensionless but in its own dimension)
base_units! {
    Radian: "radian", "rad";
    Degree: "degree", "°";
    Gradian: "gradian", "gon";
    Turn: "turn", "rev";
}

// ===== CONVERSION RELATIONSHIPS =====

// Degree to Radian
convert_base_unit! {
    Degree: |radian| radian * 180.0 / std::f64::consts::PI;
    Radian: |degree| degree * std::f64::consts::PI / 180.0;
}

// Gradian to Radian
convert_base_unit! {
    Gradian: |radian| radian * 200.0 / std::f64::consts::PI;
    Radian: |gradian| gradian * std::f64::consts::PI / 200.0;
}

// Turn to Radian
convert_base_unit! {
    Turn: |radian| radian / (2.0 * std::f64::consts::PI);
    Radian: |turn| turn * 2.0 * std::f64::consts::PI;
}
