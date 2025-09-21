/// # Acceleration Units - SI Acceleration Measurements
///
/// This module defines SI acceleration units and their conversions. Acceleration is the rate
/// of change of velocity, with meter per second squared as the SI base unit.
///
/// ## Base Unit
///
/// - **MeterPerSecondSquared (m/s²)**: The SI base unit of acceleration
///
use typenum::*;

// SI base unit
units! {
    MeterPerSecondSquared: "m/s²", "meter per second squared";
}

units! {
    // SI prefix units
    YottameterPerSecondSquared: "Ym/s²", "yottameter per second squared";
    ZettameterPerSecondSquared: "Zm/s²", "zettameter per second squared";
    ExameterPerSecondSquared: "Em/s²", "exameter per second squared";
    PetameterPerSecondSquared: "Pm/s²", "petameter per second squared";
    TerameterPerSecondSquared: "Tm/s²", "terameter per second squared";
    GigameterPerSecondSquared: "Gm/s²", "gigameter per second squared";
    MegameterPerSecondSquared: "Mm/s²", "megameter per second squared";
    KilometerPerSecondSquared: "km/s²", "kilometer per second squared";
    HectometerPerSecondSquared: "hm/s²", "hectometer per second squared";
    DecameterPerSecondSquared: "dam/s²", "decameter per second squared";
    DecimeterPerSecondSquared: "dm/s²", "decimeter per second squared";
    CentimeterPerSecondSquared: "cm/s²", "centimeter per second squared";
    MillimeterPerSecondSquared: "mm/s²", "millimeter per second squared";
    MicrometerPerSecondSquared: "µm/s²", "micrometer per second squared";
    NanometerPerSecondSquared: "nm/s²", "nanometer per second squared";
    PicometerPerSecondSquared: "pm/s²", "picometer per second squared";
    FemtometerPerSecondSquared: "fm/s²", "femtometer per second squared";
    AttometerPerSecondSquared: "am/s²", "attometer per second squared";
    ZeptometerPerSecondSquared: "zm/s²", "zeptometer per second squared";
    YoctometerPerSecondSquared: "ym/s²", "yoctometer per second squared";

    // Other units
    FootPerSecondSquared: "ft/s²", "foot per second squared";
    Galileo: "Gal", "galileo";
    InchPerSecondSquared: "in/s²", "inch per second squared";
    MillimeterPerMinuteSquared: "mm/min²", "millimeter per minute squared";
    StandardGravity: "g₀", "standard acceleration of gravity";

    // Time combinations with minutes and hours
    // Per minute per second
    YottameterPerMinutePerSecond: "Ym/(min·s)", "yottameter per minute per second";
    ZettameterPerMinutePerSecond: "Zm/(min·s)", "zettameter per minute per second";
    ExameterPerMinutePerSecond: "Em/(min·s)", "exameter per minute per second";
    PetameterPerMinutePerSecond: "Pm/(min·s)", "petameter per minute per second";
    TerameterPerMinutePerSecond: "Tm/(min·s)", "terameter per minute per second";
    GigameterPerMinutePerSecond: "Gm/(min·s)", "gigameter per minute per second";
    MegameterPerMinutePerSecond: "Mm/(min·s)", "megameter per minute per second";
    KilometerPerMinutePerSecond: "km/(min·s)", "kilometer per minute per second";
    HectometerPerMinutePerSecond: "hm/(min·s)", "hectometer per minute per second";
    DecameterPerMinutePerSecond: "dam/(min·s)", "decameter per minute per second";
    MeterPerMinutePerSecond: "m/(min·s)", "meter per minute per second";
    DecimeterPerMinutePerSecond: "dm/(min·s)", "decimeter per minute per second";
    CentimeterPerMinutePerSecond: "cm/(min·s)", "centimeter per minute per second";
    MillimeterPerMinutePerSecond: "mm/(min·s)", "millimeter per minute per second";
    MicrometerPerMinutePerSecond: "µm/(min·s)", "micrometer per minute per second";
    NanometerPerMinutePerSecond: "nm/(min·s)", "nanometer per minute per second";
    PicometerPerMinutePerSecond: "pm/(min·s)", "picometer per minute per second";
    FemtometerPerMinutePerSecond: "fm/(min·s)", "femtometer per minute per second";
    AttometerPerMinutePerSecond: "am/(min·s)", "attometer per minute per second";
    ZeptometerPerMinutePerSecond: "zm/(min·s)", "zeptometer per minute per second";
    YoctometerPerMinutePerSecond: "ym/(min·s)", "yoctometer per minute per second";
    FootPerMinutePerSecond: "ft/(min·s)", "foot per minute per second";
    InchPerMinutePerSecond: "in/(min·s)", "inch per minute per second";

    // Per hour per second
    YottameterPerHourPerSecond: "Ym/(h·s)", "yottameter per hour per second";
    ZettameterPerHourPerSecond: "Zm/(h·s)", "zettameter per hour per second";
    ExameterPerHourPerSecond: "Em/(h·s)", "exameter per hour per second";
    PetameterPerHourPerSecond: "Pm/(h·s)", "petameter per hour per second";
    TerameterPerHourPerSecond: "Tm/(h·s)", "terameter per hour per second";
    GigameterPerHourPerSecond: "Gm/(h·s)", "gigameter per hour per second";
    MegameterPerHourPerSecond: "Mm/(h·s)", "megameter per hour per second";
    KilometerPerHourPerSecond: "km/(h·s)", "kilometer per hour per second";
    HectometerPerHourPerSecond: "hm/(h·s)", "hectometer per hour per second";
    DecameterPerHourPerSecond: "dam/(h·s)", "decameter per hour per second";
    MeterPerHourPerSecond: "m/(h·s)", "meter per hour per second";
    DecimeterPerHourPerSecond: "dm/(h·s)", "decimeter per hour per second";
    CentimeterPerHourPerSecond: "cm/(h·s)", "centimeter per hour per second";
    MillimeterPerHourPerSecond: "mm/(h·s)", "millimeter per hour per second";
    MicrometerPerHourPerSecond: "µm/(h·s)", "micrometer per hour per second";
    NanometerPerHourPerSecond: "nm/(h·s)", "nanometer per hour per second";
    PicometerPerHourPerSecond: "pm/(h·s)", "picometer per hour per second";
    FemtometerPerHourPerSecond: "fm/(h·s)", "femtometer per hour per second";
    AttometerPerHourPerSecond: "am/(h·s)", "attometer per hour per second";
    ZeptometerPerHourPerSecond: "zm/(h·s)", "zeptometer per hour per second";
    YoctometerPerHourPerSecond: "ym/(h·s)", "yoctometer per hour per second";
    FootPerHourPerSecond: "ft/(h·s)", "foot per hour per second";
    InchPerHourPerSecond: "in/(h·s)", "inch per hour per second";

    // Per minute squared
    YottameterPerMinuteSquared: "Ym/min²", "yottameter per minute squared";
    ZettameterPerMinuteSquared: "Zm/min²", "zettameter per minute squared";
    ExameterPerMinuteSquared: "Em/min²", "exameter per minute squared";
    PetameterPerMinuteSquared: "Pm/min²", "petameter per minute squared";
    TerameterPerMinuteSquared: "Tm/min²", "terameter per minute squared";
    GigameterPerMinuteSquared: "Gm/min²", "gigameter per minute squared";
    MegameterPerMinuteSquared: "Mm/min²", "megameter per minute squared";
    KilometerPerMinuteSquared: "km/min²", "kilometer per minute squared";
    HectometerPerMinuteSquared: "hm/min²", "hectometer per minute squared";
    DecameterPerMinuteSquared: "dam/min²", "decameter per minute squared";
    MeterPerMinuteSquared: "m/min²", "meter per minute squared";
    DecimeterPerMinuteSquared: "dm/min²", "decimeter per minute squared";
    CentimeterPerMinuteSquared: "cm/min²", "centimeter per minute squared";
    MicrometerPerMinuteSquared: "µm/min²", "micrometer per minute squared";
    NanometerPerMinuteSquared: "nm/min²", "nanometer per minute squared";
    PicometerPerMinuteSquared: "pm/min²", "picometer per minute squared";
    FemtometerPerMinuteSquared: "fm/min²", "femtometer per minute squared";
    AttometerPerMinuteSquared: "am/min²", "attometer per minute squared";
    ZeptometerPerMinuteSquared: "zm/min²", "zeptometer per minute squared";
    YoctometerPerMinuteSquared: "ym/min²", "yoctometer per minute squared";
    FootPerMinuteSquared: "ft/min²", "foot per minute squared";
    InchPerMinuteSquared: "in/min²", "inch per minute squared";

    // Per hour per minute
    YottameterPerHourPerMinute: "Ym/(h·min)", "yottameter per hour per minute";
    ZettameterPerHourPerMinute: "Zm/(h·min)", "zettameter per hour per minute";
    ExameterPerHourPerMinute: "Em/(h·min)", "exameter per hour per minute";
    PetameterPerHourPerMinute: "Pm/(h·min)", "petameter per hour per minute";
    TerameterPerHourPerMinute: "Tm/(h·min)", "terameter per hour per minute";
    GigameterPerHourPerMinute: "Gm/(h·min)", "gigameter per hour per minute";
    MegameterPerHourPerMinute: "Mm/(h·min)", "megameter per hour per minute";
    KilometerPerHourPerMinute: "km/(h·min)", "kilometer per hour per minute";
    HectometerPerHourPerMinute: "hm/(h·min)", "hectometer per hour per minute";
    DecameterPerHourPerMinute: "dam/(h·min)", "decameter per hour per minute";
    MeterPerHourPerMinute: "m/(h·min)", "meter per hour per minute";
    DecimeterPerHourPerMinute: "dm/(h·min)", "decimeter per hour per minute";
    CentimeterPerHourPerMinute: "cm/(h·min)", "centimeter per hour per minute";
    MillimeterPerHourPerMinute: "mm/(h·min)", "millimeter per hour per minute";
    MicrometerPerHourPerMinute: "µm/(h·min)", "micrometer per hour per minute";
    NanometerPerHourPerMinute: "nm/(h·min)", "nanometer per hour per minute";
    PicometerPerHourPerMinute: "pm/(h·min)", "picometer per hour per minute";
    FemtometerPerHourPerMinute: "fm/(h·min)", "femtometer per hour per minute";
    AttometerPerHourPerMinute: "am/(h·min)", "attometer per hour per minute";
    ZeptometerPerHourPerMinute: "zm/(h·min)", "zeptometer per hour per minute";
    YoctometerPerHourPerMinute: "ym/(h·min)", "yoctometer per hour per minute";
    FootPerHourPerMinute: "ft/(h·min)", "foot per hour per minute";
    InchPerHourPerMinute: "in/(h·min)", "inch per hour per minute";

    // Per hour squared
    YottameterPerHourSquared: "Ym/h²", "yottameter per hour squared";
    ZettameterPerHourSquared: "Zm/h²", "zettameter per hour squared";
    ExameterPerHourSquared: "Em/h²", "exameter per hour squared";
    PetameterPerHourSquared: "Pm/h²", "petameter per hour squared";
    TerameterPerHourSquared: "Tm/h²", "terameter per hour squared";
    GigameterPerHourSquared: "Gm/h²", "gigameter per hour squared";
    MegameterPerHourSquared: "Mm/h²", "megameter per hour squared";
    KilometerPerHourSquared: "km/h²", "kilometer per hour squared";
    HectometerPerHourSquared: "hm/h²", "hectometer per hour squared";
    DecameterPerHourSquared: "dam/h²", "decameter per hour squared";
    MeterPerHourSquared: "m/h²", "meter per hour squared";
    DecimeterPerHourSquared: "dm/h²", "decimeter per hour squared";
    CentimeterPerHourSquared: "cm/h²", "centimeter per hour squared";
    MillimeterPerHourSquared: "mm/h²", "millimeter per hour squared";
    MicrometerPerHourSquared: "µm/h²", "micrometer per hour squared";
    NanometerPerHourSquared: "nm/h²", "nanometer per hour squared";
    PicometerPerHourSquared: "pm/h²", "picometer per hour squared";
    FemtometerPerHourSquared: "fm/h²", "femtometer per hour squared";
    AttometerPerHourSquared: "am/h²", "attometer per hour squared";
    ZeptometerPerHourSquared: "zm/h²", "zeptometer per hour squared";
    YoctometerPerHourSquared: "ym/h²", "yoctometer per hour squared";
    FootPerHourSquared: "ft/h²", "foot per hour squared";
    InchPerHourSquared: "in/h²", "inch per hour squared";
}

// Import necessary prefixes
use crate::prefix::{
    ATTO, CENTI, DECA, DECI, EXA, FEMTO, GIGA, HECTO, HOUR, KILO, MEGA, MICRO, MILLI, MINUTE, NANO,
    PETA, PICO, TERA, YOCTO, YOTTA, ZEPTO, ZETTA,
};

crate::convert_linear! {
    // SI prefix units
    YottameterPerSecondSquared => MeterPerSecondSquared: YOTTA;
    ZettameterPerSecondSquared => MeterPerSecondSquared: ZETTA;
    ExameterPerSecondSquared => MeterPerSecondSquared: EXA;
    PetameterPerSecondSquared => MeterPerSecondSquared: PETA;
    TerameterPerSecondSquared => MeterPerSecondSquared: TERA;
    GigameterPerSecondSquared => MeterPerSecondSquared: GIGA;
    MegameterPerSecondSquared => MeterPerSecondSquared: MEGA;
    KilometerPerSecondSquared => MeterPerSecondSquared: KILO;
    HectometerPerSecondSquared => MeterPerSecondSquared: HECTO;
    DecameterPerSecondSquared => MeterPerSecondSquared: DECA;
    DecimeterPerSecondSquared => MeterPerSecondSquared: DECI;
    CentimeterPerSecondSquared => MeterPerSecondSquared: CENTI;
    MillimeterPerSecondSquared => MeterPerSecondSquared: MILLI;
    MicrometerPerSecondSquared => MeterPerSecondSquared: MICRO;
    NanometerPerSecondSquared => MeterPerSecondSquared: NANO;
    PicometerPerSecondSquared => MeterPerSecondSquared: PICO;
    FemtometerPerSecondSquared => MeterPerSecondSquared: FEMTO;
    AttometerPerSecondSquared => MeterPerSecondSquared: ATTO;
    ZeptometerPerSecondSquared => MeterPerSecondSquared: ZEPTO;
    YoctometerPerSecondSquared => MeterPerSecondSquared: YOCTO;

    // Other units
    FootPerSecondSquared => MeterPerSecondSquared: 3.048E-1;
    Galileo => MeterPerSecondSquared: 1.0E-2;
    InchPerSecondSquared => MeterPerSecondSquared: 2.54E-2;
    MillimeterPerMinuteSquared => MeterPerSecondSquared: 2.7777777777777778E-7;
    StandardGravity => MeterPerSecondSquared: 9.80665;

    // Time combinations - Per minute per second (m/(min·s) = m/s² × 60)
    YottameterPerMinutePerSecond => MeterPerSecondSquared: YOTTA * MINUTE;
    ZettameterPerMinutePerSecond => MeterPerSecondSquared: ZETTA * MINUTE;
    ExameterPerMinutePerSecond => MeterPerSecondSquared: EXA * MINUTE;
    PetameterPerMinutePerSecond => MeterPerSecondSquared: PETA * MINUTE;
    TerameterPerMinutePerSecond => MeterPerSecondSquared: TERA * MINUTE;
    GigameterPerMinutePerSecond => MeterPerSecondSquared: GIGA * MINUTE;
    MegameterPerMinutePerSecond => MeterPerSecondSquared: MEGA * MINUTE;
    KilometerPerMinutePerSecond => MeterPerSecondSquared: KILO * MINUTE;
    HectometerPerMinutePerSecond => MeterPerSecondSquared: HECTO * MINUTE;
    DecameterPerMinutePerSecond => MeterPerSecondSquared: DECA * MINUTE;
    MeterPerMinutePerSecond => MeterPerSecondSquared: MINUTE;
    DecimeterPerMinutePerSecond => MeterPerSecondSquared: DECI * MINUTE;
    CentimeterPerMinutePerSecond => MeterPerSecondSquared: CENTI * MINUTE;
    MillimeterPerMinutePerSecond => MeterPerSecondSquared: MILLI * MINUTE;
    MicrometerPerMinutePerSecond => MeterPerSecondSquared: MICRO * MINUTE;
    NanometerPerMinutePerSecond => MeterPerSecondSquared: NANO * MINUTE;
    PicometerPerMinutePerSecond => MeterPerSecondSquared: PICO * MINUTE;
    FemtometerPerMinutePerSecond => MeterPerSecondSquared: FEMTO * MINUTE;
    AttometerPerMinutePerSecond => MeterPerSecondSquared: ATTO * MINUTE;
    ZeptometerPerMinutePerSecond => MeterPerSecondSquared: ZEPTO * MINUTE;
    YoctometerPerMinutePerSecond => MeterPerSecondSquared: YOCTO * MINUTE;
    FootPerMinutePerSecond => MeterPerSecondSquared: 3.048E-1 * MINUTE;
    InchPerMinutePerSecond => MeterPerSecondSquared: 2.54E-2 * MINUTE;

    // Per hour per second (m/(h·s) = m/s² × 3600)
    YottameterPerHourPerSecond => MeterPerSecondSquared: YOTTA * HOUR;
    ZettameterPerHourPerSecond => MeterPerSecondSquared: ZETTA * HOUR;
    ExameterPerHourPerSecond => MeterPerSecondSquared: EXA * HOUR;
    PetameterPerHourPerSecond => MeterPerSecondSquared: PETA * HOUR;
    TerameterPerHourPerSecond => MeterPerSecondSquared: TERA * HOUR;
    GigameterPerHourPerSecond => MeterPerSecondSquared: GIGA * HOUR;
    MegameterPerHourPerSecond => MeterPerSecondSquared: MEGA * HOUR;
    KilometerPerHourPerSecond => MeterPerSecondSquared: KILO * HOUR;
    HectometerPerHourPerSecond => MeterPerSecondSquared: HECTO * HOUR;
    DecameterPerHourPerSecond => MeterPerSecondSquared: DECA * HOUR;
    MeterPerHourPerSecond => MeterPerSecondSquared: HOUR;
    DecimeterPerHourPerSecond => MeterPerSecondSquared: DECI * HOUR;
    CentimeterPerHourPerSecond => MeterPerSecondSquared: CENTI * HOUR;
    MillimeterPerHourPerSecond => MeterPerSecondSquared: MILLI * HOUR;
    MicrometerPerHourPerSecond => MeterPerSecondSquared: MICRO * HOUR;
    NanometerPerHourPerSecond => MeterPerSecondSquared: NANO * HOUR;
    PicometerPerHourPerSecond => MeterPerSecondSquared: PICO * HOUR;
    FemtometerPerHourPerSecond => MeterPerSecondSquared: FEMTO * HOUR;
    AttometerPerHourPerSecond => MeterPerSecondSquared: ATTO * HOUR;
    ZeptometerPerHourPerSecond => MeterPerSecondSquared: ZEPTO * HOUR;
    YoctometerPerHourPerSecond => MeterPerSecondSquared: YOCTO * HOUR;
    FootPerHourPerSecond => MeterPerSecondSquared: 3.048E-1 * HOUR;
    InchPerHourPerSecond => MeterPerSecondSquared: 2.54E-2 * HOUR;

    // Per minute squared (m/min² = m/s² / 3600)
    YottameterPerMinuteSquared => MeterPerSecondSquared: YOTTA / HOUR;
    ZettameterPerMinuteSquared => MeterPerSecondSquared: ZETTA / HOUR;
    ExameterPerMinuteSquared => MeterPerSecondSquared: EXA / HOUR;
    PetameterPerMinuteSquared => MeterPerSecondSquared: PETA / HOUR;
    TerameterPerMinuteSquared => MeterPerSecondSquared: TERA / HOUR;
    GigameterPerMinuteSquared => MeterPerSecondSquared: GIGA / HOUR;
    MegameterPerMinuteSquared => MeterPerSecondSquared: MEGA / HOUR;
    KilometerPerMinuteSquared => MeterPerSecondSquared: KILO / HOUR;
    HectometerPerMinuteSquared => MeterPerSecondSquared: HECTO / HOUR;
    DecameterPerMinuteSquared => MeterPerSecondSquared: DECA / HOUR;
    MeterPerMinuteSquared => MeterPerSecondSquared: 1.0 / HOUR;
    DecimeterPerMinuteSquared => MeterPerSecondSquared: DECI / HOUR;
    CentimeterPerMinuteSquared => MeterPerSecondSquared: CENTI / HOUR;
    MicrometerPerMinuteSquared => MeterPerSecondSquared: MICRO / HOUR;
    NanometerPerMinuteSquared => MeterPerSecondSquared: NANO / HOUR;
    PicometerPerMinuteSquared => MeterPerSecondSquared: PICO / HOUR;
    FemtometerPerMinuteSquared => MeterPerSecondSquared: FEMTO / HOUR;
    AttometerPerMinuteSquared => MeterPerSecondSquared: ATTO / HOUR;
    ZeptometerPerMinuteSquared => MeterPerSecondSquared: ZEPTO / HOUR;
    YoctometerPerMinuteSquared => MeterPerSecondSquared: YOCTO / HOUR;
    FootPerMinuteSquared => MeterPerSecondSquared: 3.048E-1 / HOUR;
    InchPerMinuteSquared => MeterPerSecondSquared: 2.54E-2 / HOUR;

    // Per hour per minute (m/(h·min) = m/s² × 60)
    YottameterPerHourPerMinute => MeterPerSecondSquared: YOTTA * MINUTE;
    ZettameterPerHourPerMinute => MeterPerSecondSquared: ZETTA * MINUTE;
    ExameterPerHourPerMinute => MeterPerSecondSquared: EXA * MINUTE;
    PetameterPerHourPerMinute => MeterPerSecondSquared: PETA * MINUTE;
    TerameterPerHourPerMinute => MeterPerSecondSquared: TERA * MINUTE;
    GigameterPerHourPerMinute => MeterPerSecondSquared: GIGA * MINUTE;
    MegameterPerHourPerMinute => MeterPerSecondSquared: MEGA * MINUTE;
    KilometerPerHourPerMinute => MeterPerSecondSquared: KILO * MINUTE;
    HectometerPerHourPerMinute => MeterPerSecondSquared: HECTO * MINUTE;
    DecameterPerHourPerMinute => MeterPerSecondSquared: DECA * MINUTE;
    MeterPerHourPerMinute => MeterPerSecondSquared: MINUTE;
    DecimeterPerHourPerMinute => MeterPerSecondSquared: DECI * MINUTE;
    CentimeterPerHourPerMinute => MeterPerSecondSquared: CENTI * MINUTE;
    MillimeterPerHourPerMinute => MeterPerSecondSquared: MILLI * MINUTE;
    MicrometerPerHourPerMinute => MeterPerSecondSquared: MICRO * MINUTE;
    NanometerPerHourPerMinute => MeterPerSecondSquared: NANO * MINUTE;
    PicometerPerHourPerMinute => MeterPerSecondSquared: PICO * MINUTE;
    FemtometerPerHourPerMinute => MeterPerSecondSquared: FEMTO * MINUTE;
    AttometerPerHourPerMinute => MeterPerSecondSquared: ATTO * MINUTE;
    ZeptometerPerHourPerMinute => MeterPerSecondSquared: ZEPTO * MINUTE;
    YoctometerPerHourPerMinute => MeterPerSecondSquared: YOCTO * MINUTE;
    FootPerHourPerMinute => MeterPerSecondSquared: 3.048E-1 * MINUTE;
    InchPerHourPerMinute => MeterPerSecondSquared: 2.54E-2 * MINUTE;

    // Per hour squared (m/h² = m/s² / 12960000)
    YottameterPerHourSquared => MeterPerSecondSquared: YOTTA / 12960000.0;
    ZettameterPerHourSquared => MeterPerSecondSquared: ZETTA / 12960000.0;
    ExameterPerHourSquared => MeterPerSecondSquared: EXA / 12960000.0;
    PetameterPerHourSquared => MeterPerSecondSquared: PETA / 12960000.0;
    TerameterPerHourSquared => MeterPerSecondSquared: TERA / 12960000.0;
    GigameterPerHourSquared => MeterPerSecondSquared: GIGA / 12960000.0;
    MegameterPerHourSquared => MeterPerSecondSquared: MEGA / 12960000.0;
    KilometerPerHourSquared => MeterPerSecondSquared: KILO / 12960000.0;
    HectometerPerHourSquared => MeterPerSecondSquared: HECTO / 12960000.0;
    DecameterPerHourSquared => MeterPerSecondSquared: DECA / 12960000.0;
    MeterPerHourSquared => MeterPerSecondSquared: 1.0 / 12960000.0;
    DecimeterPerHourSquared => MeterPerSecondSquared: DECI / 12960000.0;
    CentimeterPerHourSquared => MeterPerSecondSquared: CENTI / 12960000.0;
    MillimeterPerHourSquared => MeterPerSecondSquared: MILLI / 12960000.0;
    MicrometerPerHourSquared => MeterPerSecondSquared: MICRO / 12960000.0;
    NanometerPerHourSquared => MeterPerSecondSquared: NANO / 12960000.0;
    PicometerPerHourSquared => MeterPerSecondSquared: PICO / 12960000.0;
    FemtometerPerHourSquared => MeterPerSecondSquared: FEMTO / 12960000.0;
    AttometerPerHourSquared => MeterPerSecondSquared: ATTO / 12960000.0;
    ZeptometerPerHourSquared => MeterPerSecondSquared: ZEPTO / 12960000.0;
    YoctometerPerHourSquared => MeterPerSecondSquared: YOCTO / 12960000.0;
    FootPerHourSquared => MeterPerSecondSquared: 3.048E-1 / 12960000.0;
    InchPerHourSquared => MeterPerSecondSquared: 2.54E-2 / 12960000.0;
}

// Split into multiple convert_matrix! calls to avoid recursion limit
convert_matrix! {
    MeterPerSecondSquared => YottameterPerSecondSquared, ZettameterPerSecondSquared, ExameterPerSecondSquared,
        PetameterPerSecondSquared, TerameterPerSecondSquared, GigameterPerSecondSquared,
        MegameterPerSecondSquared, KilometerPerSecondSquared, HectometerPerSecondSquared,
        DecameterPerSecondSquared, DecimeterPerSecondSquared, CentimeterPerSecondSquared,
        MillimeterPerSecondSquared, MicrometerPerSecondSquared, NanometerPerSecondSquared,
        PicometerPerSecondSquared, FemtometerPerSecondSquared, AttometerPerSecondSquared,
        ZeptometerPerSecondSquared, YoctometerPerSecondSquared, FootPerSecondSquared,
        Galileo, InchPerSecondSquared, MillimeterPerMinuteSquared, StandardGravity, YottameterPerMinutePerSecond, ZettameterPerMinutePerSecond, ExameterPerMinutePerSecond,
        PetameterPerMinutePerSecond, TerameterPerMinutePerSecond, GigameterPerMinutePerSecond,
        MegameterPerMinutePerSecond, KilometerPerMinutePerSecond, HectometerPerMinutePerSecond,
        DecameterPerMinutePerSecond, MeterPerMinutePerSecond, DecimeterPerMinutePerSecond,
        CentimeterPerMinutePerSecond, MillimeterPerMinutePerSecond, MicrometerPerMinutePerSecond,
        NanometerPerMinutePerSecond, PicometerPerMinutePerSecond, FemtometerPerMinutePerSecond,
        AttometerPerMinutePerSecond, ZeptometerPerMinutePerSecond, YoctometerPerMinutePerSecond,
        FootPerMinutePerSecond, InchPerMinutePerSecond,
        YottameterPerHourPerSecond, ZettameterPerHourPerSecond, ExameterPerHourPerSecond,
        PetameterPerHourPerSecond, TerameterPerHourPerSecond, GigameterPerHourPerSecond,
        MegameterPerHourPerSecond, KilometerPerHourPerSecond, HectometerPerHourPerSecond,
        DecameterPerHourPerSecond, MeterPerHourPerSecond, DecimeterPerHourPerSecond,
        CentimeterPerHourPerSecond, MillimeterPerHourPerSecond, MicrometerPerHourPerSecond,
        NanometerPerHourPerSecond, PicometerPerHourPerSecond, FemtometerPerHourPerSecond,
        AttometerPerHourPerSecond, ZeptometerPerHourPerSecond, YoctometerPerHourPerSecond,
        FootPerHourPerSecond, InchPerHourPerSecond,
        YottameterPerMinuteSquared, ZettameterPerMinuteSquared, ExameterPerMinuteSquared,
        PetameterPerMinuteSquared, TerameterPerMinuteSquared, GigameterPerMinuteSquared,
        MegameterPerMinuteSquared, KilometerPerMinuteSquared, HectometerPerMinuteSquared,
        DecameterPerMinuteSquared, MeterPerMinuteSquared, DecimeterPerMinuteSquared,
        CentimeterPerMinuteSquared, MicrometerPerMinuteSquared, NanometerPerMinuteSquared,
        PicometerPerMinuteSquared, FemtometerPerMinuteSquared, AttometerPerMinuteSquared,
        ZeptometerPerMinuteSquared, YoctometerPerMinuteSquared, FootPerMinuteSquared,
        InchPerMinuteSquared,
        YottameterPerHourPerMinute, ZettameterPerHourPerMinute, ExameterPerHourPerMinute,
        PetameterPerHourPerMinute, TerameterPerHourPerMinute, GigameterPerHourPerMinute,
        MegameterPerHourPerMinute, KilometerPerHourPerMinute, HectometerPerHourPerMinute,
        DecameterPerHourPerMinute, MeterPerHourPerMinute, DecimeterPerHourPerMinute,
        CentimeterPerHourPerMinute, MillimeterPerHourPerMinute, MicrometerPerHourPerMinute,
        NanometerPerHourPerMinute, PicometerPerHourPerMinute, FemtometerPerHourPerMinute,
        AttometerPerHourPerMinute, ZeptometerPerHourPerMinute, YoctometerPerHourPerMinute,
        FootPerHourPerMinute, InchPerHourPerMinute,
        YottameterPerHourSquared, ZettameterPerHourSquared, ExameterPerHourSquared,
        PetameterPerHourSquared, TerameterPerHourSquared, GigameterPerHourSquared,
        MegameterPerHourSquared, KilometerPerHourSquared, HectometerPerHourSquared,
        DecameterPerHourSquared, MeterPerHourSquared, DecimeterPerHourSquared,
        CentimeterPerHourSquared, MillimeterPerHourSquared, MicrometerPerHourSquared,
        NanometerPerHourSquared, PicometerPerHourSquared, FemtometerPerHourSquared,
        AttometerPerHourSquared, ZeptometerPerHourSquared, YoctometerPerHourSquared,
        FootPerHourSquared, InchPerHourSquared
}

// Acceleration quantity definition (Length/Time²)
use super::{ISQ, SiScale};
quantity!(Acceleration, ISQ<P1, Z0, N2, Z0, Z0, Z0, Z0>, SiScale, MeterPerSecondSquared);

// Re-export types for convenience
pub use acceleration::Acceleration;
pub use acceleration::*;

#[cfg(test)]
mod tests {
    macro_rules! test_uom_acceleration {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::acceleration,
                uom::si::acceleration,
                Acceleration,
                Acceleration,
                MeterPerSecondSquared,
                $num_units_unit,
                meter_per_second_squared,
                $uom_unit
            );
        };
    }

    // Test SI prefix units
    test_uom_acceleration!(YottameterPerSecondSquared, yottameter_per_second_squared);
    test_uom_acceleration!(ZettameterPerSecondSquared, zettameter_per_second_squared);
    test_uom_acceleration!(ExameterPerSecondSquared, exameter_per_second_squared);
    test_uom_acceleration!(PetameterPerSecondSquared, petameter_per_second_squared);
    test_uom_acceleration!(TerameterPerSecondSquared, terameter_per_second_squared);
    test_uom_acceleration!(GigameterPerSecondSquared, gigameter_per_second_squared);
    test_uom_acceleration!(MegameterPerSecondSquared, megameter_per_second_squared);
    test_uom_acceleration!(KilometerPerSecondSquared, kilometer_per_second_squared);
    test_uom_acceleration!(HectometerPerSecondSquared, hectometer_per_second_squared);
    test_uom_acceleration!(DecameterPerSecondSquared, decameter_per_second_squared);
    test_uom_acceleration!(DecimeterPerSecondSquared, decimeter_per_second_squared);
    test_uom_acceleration!(CentimeterPerSecondSquared, centimeter_per_second_squared);
    test_uom_acceleration!(MillimeterPerSecondSquared, millimeter_per_second_squared);
    test_uom_acceleration!(MicrometerPerSecondSquared, micrometer_per_second_squared);
    test_uom_acceleration!(NanometerPerSecondSquared, nanometer_per_second_squared);
    test_uom_acceleration!(PicometerPerSecondSquared, picometer_per_second_squared);
    test_uom_acceleration!(FemtometerPerSecondSquared, femtometer_per_second_squared);
    test_uom_acceleration!(AttometerPerSecondSquared, attometer_per_second_squared);
    test_uom_acceleration!(ZeptometerPerSecondSquared, zeptometer_per_second_squared);
    test_uom_acceleration!(YoctometerPerSecondSquared, yoctometer_per_second_squared);

    // Test other units
    test_uom_acceleration!(FootPerSecondSquared, foot_per_second_squared);
    test_uom_acceleration!(Galileo, galileo);
    test_uom_acceleration!(InchPerSecondSquared, inch_per_second_squared);
    test_uom_acceleration!(MillimeterPerMinuteSquared, millimeter_per_minute_squared);
    test_uom_acceleration!(StandardGravity, standard_gravity);
}
