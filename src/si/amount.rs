use crate::{
    prefix::{KILO, MICRO, MILLI, NANO, PICO},
    unit,
};

// All amount of substance units
unit! {
    system: crate::si;
    quantity: crate::amount_of_substance;

    // SI base unit
    @mole: 1.0; "mol", "mole", "moles";

    // Metric prefixes
    @millimole: MILLI; "mmol", "millimole", "millimoles";
    @micromole: MICRO; "μmol", "micromole", "micromoles";
    @nanomole: NANO; "nmol", "nanomole", "nanomoles";
    @picomole: PICO; "pmol", "picomole", "picomoles";
    @kilomole: KILO; "kmol", "kilomole", "kilomoles";

    // Legacy chemistry units
    @pound_mole: 453.59237; "lb-mol", "pound-mole", "pound-moles";

    // Particle counting
    @particle: 1.66053906660e-24; "particle", "particle", "particles";
}
