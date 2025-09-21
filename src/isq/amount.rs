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

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Millimole => Mole: MILLI;      // 1 mmol = 0.001 mol
    Micromole => Mole: MICRO;      // 1 μmol = 0.000001 mol
    Nanomole => Mole: NANO;        // 1 nmol = 0.000000001 mol
    Picomole => Mole: PICO;        // 1 pmol = 0.000000000001 mol
    Kilomole => Mole: KILO;        // 1 kmol = 1000 mol
}

// Particle counting conversions using Avogadro's number
crate::convert_linear! {
    Particle => Mole: 1.66053906660e-24;
}

// Amount quantity definition
use super::{ISQ, SiScale};
quantity!(Amount, ISQ<Z0, Z0, Z0, Z0, Z0, P1, Z0>, SiScale, Mole);

// Re-export types for convenience
pub use amount::Amount;
pub use amount::*;
