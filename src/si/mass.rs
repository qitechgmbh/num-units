use crate::{
    prefix::{KILO, MEGA, MICRO, MILLI},
    si::SI,
    unit,
};

pub type MassDimension = SI<0, 1, 0, 0, 0, 0, 0>;

// SI base unit (grams)
unit!(Gram, grams, 1.0, 0.0, "gram", "g", MassDimension);

// Metric prefixes
unit!(
    Kilogram,
    kilograms,
    KILO,
    0.0,
    "kilogram",
    "kg",
    MassDimension
);
unit!(
    Milligram,
    milligrams,
    MILLI,
    0.0,
    "milligram",
    "mg",
    MassDimension
);
unit!(
    Microgram,
    micrograms,
    MICRO,
    0.0,
    "microgram",
    "μg",
    MassDimension
);
unit!(Tonne, tonnes, MEGA, 0.0, "tonne", "t", MassDimension);

// Imperial/US units (converted to grams)
unit!(Pound, pounds, 453.59237, 0.0, "pound", "lb", MassDimension);
unit!(
    Ounce,
    ounces,
    28.349523125,
    0.0,
    "ounce",
    "oz",
    MassDimension
);
unit!(Stone, stones, 6350.29318, 0.0, "stone", "st", MassDimension);
unit!(
    ShortTon,
    short_tons,
    907184.74,
    0.0,
    "short ton",
    "ton",
    MassDimension
);
unit!(
    LongTon,
    long_tons,
    1016046.9088,
    0.0,
    "long ton",
    "long_ton",
    MassDimension
);

// Troy system (precious metals) - converted to grams
unit!(
    TroyOunce,
    troy_ounces,
    31.1034768,
    0.0,
    "troy ounce",
    "oz_t",
    MassDimension
);
unit!(
    TroyPound,
    troy_pounds,
    373.2417216,
    0.0,
    "troy pound",
    "lb_t",
    MassDimension
);

// Atomic scale
unit!(
    AtomicMassUnit,
    atomic_mass_units,
    1.66053906660e-27,
    0.0,
    "atomic mass unit",
    "u",
    MassDimension
);
