use crate::prefix::{KILO, MICRO, MILLI, NANO, PICO};
use typenum::*;

// ===== SI BASE UNIT =====
units! {
    Mole: "mol", "mole";
}

// ===== METRIC PREFIXES =====
units! {
    Millimole: "mmol", "millimole";
    Micromole: "μmol", "micromole";
    Nanomole: "nmol", "nanomole";
    Picomole: "pmol", "picomole";
    Kilomole: "kmol", "kilomole";
}

// ===== PARTICLE COUNTING =====
units! {
    Particle: "particle", "particle";
}

// ===== CONVERSION RELATIONSHIPS =====

// Metric prefix conversions
convert_unit! {
    Mole: |mole| mole * MILLI;
    Millimole: |millimole| millimole / MILLI;
}

convert_unit! {
    Mole: |mole| mole * MICRO;
    Micromole: |micromole| micromole / MICRO;
}

convert_unit! {
    Nanomole: |mole| mole * NANO;
    Mole: |nanomole| nanomole / NANO;
}

convert_unit! {
    Mole: |mole| mole * PICO;
    Picomole: |picomole| picomole / PICO;
}

convert_unit! {
    Mole: |mole| mole * KILO;
    Kilomole: |kilomole| kilomole / KILO;
}

// Particle counting conversions
convert_unit! {
    Particle: |mole| mole / 1.66053906660e-24;
    Mole: |particle| particle * 1.66053906660e-24;
}

// Amount quantity definition
use super::{SI, SIScale};
quantity!(Amount, SI<Z0, Z0, Z0, Z0, Z0, P1, Z0>, SIScale);
