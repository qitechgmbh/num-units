use crate::{base_units::*, prefix::*, unit::Unit};

// ===== SI BASE UNIT =====
base_unit! {
    dimension: AmountDimension;
    Mole: "mole", "mol";
}

// ===== METRIC PREFIXES =====
base_unit! {
    dimension: AmountDimension;
    Millimole: "millimole", "mmol";
    Micromole: "micromole", "μmol";
    Nanomole: "nanomole", "nmol";
    Picomole: "picomole", "pmol";
    Kilomole: "kilomole", "kmol";
}

// ===== PARTICLE COUNTING =====
base_unit! {
    dimension: AmountDimension;
    Particle: "particle", "particle";
}

// ===== CONVERSION RELATIONSHIPS =====

// Metric prefix conversions
convert_base_unit! {
    Mole: |mole| mole * MILLI;
    Millimole: |millimole| millimole / MILLI;
}

convert_base_unit! {
    Mole: |mole| mole * MICRO;
    Micromole: |micromole| micromole / MICRO;
}

convert_base_unit! {
    Nanomole: |mole| mole * NANO;
    Mole: |nanomole| nanomole / NANO;
}

convert_base_unit! {
    Mole: |mole| mole * PICO;
    Picomole: |picomole| picomole / PICO;
}

convert_base_unit! {
    Mole: |mole| mole * KILO;
    Kilomole: |kilomole| kilomole / KILO;
}

// Particle counting conversions
convert_base_unit! {
    Particle: |mole| mole / 1.66053906660e-24;
    Mole: |particle| particle * 1.66053906660e-24;
}
