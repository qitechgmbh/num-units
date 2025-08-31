use crate::unit;

// SI base unit and all other units
unit! {
    system: crate::si;
    quantity: crate::length;

    // ===== SI/METRIC UNITS =====
    // SI base unit
    @Meter: 1.0; "m", "meter", "meters";

    // Metric prefixes for meter
    @Kilometer: 1000.0; "km", "kilometer", "kilometers";
    @Centimeter: 0.01; "cm", "centimeter", "centimeters";
    @Millimeter: 0.001; "mm", "millimeter", "millimeters";
    @Micrometer: 0.000001; "μm", "micrometer", "micrometers";
    @Nanometer: 0.000000001; "nm", "nanometer", "nanometers";

    // Us/Imperial
    @Foot: 0.3048; "ft", "foot", "feet";
    @Inch: 0.0254; "in", "inch", "inches";
    @Yard: 0.9144; "yd", "yard", "yards";
    @Mile: 1609.344; "mi", "mile", "miles";

    // ===== NAUTICAL UNITS =====
    // Maritime navigation units
    @NauticalMile: 1852.0; "nmi", "nautical mile", "nautical miles";

    // Astronomical
    @AstronomicalUnit: 149597870700.0; "AU", "astronomical unit", "astronomical units";
    @LightYear: 9460730472580800.0; "ly", "light year", "light years";
    @Parsec: 30857000000000000.0; "pc", "parsec", "parsecs";
}
