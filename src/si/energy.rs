/// # Energy Units - SI Energy Measurements
///
/// This module defines SI energy units and their conversions. Energy is a derived quantity
/// in the SI system with dimensions of mass × length² × time⁻², with the joule as its base unit.
///
/// ## Base Unit
///
/// - **Joule (J)**: The SI derived unit of energy (kg·m²/s²)
///
/// ## SI Prefixed Units
///
/// All SI prefixes from yocto- to yotta- are supported for joules:
/// - **Yottajoule (YJ)**: 10²⁴ joules
/// - **Zettajoule (ZJ)**: 10²¹ joules
/// - **Exajoule (EJ)**: 10¹⁸ joules
/// - **Petajoule (PJ)**: 10¹⁵ joules
/// - **Terajoule (TJ)**: 10¹² joules
/// - **Gigajoule (GJ)**: 10⁹ joules
/// - **Megajoule (MJ)**: 10⁶ joules
/// - **Kilojoule (kJ)**: 10³ joules
/// - **Hectojoule (hJ)**: 10² joules
/// - **Decajoule (daJ)**: 10 joules
/// - **Joule (J)**: Base unit
/// - **Decijoule (dJ)**: 10⁻¹ joules
/// - **Centijoule (cJ)**: 10⁻² joules
/// - **Millijoule (mJ)**: 10⁻³ joules
/// - **Microjoule (μJ)**: 10⁻⁶ joules
/// - **Nanojoule (nJ)**: 10⁻⁹ joules
/// - **Picojoule (pJ)**: 10⁻¹² joules
/// - **Femtojoule (fJ)**: 10⁻¹⁵ joules
/// - **Attojoule (aJ)**: 10⁻¹⁸ joules
/// - **Zeptojoule (zJ)**: 10⁻²¹ joules
/// - **Yoctojoule (yJ)**: 10⁻²⁴ joules
///
/// ## Watt-Hour Units
///
/// Energy units based on power × time:
/// - **PetawattHour (PW·h)**: 3.6 × 10¹⁸ joules
/// - **TerawattHour (TW·h)**: 3.6 × 10¹⁵ joules
/// - **GigawattHour (GW·h)**: 3.6 × 10¹² joules
/// - **MegawattHour (MW·h)**: 3.6 × 10⁹ joules
/// - **KilowattHour (kW·h)**: 3.6 × 10⁶ joules
/// - **HectowattHour (hW·h)**: 3.6 × 10⁵ joules
/// - **DecawattHour (daW·h)**: 3.6 × 10⁴ joules
/// - **WattHour (W·h)**: 3.6 × 10³ joules
/// - **MilliwattHour (mW·h)**: 3.6 joules
/// - **MicrowattHour (μW·h)**: 3.6 × 10⁻³ joules
///
/// ## Electronvolt Units
///
/// Energy units used in particle physics:
/// - **Petaelectronvolt (PeV)**: 1.602176634 × 10⁻⁴ joules
/// - **Teraelectronvolt (TeV)**: 1.602176634 × 10⁻⁷ joules
/// - **Gigaelectronvolt (GeV)**: 1.602176634 × 10⁻¹⁰ joules
/// - **Megaelectronvolt (MeV)**: 1.602176634 × 10⁻¹³ joules
/// - **Kiloelectronvolt (keV)**: 1.602176634 × 10⁻¹⁶ joules
/// - **Hectoelectronvolt (heV)**: 1.602176634 × 10⁻¹⁷ joules
/// - **Decaelectronvolt (daeV)**: 1.602176634 × 10⁻¹⁸ joules
/// - **Electronvolt (eV)**: 1.602176634 × 10⁻¹⁹ joules
///
/// ## Other Units
///
/// - **Hartree (Eₕ)**: 4.3597447222071 × 10⁻¹⁸ joules (atomic unit)
/// - **British Thermal Units (BTU)**: Various definitions
/// - **Calories**: Various definitions
/// - **Erg**: 10⁻⁷ joules (CGS unit)
/// - **Foot Poundal**: 4.214011 × 10⁻² joules
/// - **Foot Pound**: 1.355818 joules
/// - **Quad**: 1.055056 × 10¹⁸ joules
/// - **Therm**: Various definitions
/// - **Ton of TNT**: 4.184 × 10⁹ joules
/// - **Watt Second**: 1 joule
///
/// ## Usage
///
/// ```rust,ignore
/// use num_units::energy::Energy;
/// use num_units::energy::{Joule, KilowattHour, Calorie};
///
/// // Create energy quantities
/// let energy = Energy::from::<Joule>(1000.0);
/// let power_energy = Energy::from::<KilowattHour>(1.0);
/// let food_energy = Energy::from::<Calorie>(500.0);
///
/// // Convert between units
/// let energy_kwh = energy.to::<KilowattHour>();    // 0.0002778 kW·h
/// let power_energy_j = power_energy.to::<Joule>(); // 3600000.0 J
/// let food_energy_j = food_energy.to::<Joule>();   // 2092.0 J
///
/// // Arithmetic operations
/// let total_energy = energy + power_energy;        // Automatic conversion
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All energy operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
use crate::prefix::{
    ATTO, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI, NANO, PETA, PICO, TERA,
    YOCTO, YOTTA, ZEPTO, ZETTA,
};
use typenum::*;

// SI base unit
units! {
    Joule: "J", "joule";
}

// SI prefixed joules
units! {
    Yottajoule: "YJ", "yottajoule";
    Zettajoule: "ZJ", "zettajoule";
    Exajoule: "EJ", "exajoule";
    Petajoule: "PJ", "petajoule";
    Terajoule: "TJ", "terajoule";
    Gigajoule: "GJ", "gigajoule";
    Megajoule: "MJ", "megajoule";
    Kilojoule: "kJ", "kilojoule";
    Hectojoule: "hJ", "hectojoule";
    Decajoule: "daJ", "decajoule";
    Decijoule: "dJ", "decijoule";
    Centijoule: "cJ", "centijoule";
    Millijoule: "mJ", "millijoule";
    Microjoule: "μJ", "microjoule";
    Nanojoule: "nJ", "nanojoule";
    Picojoule: "pJ", "picojoule";
    Femtojoule: "fJ", "femtojoule";
    Attojoule: "aJ", "attojoule";
    Zeptojoule: "zJ", "zeptojoule";
    Yoctojoule: "yJ", "yoctojoule";
}

// Watt-hour units
units! {
    PetawattHour: "PW·h", "petawatt hour";
    TerawattHour: "TW·h", "terawatt hour";
    GigawattHour: "GW·h", "gigawatt hour";
    MegawattHour: "MW·h", "megawatt hour";
    KilowattHour: "kW·h", "kilowatt hour";
    HectowattHour: "hW·h", "hectowatt hour";
    DecawattHour: "daW·h", "decawatt hour";
    WattHour: "W·h", "watt hour";
    MilliwattHour: "mW·h", "milliwatt hour";
    MicrowattHour: "μW·h", "microwatt hour";
}

// Electronvolt units
units! {
    Petaelectronvolt: "PeV", "petaelectronvolt";
    Teraelectronvolt: "TeV", "teraelectronvolt";
    Gigaelectronvolt: "GeV", "gigaelectronvolt";
    Megaelectronvolt: "MeV", "megaelectronvolt";
    Kiloelectronvolt: "keV", "kiloelectronvolt";
    Hectoelectronvolt: "heV", "hectoelectronvolt";
    Decaelectronvolt: "daeV", "decaelectronvolt";
    Electronvolt: "eV", "electronvolt";
}

// Other energy units
units! {
    Hartree: "Eₕ", "hartree";
    BtuIt: "Btu (IT)", "British thermal unit (IT)";
    Btu: "Btu", "British thermal unit";
    Btu39: "Btu₃₉", "British thermal unit (39 °F)";
    Btu59: "Btu₅₉", "British thermal unit (59 °F)";
    Btu60: "Btu₆₀", "British thermal unit (60 °F)";
    CalorieIt: "cal (IT)", "calorie (IT)";
    Calorie: "cal", "calorie";
    Calorie15: "cal₁₅", "calorie (15 °C)";
    Calorie20: "cal₂₀", "calorie (20 °C)";
    CalorieItNutrition: "Cal (IT)", "Calorie (IT)";
    CalorieNutrition: "Cal", "Calorie";
    Erg: "erg", "erg";
    FootPoundal: "ft·pdl", "foot poundal";
    FootPound: "ft·lbf", "foot pound-force";
    KilocalorieIt: "kcal (IT)", "kilocalorie (IT)";
    Kilocalorie: "kcal", "kilocalorie";
    Quad: "10¹⁵ Btu (IT)", "quad";
    ThermEc: "thm (EC)", "therm (EC)";
    ThermUs: "thm", "therm";
    TonTnt: "t of TNT", "ton of TNT";
    WattSecond: "W·s", "watt second";
}

// ===== CONVERSION RELATIONSHIPS =====

// Unit conversions using convert_linear! with exact UOM coefficients
crate::convert_linear! {
    // SI prefixed joules
    Yottajoule => Joule: YOTTA;
    Zettajoule => Joule: ZETTA;
    Exajoule => Joule: EXA;
    Petajoule => Joule: PETA;
    Terajoule => Joule: TERA;
    Gigajoule => Joule: GIGA;
    Megajoule => Joule: MEGA;
    Kilojoule => Joule: KILO;
    Hectojoule => Joule: HECTO;
    Decajoule => Joule: DECA;
    Decijoule => Joule: DECI;
    Centijoule => Joule: crate::prefix::CENTI;
    Millijoule => Joule: MILLI;
    Microjoule => Joule: MICRO;
    Nanojoule => Joule: NANO;
    Picojoule => Joule: PICO;
    Femtojoule => Joule: FEMTO;
    Attojoule => Joule: ATTO;
    Zeptojoule => Joule: ZEPTO;
    Yoctojoule => Joule: YOCTO;

    // Watt-hour units - exact UOM coefficients
    PetawattHour => Joule: 3.6_E18;
    TerawattHour => Joule: 3.6_E15;
    GigawattHour => Joule: 3.6_E12;
    MegawattHour => Joule: 3.6_E9;
    KilowattHour => Joule: 3.6_E6;
    HectowattHour => Joule: 3.6_E5;
    DecawattHour => Joule: 3.6_E4;
    WattHour => Joule: 3.6_E3;
    MilliwattHour => Joule: 3.6_E0;
    MicrowattHour => Joule: 3.6_E-3;

    // Electronvolt units - exact UOM coefficients
    Petaelectronvolt => Joule: 1.602_176_634_E-4;
    Teraelectronvolt => Joule: 1.602_176_634_E-7;
    Gigaelectronvolt => Joule: 1.602_176_634_E-10;
    Megaelectronvolt => Joule: 1.602_176_634_E-13;
    Kiloelectronvolt => Joule: 1.602_176_634_E-16;
    Hectoelectronvolt => Joule: 1.602_176_634_E-17;
    Decaelectronvolt => Joule: 1.602_176_634_E-18;
    Electronvolt => Joule: 1.602_176_634_E-19;

    // Other units - exact UOM coefficients
    Hartree => Joule: 4.359_744_722_207_1_E-18;
    BtuIt => Joule: 1.055_056_E3;
    Btu => Joule: 1.054_350_E3;
    Btu39 => Joule: 1.059_67_E3;
    Btu59 => Joule: 1.054_80_E3;
    Btu60 => Joule: 1.054_68_E3;
    CalorieIt => Joule: 4.186_8_E0;
    Calorie => Joule: 4.184_E0;
    Calorie15 => Joule: 4.185_80_E0;
    Calorie20 => Joule: 4.181_90_E0;
    CalorieItNutrition => Joule: 4.186_8_E3;
    CalorieNutrition => Joule: 4.184_E3;
    Erg => Joule: 1.0_E-7;
    FootPoundal => Joule: 4.214_011_E-2;
    FootPound => Joule: 1.355_818_E0;
    KilocalorieIt => Joule: 4.186_8_E3;
    Kilocalorie => Joule: 4.184_E3;
    Quad => Joule: 1.055_056_E18;
    ThermEc => Joule: 1.055_06_E8;
    ThermUs => Joule: 1.054_804_E8;
    TonTnt => Joule: 4.184_E9;
    WattSecond => Joule: 1.0_E0;
}

crate::convert_matrix! {
    Joule => Yottajoule, Zettajoule, Exajoule, Petajoule, Terajoule, Gigajoule, Megajoule, Kilojoule, Hectojoule, Decajoule, Decijoule, Centijoule, Millijoule, Microjoule, Nanojoule, Picojoule, Femtojoule, Attojoule, Zeptojoule, Yoctojoule, PetawattHour, TerawattHour, GigawattHour, MegawattHour, KilowattHour, HectowattHour, DecawattHour, WattHour, MilliwattHour, MicrowattHour, Petaelectronvolt, Teraelectronvolt, Gigaelectronvolt, Megaelectronvolt, Kiloelectronvolt, Hectoelectronvolt, Decaelectronvolt, Electronvolt, Hartree, BtuIt, Btu, Btu39, Btu59, Btu60, CalorieIt, Calorie, Calorie15, Calorie20, CalorieItNutrition, CalorieNutrition, Erg, FootPoundal, FootPound, KilocalorieIt, Kilocalorie, Quad, ThermEc, ThermUs, TonTnt, WattSecond
}

// Energy quantity definition (Mass×Length²/Time²)
use super::{ISQ, SiScale};
quantity!(Energy, ISQ<P2, P1, N2, Z0, Z0, Z0, Z0>, SiScale, Joule);

// UOM compatibility tests
#[cfg(test)]
mod tests {

    macro_rules! test_uom_energy {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::energy,
                uom::si::energy,
                Energy,
                Energy,
                Joule,
                $num_units_unit,
                joule,
                $uom_unit
            );
        };
    }

    // Test SI prefixed joules
    test_uom_energy!(Yottajoule, yottajoule);
    test_uom_energy!(Zettajoule, zettajoule);
    test_uom_energy!(Exajoule, exajoule);
    test_uom_energy!(Petajoule, petajoule);
    test_uom_energy!(Terajoule, terajoule);
    test_uom_energy!(Gigajoule, gigajoule);
    test_uom_energy!(Megajoule, megajoule);
    test_uom_energy!(Kilojoule, kilojoule);
    test_uom_energy!(Hectojoule, hectojoule);
    test_uom_energy!(Decajoule, decajoule);
    test_uom_energy!(Joule, joule);
    test_uom_energy!(Decijoule, decijoule);
    test_uom_energy!(Centijoule, centijoule);
    test_uom_energy!(Millijoule, millijoule);
    test_uom_energy!(Microjoule, microjoule);
    test_uom_energy!(Nanojoule, nanojoule);
    test_uom_energy!(Picojoule, picojoule);
    test_uom_energy!(Femtojoule, femtojoule);
    test_uom_energy!(Attojoule, attojoule);
    test_uom_energy!(Zeptojoule, zeptojoule);
    test_uom_energy!(Yoctojoule, yoctojoule);

    // Test watt-hour units
    test_uom_energy!(PetawattHour, petawatt_hour);
    test_uom_energy!(TerawattHour, terawatt_hour);
    test_uom_energy!(GigawattHour, gigawatt_hour);
    test_uom_energy!(MegawattHour, megawatt_hour);
    test_uom_energy!(KilowattHour, kilowatt_hour);
    test_uom_energy!(HectowattHour, hectowatt_hour);
    test_uom_energy!(DecawattHour, decawatt_hour);
    test_uom_energy!(WattHour, watt_hour);
    test_uom_energy!(MilliwattHour, milliwatt_hour);
    test_uom_energy!(MicrowattHour, microwatt_hour);

    // Test electronvolt units
    test_uom_energy!(Petaelectronvolt, petaelectronvolt);
    test_uom_energy!(Teraelectronvolt, teraelectronvolt);
    test_uom_energy!(Gigaelectronvolt, gigaelectronvolt);
    test_uom_energy!(Megaelectronvolt, megaelectronvolt);
    test_uom_energy!(Kiloelectronvolt, kiloelectronvolt);
    test_uom_energy!(Hectoelectronvolt, hectoelectronvolt);
    test_uom_energy!(Decaelectronvolt, decaelectronvolt);
    test_uom_energy!(Electronvolt, electronvolt);

    // Test other units
    test_uom_energy!(Hartree, hartree);
    test_uom_energy!(BtuIt, btu_it);
    test_uom_energy!(Btu, btu);
    test_uom_energy!(Btu39, btu_39);
    test_uom_energy!(Btu59, btu_59);
    test_uom_energy!(Btu60, btu_60);
    test_uom_energy!(CalorieIt, calorie_it);
    test_uom_energy!(Calorie, calorie);
    test_uom_energy!(Calorie15, calorie_15);
    test_uom_energy!(Calorie20, calorie_20);
    test_uom_energy!(CalorieItNutrition, calorie_it_nutrition);
    test_uom_energy!(CalorieNutrition, calorie_nutrition);
    test_uom_energy!(Erg, erg);
    test_uom_energy!(FootPoundal, foot_poundal);
    test_uom_energy!(FootPound, foot_pound);
    test_uom_energy!(KilocalorieIt, kilocalorie_it);
    test_uom_energy!(Kilocalorie, kilocalorie);
    test_uom_energy!(Quad, quad);
    test_uom_energy!(ThermEc, therm_ec);
    test_uom_energy!(ThermUs, therm_us);
    test_uom_energy!(TonTnt, ton_tnt);
    test_uom_energy!(WattSecond, watt_second);
}

// Re-export types for convenience
pub use energy::Energy;
pub use energy::*;
