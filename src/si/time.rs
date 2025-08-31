use crate::{base_units::*, prefix::*, unit::Unit};

// ===== SI BASE UNIT =====
base_unit! {
    dimension: TimeDimension;
    Second: "second", "s";
}

// ===== METRIC PREFIXES =====
base_unit! {
    dimension: TimeDimension;
    Millisecond: "millisecond", "ms";
    Microsecond: "microsecond", "μs";
    Nanosecond: "nanosecond", "ns";
    Picosecond: "picosecond", "ps";
    Kilosecond: "kilosecond", "ks";
}

convert_base_unit! {
    Millisecond: |second| second * MILLI;
    Second: |millisecond| millisecond / MILLI;
}

convert_base_unit! {
    Microsecond: |second| second * MICRO;
    Second: |microsecond| microsecond / MICRO;
}

convert_base_unit! {
    Nanosecond: |second| second * NANO;
    Second: |nanosecond| nanosecond / NANO;
}

convert_base_unit! {
    Picosecond: |second| second * PICO;
    Second: |picosecond| picosecond / PICO;
}

convert_base_unit! {
    Kilosecond: |second| second * KILO;
    Second: |kilosecond| kilosecond / KILO;
}
