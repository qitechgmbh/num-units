use typenum::*;
use crate::{si::ISQ, unit};

pub type ThermodynamicTemperatureDimension = ISQ<Z0, Z0, Z0, Z0, P1, Z0, Z0>;

// SI base unit
unit!(
    Kelvin,
    kelvins,
    1.0,
    0.0,
    "kelvin",
    "K",
    ThermodynamicTemperatureDimension
);

// Common temperature scales with offset
unit!(
    Celsius,
    celsius,
    1.0,
    273.15,
    "celsius",
    "°C",
    ThermodynamicTemperatureDimension
);
unit!(
    Fahrenheit,
    fahrenheit,
    5.0 / 9.0,
    459.67 * 5.0 / 9.0,
    "fahrenheit",
    "°F",
    ThermodynamicTemperatureDimension
);

// Other temperature scales
unit!(
    Rankine,
    rankine,
    5.0 / 9.0,
    0.0,
    "rankine",
    "°R",
    ThermodynamicTemperatureDimension
);
unit!(
    Delisle,
    delisle,
    -2.0 / 3.0,
    559.725,
    "delisle",
    "°De",
    ThermodynamicTemperatureDimension
);
unit!(
    Newton,
    newton,
    100.0 / 33.0,
    273.15,
    "newton",
    "°N",
    ThermodynamicTemperatureDimension
);
unit!(
    Reaumur,
    reaumur,
    1.25,
    273.15,
    "réaumur",
    "°Ré",
    ThermodynamicTemperatureDimension
);
unit!(
    Romer,
    romer,
    40.0 / 21.0,
    273.15 - 7.5 * 40.0 / 21.0,
    "rømer",
    "°Rø",
    ThermodynamicTemperatureDimension
);
