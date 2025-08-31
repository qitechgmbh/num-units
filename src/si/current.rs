use crate::{base_units::*, prefix::*, unit::Unit};

// ===== SI BASE UNIT =====
base_unit! {
    dimension: CurrentDimension;
    Ampere: "ampere", "A";
}

// ===== METRIC PREFIXES =====
base_unit! {
    dimension: CurrentDimension;
    Milliampere: "milliampere", "mA";
    Microampere: "microampere", "μA";
}

// ===== CONVERSION RELATIONSHIPS =====

// Metric prefix conversions
convert_base_unit! {
    Milliampere: |ampere| ampere * MILLI;
    Ampere: |milliampere| milliampere / MILLI;
}
