/// # Mass Units - SI Mass Measurements
///
/// This module defines SI mass units and their conversions. Mass is one of the seven
/// base quantities in the SI system, with the kilogram as its base unit.
///
/// ## Base Unit
///
/// - **Kilogram (kg)**: The SI base unit of mass
///
/// ## SI Prefixed Units (Grams)
///
/// All SI prefixes from yocto- to yotta- are supported for grams:
/// - **Yottagram (Yg)**: 10²¹ grams = 10¹⁸ kg
/// - **Zettagram (Zg)**: 10¹⁸ grams = 10¹⁵ kg
/// - **Exagram (Eg)**: 10¹⁵ grams = 10¹² kg
/// - **Petagram (Pg)**: 10¹² grams = 10⁹ kg
/// - **Teragram (Tg)**: 10⁹ grams = 10⁶ kg
/// - **Gigagram (Gg)**: 10⁶ grams = 10³ kg
/// - **Megagram (Mg)**: 10³ grams = 10⁰ kg
/// - **Kilogram (kg)**: 10³ grams = Base unit
/// - **Hectogram (hg)**: 10² grams = 10⁻¹ kg
/// - **Decagram (dag)**: 10¹ grams = 10⁻² kg
/// - **Gram (g)**: Base gram unit = 10⁻³ kg
/// - **Decigram (dg)**: 10⁻¹ grams = 10⁻⁴ kg
/// - **Centigram (cg)**: 10⁻² grams = 10⁻⁵ kg
/// - **Milligram (mg)**: 10⁻³ grams = 10⁻⁶ kg
/// - **Microgram (μg)**: 10⁻⁶ grams = 10⁻⁹ kg
/// - **Nanogram (ng)**: 10⁻⁹ grams = 10⁻¹² kg
/// - **Picogram (pg)**: 10⁻¹² grams = 10⁻¹⁵ kg
/// - **Femtogram (fg)**: 10⁻¹⁵ grams = 10⁻¹⁸ kg
/// - **Attogram (ag)**: 10⁻¹⁸ grams = 10⁻²¹ kg
/// - **Zeptogram (zg)**: 10⁻²¹ grams = 10⁻²⁴ kg
/// - **Yoctogram (yg)**: 10⁻²⁴ grams = 10⁻²⁷ kg
///
/// ## Other Units
///
/// - **Carat (ct)**: 0.0002 kg (200 mg)
/// - **Dalton (Da)**: 1.66053906660 × 10⁻²⁷ kg (unified atomic mass unit)
/// - **Grain (gr)**: 6.479891 × 10⁻⁵ kg
/// - **HundredweightLong**: 50.80235 kg (long hundredweight)
/// - **HundredweightShort**: 45.35924 kg (short hundredweight)
/// - **Ounce (oz)**: 0.02834952 kg
/// - **OunceTroy (oz t)**: 0.03110348 kg
/// - **Pennyweight (dwt)**: 0.001555174 kg
/// - **Pound (lb)**: 0.4535924 kg
/// - **PoundTroy (lb t)**: 0.3732417 kg
/// - **Slug**: 14.59390 kg
/// - **TonAssay (AT)**: 0.02916667 kg
/// - **TonLong**: 1016.047 kg (2240 lb)
/// - **TonShort**: 907.1847 kg (2000 lb)
/// - **Ton (t)**: 1000 kg (metric ton)
///
/// ## Usage
///
/// ```rust,ignore
/// use num_units::mass::Mass;
/// use num_units::mass::{Kilogram, Gram, Pound, Ton};
///
/// // Create mass quantities
/// let mass = Mass::from::<Kilogram>(5.0);
/// let small_mass = Mass::from::<Gram>(500.0);
/// let imperial_mass = Mass::from::<Pound>(10.0);
/// let large_mass = Mass::from::<Ton>(2.0);
///
/// // Convert between units
/// let mass_g = mass.to::<Gram>();         // 5000.0 g
/// let small_mass_kg = small_mass.to::<Kilogram>(); // 0.5 kg
/// let imperial_mass_kg = imperial_mass.to::<Kilogram>(); // 4.535924 kg
/// let large_mass_kg = large_mass.to::<Kilogram>(); // 2000.0 kg
///
/// // Arithmetic operations
/// let total_mass = mass + small_mass;     // Automatic conversion
/// // let force = mass * acceleration;       // Creates force quantity
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All mass operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
use crate::prefix::{
    ATTO, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI, NANO, PETA, PICO, TERA,
    YOCTO, YOTTA, ZEPTO, ZETTA,
};
use typenum::*;

// SI base unit
units! {
    Kilogram: "kg", "kilogram";
}

// SI prefixed grams
units! {
    Yottagram: "Yg", "yottagram";
    Zettagram: "Zg", "zettagram";
    Exagram: "Eg", "exagram";
    Petagram: "Pg", "petagram";
    Teragram: "Tg", "teragram";
    Gigagram: "Gg", "gigagram";
    Megagram: "Mg", "megagram";
    Hectogram: "hg", "hectogram";
    Decagram: "dag", "decagram";
    Gram: "g", "gram";
    Decigram: "dg", "decigram";
    Centigram: "cg", "centigram";
    Milligram: "mg", "milligram";
    Microgram: "μg", "microgram";
    Nanogram: "ng", "nanogram";
    Picogram: "pg", "picogram";
    Femtogram: "fg", "femtogram";
    Attogram: "ag", "attogram";
    Zeptogram: "zg", "zeptogram";
    Yoctogram: "yg", "yoctogram";
}

// Other mass units
units! {
    Carat: "ct", "carat";
    Dalton: "Da", "dalton";
    Grain: "gr", "grain";
    HundredweightLong: "cwt long", "hundredweight (long)";
    HundredweightShort: "cwt short", "hundredweight (short)";
    Ounce: "oz", "ounce";
    OunceTroy: "oz t", "troy ounce";
    Pennyweight: "dwt", "pennyweight";
    Pound: "lb", "pound";
    PoundTroy: "lb t", "troy pound";
    Slug: "slug", "slug";
    TonAssay: "AT", "assay ton";
    TonLong: "2240 lb", "long ton";
    TonShort: "2000 lb", "short ton";
    Ton: "t", "ton";
}

// Unit conversions using convert_linear! with exact UOM coefficients
crate::convert_linear! {
    // SI prefixed grams - exact UOM coefficients (relative to kilogram)
    Yottagram => Kilogram: YOTTA / KILO;
    Zettagram => Kilogram: ZETTA / KILO;
    Exagram => Kilogram: EXA / KILO;
    Petagram => Kilogram: PETA / KILO;
    Teragram => Kilogram: TERA / KILO;
    Gigagram => Kilogram: GIGA / KILO;
    Megagram => Kilogram: MEGA / KILO;
    Hectogram => Kilogram: HECTO / KILO;
    Decagram => Kilogram: DECA / KILO;
    Gram => Kilogram: 1.0 / KILO;
    Decigram => Kilogram: DECI / KILO;
    Centigram => Kilogram: crate::prefix::CENTI / KILO;
    Milligram => Kilogram: MILLI / KILO;
    Microgram => Kilogram: MICRO / KILO;
    Nanogram => Kilogram: NANO / KILO;
    Picogram => Kilogram: PICO / KILO;
    Femtogram => Kilogram: FEMTO / KILO;
    Attogram => Kilogram: ATTO / KILO;
    Zeptogram => Kilogram: ZEPTO / KILO;
    Yoctogram => Kilogram: YOCTO / KILO;

    // Other units - exact UOM coefficients
    Carat => Kilogram: 2.0_E-4;
    Dalton => Kilogram: 1.660_539_066_60_E-27;
    Grain => Kilogram: 6.479_891_E-5;
    HundredweightLong => Kilogram: 5.080_235_E1;
    HundredweightShort => Kilogram: 4.535_924_E1;
    Ounce => Kilogram: 2.834_952_E-2;
    OunceTroy => Kilogram: 3.110_348_E-2;
    Pennyweight => Kilogram: 1.555_174_E-3;
    Pound => Kilogram: 4.535_924_E-1;
    PoundTroy => Kilogram: 3.732_417_E-1;
    Slug => Kilogram: 1.459_390_E1;
    TonAssay => Kilogram: 2.916_667_E-2;
    TonLong => Kilogram: 1.016_047_E3;
    TonShort => Kilogram: 9.071_847_E2;
    Ton => Kilogram: 1.0_E3;
}

crate::convert_matrix! {
    Kilogram => Yottagram, Zettagram, Exagram, Petagram, Teragram, Gigagram, Megagram, Hectogram, Decagram, Gram, Decigram, Centigram, Milligram, Microgram, Nanogram, Picogram, Femtogram, Attogram, Zeptogram, Yoctogram, Carat, Dalton, Grain, HundredweightLong, HundredweightShort, Ounce, OunceTroy, Pennyweight, Pound, PoundTroy, Slug, TonAssay, TonLong, TonShort, Ton
}

// Mass quantity definition (Mass is the 2nd base dimension)
use super::{ISQ, SiScale};
quantity!(Mass, ISQ<Z0, P1, Z0, Z0, Z0, Z0, Z0>, SiScale, Kilogram);

// UOM compatibility tests
#[cfg(test)]
mod tests {

    macro_rules! test_uom_mass {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::mass,
                uom::si::mass,
                Mass,
                Mass,
                Kilogram,
                $num_units_unit,
                kilogram,
                $uom_unit
            );
        };
    }

    // Test SI prefixed grams
    test_uom_mass!(Yottagram, yottagram);
    test_uom_mass!(Zettagram, zettagram);
    test_uom_mass!(Exagram, exagram);
    test_uom_mass!(Petagram, petagram);
    test_uom_mass!(Teragram, teragram);
    test_uom_mass!(Gigagram, gigagram);
    test_uom_mass!(Megagram, megagram);
    test_uom_mass!(Kilogram, kilogram);
    test_uom_mass!(Hectogram, hectogram);
    test_uom_mass!(Decagram, decagram);
    test_uom_mass!(Gram, gram);
    test_uom_mass!(Decigram, decigram);
    test_uom_mass!(Centigram, centigram);
    test_uom_mass!(Milligram, milligram);
    test_uom_mass!(Microgram, microgram);
    test_uom_mass!(Nanogram, nanogram);
    test_uom_mass!(Picogram, picogram);
    test_uom_mass!(Femtogram, femtogram);
    test_uom_mass!(Attogram, attogram);
    test_uom_mass!(Zeptogram, zeptogram);
    test_uom_mass!(Yoctogram, yoctogram);

    // Test other units
    test_uom_mass!(Carat, carat);
    test_uom_mass!(Dalton, dalton);
    test_uom_mass!(Grain, grain);
    test_uom_mass!(HundredweightLong, hundredweight_long);
    test_uom_mass!(HundredweightShort, hundredweight_short);
    test_uom_mass!(Ounce, ounce);
    test_uom_mass!(OunceTroy, ounce_troy);
    test_uom_mass!(Pennyweight, pennyweight);
    test_uom_mass!(Pound, pound);
    test_uom_mass!(PoundTroy, pound_troy);
    test_uom_mass!(Slug, slug);
    test_uom_mass!(TonAssay, ton_assay);
    test_uom_mass!(TonLong, ton_long);
    test_uom_mass!(TonShort, ton_short);
    test_uom_mass!(Ton, ton);
}

// Re-export types for convenience
pub use mass::Mass;
pub use mass::*;
