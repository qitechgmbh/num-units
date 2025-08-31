use crate::{
    prefix::{CENTI, KILO, MICRO, MILLI, NANO},
    si::SI,
    unit,
};

pub type LengthDimension = SI<1, 0, 0, 0, 0, 0, 0>;

// SI base unit
unit!(Meter, meters, 1.0, 0.0, "meter", "m", LengthDimension);

// Metric prefixes
unit!(
    Kilometer,
    kilometers,
    KILO,
    0.0,
    "kilometer",
    "km",
    LengthDimension
);
unit!(
    Centimeter,
    centimeters,
    CENTI,
    0.0,
    "centimeter",
    "cm",
    LengthDimension
);
unit!(
    Millimeter,
    millimeters,
    MILLI,
    0.0,
    "millimeter",
    "mm",
    LengthDimension
);
unit!(
    Micrometer,
    micrometers,
    MICRO,
    0.0,
    "micrometer",
    "μm",
    LengthDimension
);
unit!(
    Nanometer,
    nanometers,
    NANO,
    0.0,
    "nanometer",
    "nm",
    LengthDimension
);

// Imperial/US units
unit!(Foot, feet, 0.3048, 0.0, "foot", "ft", LengthDimension);
unit!(Inch, inches, 0.0254, 0.0, "inch", "in", LengthDimension);
unit!(Yard, yards, 0.9144, 0.0, "yard", "yd", LengthDimension);
unit!(Mile, miles, 1609.344, 0.0, "mile", "mi", LengthDimension);

// Nautical
unit!(
    NauticalMile,
    nautical_miles,
    1852.0,
    0.0,
    "nautical mile",
    "nmi",
    LengthDimension
);

// Astronomical
unit!(
    AstronomicalUnit,
    astronomical_units,
    149597870700.0,
    0.0,
    "astronomical unit",
    "AU",
    LengthDimension
);
unit!(
    LightYear,
    light_years,
    9460730472580800.0,
    0.0,
    "light year",
    "ly",
    LengthDimension
);
unit!(
    Parsec,
    parsecs,
    3.0857e16,
    0.0,
    "parsec",
    "pc",
    LengthDimension
);
