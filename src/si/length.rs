use crate::prefix::{CENTI, KILO, MICRO, MILLI, NANO};

// SI base unit
base_units! {
    Meter: "meter", "m";
}

base_units! {
    Kilometer: "kilometer", "km";
    Centimeter: "centimeter", "cm";
    Millimeter: "millimeter", "mm";
    Micrometer: "micrometer", "μm";
    Nanometer: "nanometer", "nm";
}

convert_base_unit! {
    Kilometer: |meter| meter / KILO;
    Meter: |kilometer| kilometer * KILO;
}

convert_base_unit! {
    Centimeter: |meter| meter / CENTI;
    Meter: |centimeter| centimeter * CENTI;
}

convert_base_unit! {
    Millimeter: |meter| meter / MILLI;
    Meter: |millimeter| millimeter * MILLI;
}

convert_base_unit! {
    Micrometer: |meter| meter / MICRO;
    Meter: |micrometer| micrometer * MICRO;
}

convert_base_unit! {
    Nanometer: |meter| meter / NANO;
    Meter: |nanometer| nanometer * NANO;
}
