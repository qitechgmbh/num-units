use crate::{
    prefix::{KILO, MILLI},
    si::ISQ,
    unit,
};

pub type LuminousIntensityDimension = ISQ<0, 0, 0, 0, 0, 0, 1>;

// SI base unit
unit!(
    Candela,
    candelas,
    1.0,
    0.0,
    "candela",
    "cd",
    LuminousIntensityDimension
);

// Metric prefixes
unit!(
    Millicandela,
    millicandelas,
    MILLI,
    0.0,
    "millicandela",
    "mcd",
    LuminousIntensityDimension
);
unit!(
    Kilocandela,
    kilocandelas,
    KILO,
    0.0,
    "kilocandela",
    "kcd",
    LuminousIntensityDimension
);

// Legacy photometric units
unit!(
    Hefnerkerze,
    hefnerkerzes,
    0.903,
    0.0,
    "hefnerkerze",
    "HK",
    LuminousIntensityDimension
);
unit!(
    InternationalCandle,
    international_candles,
    1.02,
    0.0,
    "international candle",
    "IC",
    LuminousIntensityDimension
);

// Practical lighting units
unit!(
    Footcandle,
    footcandles,
    10.764,
    0.0,
    "footcandle",
    "fc",
    LuminousIntensityDimension
);
