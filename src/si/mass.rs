use crate::{
    prefix::{KILO, MEGA, MICRO, MILLI},
    unit,
};

// All mass units
unit! {
    system: crate::si;
    quantity: crate::mass;

    // ===== SI/METRIC UNITS =====
    // SI base unit
    @gram: 1.0; "g", "gram", "grams";
    @kilogram: KILO; "kg", "kilogram", "kilograms";

    // Metric prefixes for gram
    @milligram: MILLI; "mg", "milligram", "milligrams";
    @microgram: MICRO; "μg", "microgram", "micrograms";

    // Large metric mass units
    @tonne: MEGA; "t", "tonne", "tonnes";

    // ===== US/IMPERIAL UNITS =====
    // Traditional US customary mass units
    @pound: 453.59237; "lb", "pound", "pounds";
    @ounce: 28.349523125; "oz", "ounce", "ounces";
    @stone: 6350.29318; "st", "stone", "stones";

    // ===== HISTORICAL WEIGHT UNITS =====
    // Traditional weight measurement systems
    @short_ton: 907184.74; "ton", "short ton", "short tons";
    @long_ton: 1016046.9088; "long_ton", "long ton", "long tons";

    // ===== PRECIOUS METAL UNITS =====
    // Troy weight system for precious metals
    @troy_ounce: 31.1034768; "oz_t", "troy ounce", "troy ounces";
    @troy_pound: 373.2417216; "lb_t", "troy pound", "troy pounds";

    // ===== SCIENTIFIC UNITS =====
    // Fundamental physics units
    @atomic_mass_unit: 1.66053906660e-27; "u", "atomic mass unit", "atomic mass units";
}
