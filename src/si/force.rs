/// # Force Units - SI Force Measurements
///
/// This module defines SI force units and their conversions. Force is a derived quantity
/// in the SI system with dimensions of mass × length × time⁻², with the newton as its base unit.
///
/// ## Base Unit
///
/// - **Newton (N)**: The SI derived unit of force (kg·m/s²)
///
/// ## SI Prefixed Units
///
/// All SI prefixes from yocto- to yotta- are supported for newtons:
/// - **Yottanewton (YN)**: 10²⁴ newtons
/// - **Zettanewton (ZN)**: 10²¹ newtons
/// - **Exanewton (EN)**: 10¹⁸ newtons
/// - **Petanewton (PN)**: 10¹⁵ newtons
/// - **Teranewton (TN)**: 10¹² newtons
/// - **Giganewton (GN)**: 10⁹ newtons
/// - **Meganewton (MN)**: 10⁶ newtons
/// - **Kilonewton (kN)**: 10³ newtons
/// - **Hectonewton (hN)**: 10² newtons
/// - **Decanewton (daN)**: 10 newtons
/// - **Newton (N)**: Base unit
/// - **Decinewton (dN)**: 10⁻¹ newtons
/// - **Centinewton (cN)**: 10⁻² newtons
/// - **Millinewton (mN)**: 10⁻³ newtons
/// - **Micronewton (μN)**: 10⁻⁶ newtons
/// - **Nanonewton (nN)**: 10⁻⁹ newtons
/// - **Piconewton (pN)**: 10⁻¹² newtons
/// - **Femtonewton (fN)**: 10⁻¹⁵ newtons
/// - **Attonewton (aN)**: 10⁻¹⁸ newtons
/// - **Zeptonewton (zN)**: 10⁻²¹ newtons
/// - **Yoctonewton (yN)**: 10⁻²⁴ newtons
///
/// ## Other Units
///
/// - **Dyne (dyn)**: 10⁻⁵ newtons (CGS unit)
/// - **KilogramForce (kgf)**: 9.80665 newtons (kilopond)
/// - **GramForce (gf)**: 9.80665 × 10⁻³ newtons
/// - **Kip**: 4,448.222 newtons
/// - **OunceForce (ozf)**: 0.2780139 newtons
/// - **Poundal (pdl)**: 0.138255 newtons
/// - **PoundForce (lbf)**: 4.448222 newtons
/// - **TonForce**: 8,896.443 newtons (metric ton-force)
///
/// ## Usage
///
/// ```rust,ignore
/// use num_units::force::Force;
/// use num_units::force::{Newton, Kilonewton, PoundForce};
///
/// // Create force quantities
/// let force = Force::from::<Newton>(100.0);
/// let heavy_force = Force::from::<Kilonewton>(5.0);
/// let imperial_force = Force::from::<PoundForce>(50.0);
///
/// // Convert between units
/// let force_kn = force.to::<Kilonewton>();        // 0.1 kN
/// let heavy_force_n = heavy_force.to::<Newton>(); // 5000.0 N
/// let imperial_force_n = imperial_force.to::<Newton>(); // 222.411 N
///
/// // Arithmetic operations
/// let total_force = force + heavy_force;          // Automatic conversion
/// // let acceleration = force / mass;               // Creates acceleration quantity
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All force operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
use crate::prefix::{
    ATTO, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI, NANO, PETA, PICO, TERA,
    YOCTO, YOTTA, ZEPTO, ZETTA,
};
use typenum::*;

// SI base unit
units! {
    Newton: "N", "newton";
}

// SI prefixed newtons
units! {
    Yottanewton: "YN", "yottanewton";
    Zettanewton: "ZN", "zettanewton";
    Exanewton: "EN", "exanewton";
    Petanewton: "PN", "petanewton";
    Teranewton: "TN", "teranewton";
    Giganewton: "GN", "giganewton";
    Meganewton: "MN", "meganewton";
    Kilonewton: "kN", "kilonewton";
    Hectonewton: "hN", "hectonewton";
    Decanewton: "daN", "decanewton";
    Decinewton: "dN", "decinewton";
    Centinewton: "cN", "centinewton";
    Millinewton: "mN", "millinewton";
    Micronewton: "μN", "micronewton";
    Nanonewton: "nN", "nanonewton";
    Piconewton: "pN", "piconewton";
    Femtonewton: "fN", "femtonewton";
    Attonewton: "aN", "attonewton";
    Zeptonewton: "zN", "zeptonewton";
    Yoctonewton: "yN", "yoctonewton";
}

// Other force units
units! {
    Dyne: "dyn", "dyne";
    KilogramForce: "kgf", "kilogram-force";
    GramForce: "gf", "gram-force";
    Kip: "kip", "kip";
    OunceForce: "ozf", "ounce-force";
    Poundal: "pdl", "poundal";
    PoundForce: "lbf", "pound-force";
    TonForce: "2000 lbf", "ton-force";
}

// ===== CONVERSION RELATIONSHIPS =====

// Unit conversions using convert_linear! with exact UOM coefficients
crate::convert_linear! {
    // SI prefixed newtons
    Yottanewton => Newton: YOTTA;
    Zettanewton => Newton: ZETTA;
    Exanewton => Newton: EXA;
    Petanewton => Newton: PETA;
    Teranewton => Newton: TERA;
    Giganewton => Newton: GIGA;
    Meganewton => Newton: MEGA;
    Kilonewton => Newton: KILO;
    Hectonewton => Newton: HECTO;
    Decanewton => Newton: DECA;
    Decinewton => Newton: DECI;
    Centinewton => Newton: crate::prefix::CENTI;
    Millinewton => Newton: MILLI;
    Micronewton => Newton: MICRO;
    Nanonewton => Newton: NANO;
    Piconewton => Newton: PICO;
    Femtonewton => Newton: FEMTO;
    Attonewton => Newton: ATTO;
    Zeptonewton => Newton: ZEPTO;
    Yoctonewton => Newton: YOCTO;

    // Other units - exact UOM coefficients
    Dyne => Newton: 1.0_E-5;
    KilogramForce => Newton: 9.806_65_E0;
    GramForce => Newton: 9.806_65_E-3;
    Kip => Newton: 4.448_222_E3;
    OunceForce => Newton: 2.780_139_E-1;
    Poundal => Newton: 1.382_550_E-1;
    PoundForce => Newton: 4.448_222_E0;
    TonForce => Newton: 8.896_443_E3;
}

crate::convert_matrix! {
    Newton => Yottanewton, Zettanewton, Exanewton, Petanewton, Teranewton, Giganewton, Meganewton, Kilonewton, Hectonewton, Decanewton, Decinewton, Centinewton, Millinewton, Micronewton, Nanonewton, Piconewton, Femtonewton, Attonewton, Zeptonewton, Yoctonewton, Dyne, KilogramForce, GramForce, Kip, OunceForce, Poundal, PoundForce, TonForce
}

// Force quantity definition (Mass×Length/Time²)
use super::{ISQ, SiScale};
quantity!(Force, ISQ<P1, P1, N2, Z0, Z0, Z0, Z0>, SiScale, Newton);

// UOM compatibility tests
#[cfg(test)]
mod tests {

    macro_rules! test_uom_force {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::force,
                uom::si::force,
                Force,
                Force,
                Newton,
                $num_units_unit,
                newton,
                $uom_unit
            );
        };
    }

    // Test SI prefixed newtons
    test_uom_force!(Yottanewton, yottanewton);
    test_uom_force!(Zettanewton, zettanewton);
    test_uom_force!(Exanewton, exanewton);
    test_uom_force!(Petanewton, petanewton);
    test_uom_force!(Teranewton, teranewton);
    test_uom_force!(Giganewton, giganewton);
    test_uom_force!(Meganewton, meganewton);
    test_uom_force!(Kilonewton, kilonewton);
    test_uom_force!(Hectonewton, hectonewton);
    test_uom_force!(Decanewton, decanewton);
    test_uom_force!(Newton, newton);
    test_uom_force!(Decinewton, decinewton);
    test_uom_force!(Centinewton, centinewton);
    test_uom_force!(Millinewton, millinewton);
    test_uom_force!(Micronewton, micronewton);
    test_uom_force!(Nanonewton, nanonewton);
    test_uom_force!(Piconewton, piconewton);
    test_uom_force!(Femtonewton, femtonewton);
    test_uom_force!(Attonewton, attonewton);
    test_uom_force!(Zeptonewton, zeptonewton);
    test_uom_force!(Yoctonewton, yoctonewton);

    // Test other units
    test_uom_force!(Dyne, dyne);
    test_uom_force!(KilogramForce, kilogram_force);
    test_uom_force!(GramForce, gram_force);
    test_uom_force!(Kip, kip);
    test_uom_force!(OunceForce, ounce_force);
    test_uom_force!(Poundal, poundal);
    test_uom_force!(PoundForce, pound_force);
    test_uom_force!(TonForce, ton_force);
}

// Re-export types for convenience
pub use force::Force;
pub use force::*;
