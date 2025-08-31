use crate::prefix::{MICRO, MILLI};

// ===== SI BASE UNIT =====
base_units! {
    Ampere: "ampere", "A";
}

// ===== METRIC PREFIXES =====
base_units! {
    Milliampere: "milliampere", "mA";
    Microampere: "microampere", "μA";
}

// ===== CONVERSION RELATIONSHIPS =====

// Metric prefix conversions
convert_base_unit! {
    Milliampere: |ampere| ampere / MILLI;
    Ampere: |milliampere| milliampere * MILLI;
}

convert_base_unit! {
    Microampere: |ampere| ampere / MICRO;
    Ampere: |microampere| microampere * MICRO;
}
