use crate::{
    prefix::{KILO, MEGA, MICRO, MILLI, NANO, PICO},
    unit,
};

// All electric current units
unit! {
    system: crate::si;
    quantity: crate::electric_current;

    // SI base unit
    @ampere: 1.0; "A", "ampere", "amperes";

    // Metric prefixes
    @milliampere: MILLI; "mA", "milliampere", "milliamperes";
    @microampere: MICRO; "μA", "microampere", "microamperes";
    @nanoampere: NANO; "nA", "nanoampere", "nanoamperes";
    @picoampere: PICO; "pA", "picoampere", "picoamperes";
    @kiloampere: KILO; "kA", "kiloampere", "kiloamperes";
    @megaampere: MEGA; "MA", "megaampere", "megaamperes";

    // Legacy/practical units
    @Abampere: 10.0; "abA", "abampere", "abamperes";
    @Statampere: 3.335641e-10; "statA", "statampere", "statamperes";

    // Biot (alternative name for abampere)
    @biot: 10.0; "Bi", "biot", "biots";
}
