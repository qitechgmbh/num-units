use crate::{
    prefix::{KILO, MICRO, MILLI, NANO, PICO},
    si::ISQ,
    unit,
};

pub type AmountOfSubstanceDimension = ISQ<0, 0, 0, 0, 0, 1, 0>;

// SI base unit
unit!(
    Mole,
    moles,
    1.0,
    0.0,
    "mole",
    "mol",
    AmountOfSubstanceDimension
);

// Metric prefixes
unit!(
    Millimole,
    millimoles,
    MILLI,
    0.0,
    "millimole",
    "mmol",
    AmountOfSubstanceDimension
);
unit!(
    Micromole,
    micromoles,
    MICRO,
    0.0,
    "micromole",
    "μmol",
    AmountOfSubstanceDimension
);
unit!(
    Nanomole,
    nanomoles,
    NANO,
    0.0,
    "nanomole",
    "nmol",
    AmountOfSubstanceDimension
);
unit!(
    Picomole,
    picomoles,
    PICO,
    0.0,
    "picomole",
    "pmol",
    AmountOfSubstanceDimension
);
unit!(
    Kilomole,
    kilomoles,
    KILO,
    0.0,
    "kilomole",
    "kmol",
    AmountOfSubstanceDimension
);

// Legacy chemistry units
unit!(
    PoundMole,
    pound_moles,
    453.59237,
    0.0,
    "pound-mole",
    "lb-mol",
    AmountOfSubstanceDimension
);

// Particle counting
unit!(
    Particle,
    particles,
    1.66053906660e-24,
    0.0,
    "particle",
    "particle",
    AmountOfSubstanceDimension
);
