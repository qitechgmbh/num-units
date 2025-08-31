use crate::{base_units::*, prefix::*, unit::Unit};

// ===== SI BASE UNIT =====
base_unit! {
    dimension: LuminosityDimension;
    Candela: "candela", "cd";
}

// ===== METRIC PREFIXES =====
base_unit! {
    dimension: LuminosityDimension;
    Millicandela: "millicandela", "mcd";
}

convert_base_unit! {
    Candela: |candela| candela * MILLI;
    Millicandela: |millicandela| millicandela / MILLI;
}
