use crate::prefix::{KILO, MICRO, MILLI, NANO, PICO};
use typenum::*;

// ===== SI BASE UNIT =====
base_units! {
    Mole: "mole", "mol";
}

// ===== METRIC PREFIXES =====
base_units! {
    Millimole: "millimole", "mmol";
    Micromole: "micromole", "μmol";
    Nanomole: "nanomole", "nmol";
    Picomole: "picomole", "pmol";
    Kilomole: "kilomole", "kmol";
}

// ===== PARTICLE COUNTING =====
base_units! {
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

// Amount quantity definition
use super::{SI, SIScale};
quantity!(Amount, SI<Z0, Z0, Z0, Z0, Z0, P1, Z0>, SIScale);
