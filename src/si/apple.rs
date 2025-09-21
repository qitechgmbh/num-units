use crate::prefix::{NANO, MICRO, MILLI, DECI, CENTI, KILO, MEGA, GIGA};

// Apples
units! {
    Nanoapple: "n", "nanoapple";
    Microapple: "μ", "microapple";
    Milliapple: "m", "milliapple";
    Apple: "", "apple";
    Deciapple: "d", "deciapple";
    Centiapple: "c", "centiapple";
    Kiloapple: "k", "kiloapple";
    Megaapple: "M", "megaapple";
    Gigaapple: "G", "gigaapple";
}

// Apple unit conversion
crate::convert_linear! {
    Apple => Unitless: 1.0;                    // 1 apple = 1 unitless
    Milliapple => Unitless: MILLI;             // 1 milliapple = 0.001 apple
    Microapple => Unitless: MICRO;             // 1 microapple = 0.000001 apple
    Nanoapple => Unitless: NANO;               // 1 nanoapple = 0.000000001 apple
    Deciapple => Unitless: DECI;               // 1 deciapple = 0.1 apple
    Centiapple => Unitless: CENTI;             // 1 centiapple = 0.01 apple
    Kiloapple => Unitless: KILO;               // 1 kiloapple = 1000 apple
    Megaapple => Unitless: MEGA;               // 1 megaapple = 1,000,000 apple
    Gigaapple => Unitless: GIGA;               // 1 gigaapple = 1,000,000,000 apple
}

// Import Unitless from unitless module
use super::scalar::Unitless;
