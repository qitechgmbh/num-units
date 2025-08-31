use crate::prefix::MILLI;

// ===== SI BASE UNIT =====
base_units! {
    Candela: "candela", "cd";
}

// ===== METRIC PREFIXES =====
base_units! {
    Millicandela: "millicandela", "mcd";
}

convert_base_unit! {
    Millicandela: |candela| candela * MILLI;
    Candela: |millicandela| millicandela / MILLI;
}
