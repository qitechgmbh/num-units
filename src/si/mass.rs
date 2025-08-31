use crate::{base_units::*, prefix::*, unit::Unit};

// ===== SI/METRIC UNITS =====
// SI base unit
base_unit! {
    dimension: MassDimension;
    Gram: "gram", "g";
    Kilogram: "kilogram", "kg";
    Tonne: "tonne", "t";
}


convert_base_unit! {
    Kilogram: |gram| gram * KILO;
    Gram: |kilogram| kilogram / KILO;
}
convert_base_unit! {
    Tonne: |gram| gram / 1_000_000.0;
    Gram: |tonne| tonne * 1_000_000.0;
}