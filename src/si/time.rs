use crate::prefix::*;
/// # Time Units - Time Duration Measurements
///
/// This module defines time units and their conversions. Time is measured in seconds
/// as the SI base unit, with various prefixed units and conventional time units.
///
/// ## Units
///
/// ### SI Base Unit
/// - **Second (s)**: The SI unit of time
///
/// ### SI Prefixed Units
/// - All SI prefixes from yocto to yotta applied to seconds
///
/// ### Conventional Time Units
/// - **Minute (min)**: 60 seconds
/// - **Hour (h)**: 3600 seconds
/// - **Day (d)**: 86400 seconds
/// - **Year (a)**: 31536000 seconds (365 days)
/// - **Shake**: 10 nanoseconds
/// - Sidereal variants: second_sidereal, day_sidereal, hour_sidereal, year_sidereal
/// - **Year_tropical**: Tropical year duration
use typenum::*;

// ===== SI BASE UNIT =====
units! {
    // SI prefixed seconds
    Yottasecond: "Ys", "yottasecond";
    Zettasecond: "Zs", "zettasecond";
    Exasecond: "Es", "exasecond";
    Petasecond: "Ps", "petasecond";
    Terasecond: "Ts", "terasecond";
    Gigasecond: "Gs", "gigasecond";
    Megasecond: "Ms", "megasecond";
    Kilosecond: "ks", "kilosecond";
    Hectosecond: "hs", "hectosecond";
    Decasecond: "das", "decasecond";
    Second: "s", "second";
    Decisecond: "ds", "decisecond";
    Centisecond: "cs", "centisecond";
    Millisecond: "ms", "millisecond";
    Microsecond: "µs", "microsecond";
    Nanosecond: "ns", "nanosecond";
    Picosecond: "ps", "picosecond";
    Femtosecond: "fs", "femtosecond";
    Attosecond: "as", "attosecond";
    Zeptosecond: "zs", "zeptosecond";
    Yoctosecond: "ys", "yoctosecond";

    // Conventional time units
    SecondSidereal: "s (sidereal)", "second (sidereal)";
    Minute: "min", "minute";
    Hour: "h", "hour";
    HourSidereal: "h (sidereal)", "hour (sidereal)";
    Day: "d", "day";
    DaySidereal: "d (sidereal)", "day (sidereal)";
    Shake: "10.0 ns", "shake";
    Year: "a", "year";
    YearSidereal: "a (sidereal)", "year (sidereal)";
    YearTropical: "a (tropical)", "year (tropical)";
}

// ===== CONVERSION RELATIONSHIPS =====

// SI prefixed units (linear conversions)
convert_linear! {
    Yottasecond => Second: YOTTA;
    Zettasecond => Second: ZETTA;
    Exasecond => Second: EXA;
    Petasecond => Second: PETA;
    Terasecond => Second: TERA;
    Gigasecond => Second: GIGA;
    Megasecond => Second: MEGA;
    Kilosecond => Second: KILO;
    Hectosecond => Second: HECTO;
    Decasecond => Second: DECA;
    Decisecond => Second: DECI;
    Centisecond => Second: CENTI;
    Millisecond => Second: MILLI;
    Microsecond => Second: MICRO;
    Nanosecond => Second: NANO;
    Picosecond => Second: PICO;
    Femtosecond => Second: FEMTO;
    Attosecond => Second: ATTO;
    Zeptosecond => Second: ZEPTO;
    Yoctosecond => Second: YOCTO;
}

// Conventional time units (exact UOM conversion factors)
convert_linear! {
    SecondSidereal => Second: 9.972696E-1;
    Minute => Second: MINUTE;
    Hour => Second: HOUR;
    HourSidereal => Second: 3.590170E3;
    Day => Second: 8.64E4;
    DaySidereal => Second: 8.616409E4;
    Shake => Second: 1.0E-8;
    Year => Second: 3.1536E7;
    YearSidereal => Second: 3.155815E7;
    YearTropical => Second: 3.155693E7;
}

convert_matrix! {
    Second => Yottasecond, Zettasecond, Exasecond, Petasecond, Terasecond, Gigasecond,
        Megasecond, Kilosecond, Hectosecond, Decasecond, Decisecond, Centisecond,
        Millisecond, Microsecond, Nanosecond, Picosecond, Femtosecond, Attosecond,
        Zeptosecond, Yoctosecond, SecondSidereal, Minute, Hour, HourSidereal, Day,
        DaySidereal, Shake, Year, YearSidereal, YearTropical
}

// Time quantity definition
use super::{ISQ, SiScale};
quantity!(Time, ISQ<Z0, Z0, P1, Z0, Z0, Z0, Z0>, SiScale, Second);

// Re-export types for convenience
pub use time::Time;
pub use time::*;

#[cfg(test)]
mod tests {

    macro_rules! test_uom_time {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::time,
                uom::si::time,
                Time,
                Time,
                Second,
                $num_units_unit,
                second,
                $uom_unit
            );
        };
    }

    // Test SI prefixed seconds
    test_uom_time!(Yottasecond, yottasecond);
    test_uom_time!(Zettasecond, zettasecond);
    test_uom_time!(Exasecond, exasecond);
    test_uom_time!(Petasecond, petasecond);
    test_uom_time!(Terasecond, terasecond);
    test_uom_time!(Gigasecond, gigasecond);
    test_uom_time!(Megasecond, megasecond);
    test_uom_time!(Kilosecond, kilosecond);
    test_uom_time!(Hectosecond, hectosecond);
    test_uom_time!(Decasecond, decasecond);
    test_uom_time!(Second, second);
    test_uom_time!(Decisecond, decisecond);
    test_uom_time!(Centisecond, centisecond);
    test_uom_time!(Millisecond, millisecond);
    test_uom_time!(Microsecond, microsecond);
    test_uom_time!(Nanosecond, nanosecond);
    test_uom_time!(Picosecond, picosecond);
    test_uom_time!(Femtosecond, femtosecond);
    test_uom_time!(Attosecond, attosecond);
    test_uom_time!(Zeptosecond, zeptosecond);
    test_uom_time!(Yoctosecond, yoctosecond);

    // Test conventional time units
    test_uom_time!(SecondSidereal, second_sidereal);
    test_uom_time!(Minute, minute);
    test_uom_time!(Hour, hour);
    test_uom_time!(HourSidereal, hour_sidereal);
    test_uom_time!(Day, day);
    test_uom_time!(DaySidereal, day_sidereal);
    test_uom_time!(Shake, shake);
    test_uom_time!(Year, year);
    test_uom_time!(YearSidereal, year_sidereal);
    test_uom_time!(YearTropical, year_tropical);
}
