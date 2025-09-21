/// # Luminous Intensity Units - SI Luminous Intensity Measurements
///
/// This module defines SI luminous intensity units and their conversions. Luminous intensity
/// is one of the seven base quantities in the SI system, with the candela as its base unit.
///
/// ## Base Unit
///
/// - **Candela (cd)**: The SI base unit of luminous intensity
///
/// ## SI Prefixed Units
///
/// All SI prefixes from yocto- to yotta- are supported for candela:
/// - **Yottacandela (Ycd)**: 10²⁴ candelas
/// - **Zettacandela (Zcd)**: 10²¹ candelas
/// - **Exacandela (Ecd)**: 10¹⁸ candelas
/// - **Petacandela (Pcd)**: 10¹⁵ candelas
/// - **Teracandela (Tcd)**: 10¹² candelas
/// - **Gigacandela (Gcd)**: 10⁹ candelas
/// - **Megacandela (Mcd)**: 10⁶ candelas
/// - **Kilocandela (kcd)**: 10³ candelas
/// - **Hectocandela (hcd)**: 10² candelas
/// - **Decacandela (dacd)**: 10 candelas
/// - **Candela (cd)**: Base unit
/// - **Decicandela (dcd)**: 10⁻¹ candelas
/// - **Centicandela (ccd)**: 10⁻² candelas
/// - **Millicandela (mcd)**: 10⁻³ candelas
/// - **Microcandela (μcd)**: 10⁻⁶ candelas
/// - **Nanocandela (ncd)**: 10⁻⁹ candelas
/// - **Picocandela (pcd)**: 10⁻¹² candelas
/// - **Femtocandela (fcd)**: 10⁻¹⁵ candelas
/// - **Attocandela (acd)**: 10⁻¹⁸ candelas
/// - **Zeptocandela (zcd)**: 10⁻²¹ candelas
/// - **Yoctocandela (ycd)**: 10⁻²⁴ candelas
///
/// ## Usage
///
/// ```rust,ignore
/// use num_units::luminosity::Luminosity;
/// use num_units::luminosity::{Candela, Millicandela, Kilocandela};
///
/// // Create luminous intensity quantities
/// let intensity = Luminosity::from::<Candela>(100.0);
/// let small_intensity = Luminosity::from::<Millicandela>(500.0);
/// let large_intensity = Luminosity::from::<Kilocandela>(2.0);
///
/// // Convert between units
/// let intensity_mcd = intensity.to::<Millicandela>();    // 100000.0 mcd
/// let small_intensity_cd = small_intensity.to::<Candela>(); // 0.5 cd
/// let large_intensity_cd = large_intensity.to::<Candela>(); // 2000.0 cd
///
/// // Arithmetic operations
/// let total_intensity = intensity + small_intensity;     // Automatic conversion
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All luminous intensity operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
use crate::prefix::{
    ATTO, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI, NANO, PETA, PICO, TERA,
    YOCTO, YOTTA, ZEPTO, ZETTA,
};
use typenum::*;

// SI base unit
units! {
    Candela: "cd", "candela";
}

// SI prefixed candelas
units! {
    Yottacandela: "Ycd", "yottacandela";
    Zettacandela: "Zcd", "zettacandela";
    Exacandela: "Ecd", "exacandela";
    Petacandela: "Pcd", "petacandela";
    Teracandela: "Tcd", "teracandela";
    Gigacandela: "Gcd", "gigacandela";
    Megacandela: "Mcd", "megacandela";
    Kilocandela: "kcd", "kilocandela";
    Hectocandela: "hcd", "hectocandela";
    Decacandela: "dacd", "decacandela";
    Decicandela: "dcd", "decicandela";
    Centicandela: "ccd", "centicandela";
    Millicandela: "mcd", "millicandela";
    Microcandela: "μcd", "microcandela";
    Nanocandela: "ncd", "nanocandela";
    Picocandela: "pcd", "picocandela";
    Femtocandela: "fcd", "femtocandela";
    Attocandela: "acd", "attocandela";
    Zeptocandela: "zcd", "zeptocandela";
    Yoctocandela: "ycd", "yoctocandela";
}

// Unit conversions using convert_linear! with exact UOM coefficients
crate::convert_linear! {
    // SI prefixed candelas
    Yottacandela => Candela: YOTTA;
    Zettacandela => Candela: ZETTA;
    Exacandela => Candela: EXA;
    Petacandela => Candela: PETA;
    Teracandela => Candela: TERA;
    Gigacandela => Candela: GIGA;
    Megacandela => Candela: MEGA;
    Kilocandela => Candela: KILO;
    Hectocandela => Candela: HECTO;
    Decacandela => Candela: DECA;
    Decicandela => Candela: DECI;
    Centicandela => Candela: crate::prefix::CENTI;
    Millicandela => Candela: MILLI;
    Microcandela => Candela: MICRO;
    Nanocandela => Candela: NANO;
    Picocandela => Candela: PICO;
    Femtocandela => Candela: FEMTO;
    Attocandela => Candela: ATTO;
    Zeptocandela => Candela: ZEPTO;
    Yoctocandela => Candela: YOCTO;
}

crate::convert_matrix! {
    Candela => Yottacandela, Zettacandela, Exacandela, Petacandela, Teracandela, Gigacandela, Megacandela, Kilocandela, Hectocandela, Decacandela, Decicandela, Centicandela, Millicandela, Microcandela, Nanocandela, Picocandela, Femtocandela, Attocandela, Zeptocandela, Yoctocandela
}

// Luminosity quantity definition (luminous intensity is the 7th base dimension)
use super::{ISQ, SiScale};
quantity!(Luminosity, ISQ<Z0, Z0, Z0, Z0, Z0, Z0, P1>, SiScale, Candela);

// UOM compatibility tests
#[cfg(test)]
mod tests {

    macro_rules! test_uom_luminosity {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::luminosity,
                uom::si::luminous_intensity,
                Luminosity,
                LuminousIntensity,
                Candela,
                $num_units_unit,
                candela,
                $uom_unit
            );
        };
    }

    // Test SI prefixed candelas
    test_uom_luminosity!(Yottacandela, yottacandela);
    test_uom_luminosity!(Zettacandela, zettacandela);
    test_uom_luminosity!(Exacandela, exacandela);
    test_uom_luminosity!(Petacandela, petacandela);
    test_uom_luminosity!(Teracandela, teracandela);
    test_uom_luminosity!(Gigacandela, gigacandela);
    test_uom_luminosity!(Megacandela, megacandela);
    test_uom_luminosity!(Kilocandela, kilocandela);
    test_uom_luminosity!(Hectocandela, hectocandela);
    test_uom_luminosity!(Decacandela, decacandela);
    test_uom_luminosity!(Candela, candela);
    test_uom_luminosity!(Decicandela, decicandela);
    test_uom_luminosity!(Centicandela, centicandela);
    test_uom_luminosity!(Millicandela, millicandela);
    test_uom_luminosity!(Microcandela, microcandela);
    test_uom_luminosity!(Nanocandela, nanocandela);
    test_uom_luminosity!(Picocandela, picocandela);
    test_uom_luminosity!(Femtocandela, femtocandela);
    test_uom_luminosity!(Attocandela, attocandela);
    test_uom_luminosity!(Zeptocandela, zeptocandela);
    test_uom_luminosity!(Yoctocandela, yoctocandela);
}

// Re-export types for convenience
pub use luminosity::Luminosity;
pub use luminosity::*;
