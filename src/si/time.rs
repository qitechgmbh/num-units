use crate::{
    prefix::{MICRO, MILLI, NANO, PICO},
    unit,
};

// All time units
unit! {
    system: crate::si;
    quantity: crate::time;

    // SI base unit
    @second: 1.0; "s", "second", "seconds";

    // Metric prefixes
    @millisecond: MILLI; "ms", "millisecond", "milliseconds";
    @microsecond: MICRO; "μs", "microsecond", "microseconds";
    @nanosecond: NANO; "ns", "nanosecond", "nanoseconds";
    @picosecond: PICO; "ps", "picosecond", "picoseconds";
    @kilosecond: 1000.0; "ks", "kilosecond", "kiloseconds";

    // Common time units
    @minute: 60.0; "min", "minute", "minutes";
    @hour: 3600.0; "h", "hour", "hours";
    @day: 86400.0; "d", "day", "days";
    @week: 604800.0; "wk", "week", "weeks";

    // Longer periods
    @year: 31557600.0; "yr", "year", "years";
    @month: 2629800.0; "mo", "month", "months";

    // Planck time (fundamental physics)
    @planck_time: 5.391247e-44; "tₚ", "planck time", "planck times";
}
