use crate::{
    prefix::{MICRO, MILLI, NANO, PICO},
    si::ISQ,
    unit,
};
use typenum::*;

pub type TimeDimension = ISQ<Z0, Z0, P1, Z0, Z0, Z0, Z0>;

// SI base unit
unit!(Second, seconds, 1.0, 0.0, "second", "s", TimeDimension);

// Metric prefixes
unit!(
    Millisecond,
    milliseconds,
    MILLI,
    0.0,
    "millisecond",
    "ms",
    TimeDimension
);
unit!(
    Microsecond,
    microseconds,
    MICRO,
    0.0,
    "microsecond",
    "μs",
    TimeDimension
);
unit!(
    Nanosecond,
    nanoseconds,
    NANO,
    0.0,
    "nanosecond",
    "ns",
    TimeDimension
);
unit!(
    Picosecond,
    picoseconds,
    PICO,
    0.0,
    "picosecond",
    "ps",
    TimeDimension
);
unit!(
    Kilosecond,
    kiloseconds,
    1000.0,
    0.0,
    "kilosecond",
    "ks",
    TimeDimension
);

// Common time units
unit!(Minute, minutes, 60.0, 0.0, "minute", "min", TimeDimension);
unit!(Hour, hours, 3600.0, 0.0, "hour", "h", TimeDimension);
unit!(Day, days, 86400.0, 0.0, "day", "d", TimeDimension);
unit!(Week, weeks, 604800.0, 0.0, "week", "wk", TimeDimension);

// Longer periods
unit!(Year, years, 31557600.0, 0.0, "year", "yr", TimeDimension);
unit!(Month, months, 2629800.0, 0.0, "month", "mo", TimeDimension);

// Planck time (fundamental physics)
unit!(
    PlanckTime,
    planck_times,
    5.391247e-44,
    0.0,
    "planck time",
    "tₚ",
    TimeDimension
);
