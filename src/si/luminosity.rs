use crate::{
    prefix::{KILO, MILLI},
    unit,
};

// All luminous intensity units
unit! {
    system: crate::si;
    quantity: crate::luminous_intensity;

    // SI base unit
    @candela: 1.0; "cd", "candela", "candelas";

    // Metric prefixes
    @millicandela: MILLI; "mcd", "millicandela", "millicandelas";
    @kilocandela: KILO; "kcd", "kilocandela", "kilocandelas";

    // Legacy photometric units
    @hefnerkerze: 0.903; "HK", "hefnerkerze", "hefnerkerzes";
    @international_candle: 1.02; "IC", "international candle", "international candles";

    // Practical lighting units
    @footcandle: 10.764; "fc", "footcandle", "footcandles";
}
