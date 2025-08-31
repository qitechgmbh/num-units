use crate::{
    prefix::{KILO, MEGA, MICRO, MILLI, NANO, PICO},
    si::ISQ,
    unit,
};

pub type ElectricCurrentDimension = ISQ<0, 0, 0, 1, 0, 0, 0>;

// SI base unit
unit!(
    Ampere,
    amperes,
    1.0,
    0.0,
    "ampere",
    "A",
    ElectricCurrentDimension
);

// Metric prefixes
unit!(
    Milliampere,
    milliamperes,
    MILLI,
    0.0,
    "milliampere",
    "mA",
    ElectricCurrentDimension
);
unit!(
    Microampere,
    microamperes,
    MICRO,
    0.0,
    "microampere",
    "μA",
    ElectricCurrentDimension
);
unit!(
    Nanoampere,
    nanoamperes,
    NANO,
    0.0,
    "nanoampere",
    "nA",
    ElectricCurrentDimension
);
unit!(
    Picoampere,
    picoamperes,
    PICO,
    0.0,
    "picoampere",
    "pA",
    ElectricCurrentDimension
);
unit!(
    Kiloampere,
    kiloamperes,
    KILO,
    0.0,
    "kiloampere",
    "kA",
    ElectricCurrentDimension
);
unit!(
    Megaampere,
    megaamperes,
    MEGA,
    0.0,
    "megaampere",
    "MA",
    ElectricCurrentDimension
);

// Legacy/practical units
unit!(
    Abampere,
    abamperes,
    10.0,
    0.0,
    "abampere",
    "abA",
    ElectricCurrentDimension
);
unit!(
    Statampere,
    statamperes,
    3.335641e-10,
    0.0,
    "statampere",
    "statA",
    ElectricCurrentDimension
);

// Biot (alternative name for abampere)
unit!(
    Biot,
    biots,
    10.0,
    0.0,
    "biot",
    "Bi",
    ElectricCurrentDimension
);
