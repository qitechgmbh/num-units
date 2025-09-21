/// # Length Units - SI Length Measurements
///
/// This module defines SI length units and their conversions. Length is one of the
/// seven base dimensions in the SI system, with the meter as its base unit.
///
/// ## Base Unit
///
/// - **Meter (m)**: The SI base unit of length
///
use typenum::*;

// SI base unit
units! {
    Meter: "m", "meter";
}

units! {
    // Large SI prefix units
    Yottameter: "Ym", "yottameter";
    Zettameter: "Zm", "zettameter";
    Exameter: "Em", "exameter";
    Petameter: "Pm", "petameter";
    Terameter: "Tm", "terameter";
    Gigameter: "Gm", "gigameter";
    Megameter: "Mm", "megameter";
    Kilometer: "km", "kilometer";
    Hectometer: "hm", "hectometer";
    Decameter: "dam", "decameter";

    // Small SI prefix units
    Decimeter: "dm", "decimeter";
    Centimeter: "cm", "centimeter";
    Millimeter: "mm", "millimeter";
    Micrometer: "μm", "micrometer";
    Nanometer: "nm", "nanometer";
    Picometer: "pm", "picometer";
    Femtometer: "fm", "femtometer";
    Attometer: "am", "attometer";
    Zeptometer: "zm", "zeptometer";
    Yoctometer: "ym", "yoctometer";

    // Imperial and US customary units
    Foot: "ft", "foot";
    Inch: "in", "inch";
    Mile: "mi", "mile";
    Yard: "yd", "yard";
    Chain: "ch", "chain";
    Rod: "rd", "rod";
    Fathom: "fathom", "fathom";
    FootSurvey: "ft (U.S. survey)", "foot (U.S. survey)";
    MileSurvey: "mi (U.S. survey)", "mile (U.S. survey)";
    Mil: "0.001 in", "mil";
    Microinch: "μin", "microinch";

    // Scientific and specialized units
    Angstrom: "Å", "ångström";
    BohrRadius: "a₀", "bohr radius";
    AtomicUnitOfLength: "a.u. of length", "atomic unit of length";
    AstronomicalUnit: "ua", "astronomical unit";
    LightYear: "l. y.", "light year";
    Parsec: "pc", "parsec";
    Fermi: "fermi", "fermi";
    NauticalMile: "M", "nautical mile";
    Micron: "μ", "micron";

    // Typography units
    PicaComputer: "1/6 in (computer)", "pica (computer)";
    PicaPrinters: "1/6 in", "pica (printer's)";
    PointComputer: "1/72 in (computer)", "point (computer)";
    PointPrinters: "1/72 in", "point (printer's)";
}

// Meter is the SI base unit for length
// Using convert_linear! with derived units on the left, base unit on the right

use crate::prefix::{
    ATTO, CENTI, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI, NANO, PETA, PICO,
    TERA, YOCTO, YOTTA, ZEPTO, ZETTA,
};

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    // Large SI prefix units
    Yottameter => Meter: YOTTA;    // 1 Ym = 10^24 m
    Zettameter => Meter: ZETTA;    // 1 Zm = 10^21 m
    Exameter => Meter: EXA;        // 1 Em = 10^18 m
    Petameter => Meter: PETA;      // 1 Pm = 10^15 m
    Terameter => Meter: TERA;      // 1 Tm = 10^12 m
    Gigameter => Meter: GIGA;      // 1 Gm = 10^9 m
    Megameter => Meter: MEGA;      // 1 Mm = 10^6 m
    Kilometer => Meter: KILO;      // 1 km = 1000 m
    Hectometer => Meter: HECTO;    // 1 hm = 100 m
    Decameter => Meter: DECA;      // 1 dam = 10 m

    // Small SI prefix units
    Decimeter => Meter: DECI;      // 1 dm = 0.1 m
    Centimeter => Meter: CENTI;    // 1 cm = 0.01 m
    Millimeter => Meter: MILLI;    // 1 mm = 0.001 m
    Micrometer => Meter: MICRO;    // 1 μm = 0.000001 m
    Nanometer => Meter: NANO;      // 1 nm = 0.000000001 m
    Picometer => Meter: PICO;      // 1 pm = 10^-12 m
    Femtometer => Meter: FEMTO;    // 1 fm = 10^-15 m
    Attometer => Meter: ATTO;      // 1 am = 10^-18 m
    Zeptometer => Meter: ZEPTO;    // 1 zm = 10^-21 m
    Yoctometer => Meter: YOCTO;    // 1 ym = 10^-24 m

    // Imperial and US customary units
    Foot => Meter: 3.048E-1;       // 1 ft = 0.3048 m
    Inch => Meter: 2.54E-2;        // 1 in = 0.0254 m
    Mile => Meter: 1.609344E3;     // 1 mi = 1609.344 m
    Yard => Meter: 9.144E-1;       // 1 yd = 0.9144 m
    Chain => Meter: 2.011684E1;    // 1 ch = 20.11684 m
    Rod => Meter: 5.02921E0;       // 1 rd = 5.02921 m
    Fathom => Meter: 1.828804E0;   // 1 fathom = 1.828804 m
    FootSurvey => Meter: 3.048006E-1;  // 1 ft (U.S. survey) = 0.3048006 m
    MileSurvey => Meter: 1.609347E3;   // 1 mi (U.S. survey) = 1609.347 m
    Mil => Meter: 2.54E-5;         // 1 mil = 0.0000254 m
    Microinch => Meter: 2.54E-8;   // 1 μin = 0.0000000254 m

    // Scientific and specialized units
    Angstrom => Meter: 1.0E-10;               // 1 Å = 10^-10 m
    BohrRadius => Meter: 5.291772109030E-11;   // 1 a₀ = 5.291772... × 10^-11 m
    AtomicUnitOfLength => Meter: 5.291772109030E-11;  // Same as Bohr radius
    AstronomicalUnit => Meter: 1.495979E11;    // 1 ua = 1.495979 × 10^11 m (matches UOM exactly)
    LightYear => Meter: 9.46073E15;           // 1 l.y. = 9.46073 × 10^15 m
    Parsec => Meter: 3.085678E16;             // 1 pc = 3.085678 × 10^16 m
    Fermi => Meter: 1.0E-15;                  // 1 fermi = 10^-15 m
    NauticalMile => Meter: 1.852E3;           // 1 M = 1852 m
    Micron => Meter: 1.0E-6;                  // 1 μ = 10^-6 m (same as micrometer)

    // Typography units
    PicaComputer => Meter: 4.233333333333333E-3;  // 1/6 in (computer) = 4.233... × 10^-3 m
    PicaPrinters => Meter: 4.217518E-3;           // 1/6 in = 4.217518 × 10^-3 m
    PointComputer => Meter: 3.527778E-4;          // 1/72 in (computer) = 3.527778 × 10^-4 m
    PointPrinters => Meter: 3.514598E-4;          // 1/72 in = 3.514598 × 10^-4 m
}

convert_matrix! {
    Meter => Yottameter, Zettameter, Exameter, Petameter, Terameter, Gigameter, Megameter,
             Kilometer, Hectometer, Decameter, Decimeter, Centimeter, Millimeter,
             Micrometer, Nanometer, Picometer, Femtometer, Attometer, Zeptometer, Yoctometer,
             Foot, Inch, Mile, Yard, Chain, Rod, Fathom, FootSurvey, MileSurvey, Mil, Microinch,
             Angstrom, BohrRadius, AtomicUnitOfLength, AstronomicalUnit, LightYear, Parsec,
             Fermi, NauticalMile, Micron, PicaComputer, PicaPrinters, PointComputer, PointPrinters
}

// Length quantity definition
use super::{ISQ, SiScale};
quantity!(Length, ISQ<P1, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, Meter);

// Re-export types for convenience
pub use length::Length;
pub use length::*;

#[cfg(test)]
mod tests {
    macro_rules! test_uom_length {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::length,
                uom::si::length,
                Length,
                Length,
                Meter,
                $num_units_unit,
                meter,
                $uom_unit
            );
        };
    }

    // SI prefix units tests
    test_uom_length!(Yottameter, yottameter);
    test_uom_length!(Zettameter, zettameter);
    test_uom_length!(Exameter, exameter);
    test_uom_length!(Petameter, petameter);
    test_uom_length!(Terameter, terameter);
    test_uom_length!(Gigameter, gigameter);
    test_uom_length!(Megameter, megameter);
    test_uom_length!(Kilometer, kilometer);
    test_uom_length!(Hectometer, hectometer);
    test_uom_length!(Decameter, decameter);
    test_uom_length!(Decimeter, decimeter);
    test_uom_length!(Centimeter, centimeter);
    test_uom_length!(Millimeter, millimeter);
    test_uom_length!(Micrometer, micrometer);
    test_uom_length!(Nanometer, nanometer);
    test_uom_length!(Picometer, picometer);
    test_uom_length!(Femtometer, femtometer);
    test_uom_length!(Attometer, attometer);
    test_uom_length!(Zeptometer, zeptometer);
    test_uom_length!(Yoctometer, yoctometer);

    // Imperial and US customary units tests
    test_uom_length!(Foot, foot);
    test_uom_length!(Inch, inch);
    test_uom_length!(Mile, mile);
    test_uom_length!(Yard, yard);
    test_uom_length!(Chain, chain);
    test_uom_length!(Rod, rod);
    test_uom_length!(Fathom, fathom);
    test_uom_length!(FootSurvey, foot_survey);
    test_uom_length!(MileSurvey, mile_survey);
    test_uom_length!(Mil, mil);
    test_uom_length!(Microinch, microinch);

    // Scientific and specialized units tests
    test_uom_length!(Angstrom, angstrom);
    test_uom_length!(BohrRadius, bohr_radius);
    test_uom_length!(AtomicUnitOfLength, atomic_unit_of_length);
    test_uom_length!(AstronomicalUnit, astronomical_unit);
    test_uom_length!(LightYear, light_year);
    test_uom_length!(Parsec, parsec);
    test_uom_length!(Fermi, fermi);
    test_uom_length!(NauticalMile, nautical_mile);
    test_uom_length!(Micron, micron);

    // Typography units tests
    test_uom_length!(PicaComputer, pica_computer);
    test_uom_length!(PicaPrinters, pica_printers);
    test_uom_length!(PointComputer, point_computer);
    test_uom_length!(PointPrinters, point_printers);
}
