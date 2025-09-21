/// # Amount of Substance Units - SI Amount of Substance Measurements
///
/// This module defines SI amount of substance units and their conversions. Amount of substance
/// is one of the seven base dimensions in the SI system, with the mole as its base unit.
///
/// ## Base Unit
///
/// - **Mole (mol)**: The SI base unit of amount of substance
///
/// ## SI Prefix Units
///
/// All SI prefixes from yotta to yocto are supported:
/// - **Yottamole (Ymol)**: 10²⁴ moles
/// - **Zettamole (Zmol)**: 10²¹ moles
/// - **Examole (Emol)**: 10¹⁸ moles
/// - ... down to ...
/// - **Yoctomole (ymol)**: 10⁻²⁴ moles
///
/// ## Particle Counting
///
/// - **Particle**: 1 / N_A elementary entities (where N_A is Avogadro's number)
///
/// ## Standard Gas Volumes
///
/// Units representing the amount of ideal gas at standard temperature (0°C) and pressure (1 bar):
/// - **StandardCubicMeter (m³(STP))**: Amount in 1 m³ at STP
/// - **StandardLiter (L(STP))**: Amount in 1 L at STP
/// - **StandardCubicCentimeter (cm³(STP))**: Amount in 1 cm³ at STP
/// - **StandardCubicFoot (scf)**: Amount in 1 ft³ at STP
///
use crate::prefix::*;
use typenum::*;

// SI base unit
units! {
    Mole: "mol", "mole";
}

units! {
    // SI prefix units
    Yottamole: "Ymol", "yottamole";
    Zettamole: "Zmol", "zettamole";
    Examole: "Emol", "examole";
    Petamole: "Pmol", "petamole";
    Teramole: "Tmol", "teramole";
    Gigamole: "Gmol", "gigamole";
    Megamole: "Mmol", "megamole";
    Kilomole: "kmol", "kilomole";
    Hectomole: "hmol", "hectomole";
    Decamole: "damol", "decamole";
    Decimole: "dmol", "decimole";
    Centimole: "cmol", "centimole";
    Millimole: "mmol", "millimole";
    Micromole: "μmol", "micromole";
    Nanomole: "nmol", "nanomole";
    Picomole: "pmol", "picomole";
    Femtumole: "fmol", "femtomole";
    Attomole: "amol", "attomole";
    Zeptomole: "zmol", "zeptomole";
    Yoctomole: "ymol", "yoctomole";

    // Particle counting
    Particle: "particle", "particle";

    // Standard gas volumes (at 0°C and 1 bar)
    StandardCubicMeter: "m³(STP)", "standard cubic meter";
    StandardLiter: "L(STP)", "standard liter";
    StandardCubicCentimeter: "cm³(STP)", "standard cubic centimeter";
    StandardCubicFoot: "scf", "standard cubic foot";
}

// Mole is the SI base unit for amount of substance
// Using convert_linear! with derived units on the left, base unit on the right

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    // SI prefix units
    Yottamole => Mole: YOTTA;
    Zettamole => Mole: ZETTA;
    Examole => Mole: EXA;
    Petamole => Mole: PETA;
    Teramole => Mole: TERA;
    Gigamole => Mole: GIGA;
    Megamole => Mole: MEGA;
    Kilomole => Mole: KILO;
    Hectomole => Mole: HECTO;
    Decamole => Mole: DECA;
    Decimole => Mole: DECI;
    Centimole => Mole: CENTI;
    Millimole => Mole: MILLI;
    Micromole => Mole: MICRO;
    Nanomole => Mole: NANO;
    Picomole => Mole: PICO;
    Femtumole => Mole: FEMTO;
    Attomole => Mole: ATTO;
    Zeptomole => Mole: ZEPTO;
    Yoctomole => Mole: YOCTO;

    // Particle counting (1 particle = 1 / N_A moles, where N_A is Avogadro's number)
    Particle => Mole: 1.0 / 6.02214076e23;

    // Standard gas volumes (ideal gas at 0°C and 1 bar)
    // Volume * pressure / (R * T) = n, where R = 8.314462618 J/(mol·K), T = 273.15 K, P = 1e5 Pa
    StandardCubicMeter => Mole: 1E5 / 8.314462618 / 273.15;
    StandardLiter => Mole: 1E5 * MILLI / 8.314462618 / 273.15;
    StandardCubicCentimeter => Mole: 1E5 * MICRO / 8.314462618 / 273.15;
    StandardCubicFoot => Mole: 1E5 * 2.831685E-2 / 8.314462618 / 273.15;
}

convert_matrix! {
    Mole => Yottamole, Zettamole, Examole, Petamole, Teramole, Gigamole, Megamole, Kilomole,
        Hectomole, Decamole, Decimole, Centimole, Millimole, Micromole, Nanomole, Picomole,
        Femtumole, Attomole, Zeptomole, Yoctomole, Particle, StandardCubicMeter, StandardLiter,
        StandardCubicCentimeter, StandardCubicFoot
}

// Amount quantity definition
use super::{ISQ, SiScale};
quantity!(Amount, ISQ<Z0, Z0, Z0, Z0, Z0, P1, Z0>, SiScale, Mole);

// Re-export types for convenience
pub use amount::Amount;
pub use amount::*;

#[cfg(test)]
mod tests {
    macro_rules! test_uom_amount {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::amount,
                uom::si::amount_of_substance,
                Amount,
                AmountOfSubstance,
                Mole,
                $num_units_unit,
                mole,
                $uom_unit
            );
        };
    }

    // Test SI prefix units
    test_uom_amount!(Yottamole, yottamole);
    test_uom_amount!(Zettamole, zettamole);
    test_uom_amount!(Examole, examole);
    test_uom_amount!(Petamole, petamole);
    test_uom_amount!(Teramole, teramole);
    test_uom_amount!(Gigamole, gigamole);
    test_uom_amount!(Megamole, megamole);
    test_uom_amount!(Kilomole, kilomole);
    test_uom_amount!(Hectomole, hectomole);
    test_uom_amount!(Decamole, decamole);
    test_uom_amount!(Decimole, decimole);
    test_uom_amount!(Centimole, centimole);
    test_uom_amount!(Millimole, millimole);
    test_uom_amount!(Micromole, micromole);
    test_uom_amount!(Nanomole, nanomole);
    test_uom_amount!(Picomole, picomole);
    test_uom_amount!(Femtumole, femtomole);
    test_uom_amount!(Attomole, attomole);
    test_uom_amount!(Zeptomole, zeptomole);
    test_uom_amount!(Yoctomole, yoctomole);

    // Test particle counting
    test_uom_amount!(Particle, particle);

    // Test standard gas volumes
    test_uom_amount!(StandardCubicMeter, standard_cubic_meter);
    test_uom_amount!(StandardLiter, standard_liter);
    test_uom_amount!(StandardCubicCentimeter, standard_centimeter);
    test_uom_amount!(StandardCubicFoot, standard_cubic_foot);
}
