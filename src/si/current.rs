/// # Electric Current Units - SI Electric Current Measurements
///
/// This module defines SI electric current units and their conversions. Electric current is one
/// of the seven base quantities in the SI system, with the ampere as its base unit.
///
/// ## Base Unit
///
/// - **Ampere (A)**: The SI base unit of electric current
///
/// ## Derived Units
///
/// Common electric current units include:
/// - **Kiloampere (kA)**: 1,000 amperes
/// - **Milliampere (mA)**: 0.001 amperes
/// - **Microampere (μA)**: 0.000001 amperes
/// - **Abampere (abA)**: 10 amperes (CGS unit)
/// - **Statampere (statA)**: 3.335641×10⁻¹⁰ amperes (CGS-ESU unit)
///
/// ## Usage
///
/// ```rust,ignore
/// use num_units::current::Current;
/// use num_units::current::{Ampere, Milliampere};
///
/// // Create current quantities
/// let current = Current::from::<Ampere>(5.0);
/// let small_current = Current::from::<Milliampere>(500.0);
///
/// // Convert between units
/// let current_ma = current.to::<Milliampere>();    // 5000.0 mA
/// let small_current_a = small_current.to::<Ampere>(); // 0.5 A
///
/// // Arithmetic operations
/// let total_current = current + small_current;     // Automatic conversion
/// // let power = current * voltage;                   // Creates power quantity
/// ```
///
/// ## Architecture
///
/// This module uses the dimensional analysis system to ensure type safety:
/// - All current operations are dimensionally consistent
/// - Unit conversions are automatic and type-safe
/// - Compile-time dimensional analysis prevents errors
use crate::prefix::*;
use typenum::*;

// SI base unit
units! {
    Ampere: "A", "ampere";
}

// SI prefixed units
units! {
    Yottaampere: "YA", "yottaampere";
    Zettaampere: "ZA", "zettaampere";
    Exaampere: "EA", "exaampere";
    Petaampere: "PA", "petaampere";
    Teraampere: "TA", "teraampere";
    Gigaampere: "GA", "gigaampere";
    Megaampere: "MA", "megaampere";
    Kiloampere: "kA", "kiloampere";
    Hectoampere: "hA", "hectoampere";
    Decaampere: "daA", "decaampere";
    Deciampere: "dA", "deciampere";
    Centiampere: "cA", "centiampere";
    Milliampere: "mA", "milliampere";
    Microampere: "μA", "microampere";
    Nanoampere: "nA", "nanoampere";
    Picoampere: "pA", "picoampere";
    Femtoampere: "fA", "femtoampere";
    Attoampere: "aA", "attoampere";
    Zeptoampere: "zA", "zeptoampere";
    Yoctoampere: "yA", "yoctoampere";
}

// Historical/CGS units
units! {
    Abampere: "abA", "abampere";
    Gilbert: "Gi", "gilbert";
    Statampere: "statA", "statampere";
    ElementaryChargePerSecond: "e/s", "elementary charge per second";
    AtomicUnitOfChargePerSecond: "a.u. of charge/s", "atomic unit of charge per second";
}

// Unit conversions using convert_linear! with exact UOM coefficients
crate::convert_linear! {
    // SI prefixed units
    Yottaampere => Ampere: YOTTA;
    Zettaampere => Ampere: ZETTA;
    Exaampere => Ampere: EXA;
    Petaampere => Ampere: PETA;
    Teraampere => Ampere: TERA;
    Gigaampere => Ampere: GIGA;
    Megaampere => Ampere: MEGA;
    Kiloampere => Ampere: KILO;
    Hectoampere => Ampere: HECTO;
    Decaampere => Ampere: DECA;
    Deciampere => Ampere: DECI;
    Centiampere => Ampere: CENTI;
    Milliampere => Ampere: MILLI;
    Microampere => Ampere: MICRO;
    Nanoampere => Ampere: NANO;
    Picoampere => Ampere: PICO;
    Femtoampere => Ampere: FEMTO;
    Attoampere => Ampere: ATTO;
    Zeptoampere => Ampere: ZEPTO;
    Yoctoampere => Ampere: YOCTO;

    // Historical/CGS units - exact UOM coefficients
    Abampere => Ampere: 1.0_E1;
    Gilbert => Ampere: 7.957_747_E-1;
    Statampere => Ampere: 3.335_641_E-10;
    ElementaryChargePerSecond => Ampere: 1.602_176_634_E-19;
    AtomicUnitOfChargePerSecond => Ampere: 1.602_176_634_E-19;
}

crate::convert_matrix! {
    Ampere => Yottaampere, Zettaampere, Exaampere, Petaampere, Teraampere, Gigaampere, Megaampere, Kiloampere, Hectoampere, Decaampere, Deciampere, Centiampere, Milliampere, Microampere, Nanoampere, Picoampere, Femtoampere, Attoampere, Zeptoampere, Yoctoampere, Abampere, Gilbert, Statampere, ElementaryChargePerSecond, AtomicUnitOfChargePerSecond
}

// Current quantity definition
use super::{ISQ, SiScale};
quantity!(Current, ISQ<Z0, Z0, Z0, P1, Z0, Z0, Z0>, SiScale, Ampere);

// UOM compatibility tests
#[cfg(test)]
mod tests {
    macro_rules! test_uom_current {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::current,
                uom::si::electric_current,
                Current,
                ElectricCurrent,
                Ampere,
                $num_units_unit,
                ampere,
                $uom_unit
            );
        };
    }

    // Test SI prefixed units
    test_uom_current!(Yottaampere, yottaampere);
    test_uom_current!(Zettaampere, zettaampere);
    test_uom_current!(Exaampere, exaampere);
    test_uom_current!(Petaampere, petaampere);
    test_uom_current!(Teraampere, teraampere);
    test_uom_current!(Gigaampere, gigaampere);
    test_uom_current!(Megaampere, megaampere);
    test_uom_current!(Kiloampere, kiloampere);
    test_uom_current!(Hectoampere, hectoampere);
    test_uom_current!(Decaampere, decaampere);
    test_uom_current!(Ampere, ampere);
    test_uom_current!(Deciampere, deciampere);
    test_uom_current!(Centiampere, centiampere);
    test_uom_current!(Milliampere, milliampere);
    test_uom_current!(Microampere, microampere);
    test_uom_current!(Nanoampere, nanoampere);
    test_uom_current!(Picoampere, picoampere);
    test_uom_current!(Femtoampere, femtoampere);
    test_uom_current!(Attoampere, attoampere);
    test_uom_current!(Zeptoampere, zeptoampere);
    test_uom_current!(Yoctoampere, yoctoampere);

    // Test historical/CGS units
    test_uom_current!(Abampere, abampere);
    test_uom_current!(Gilbert, gilbert);
    test_uom_current!(Statampere, statampere);
    test_uom_current!(ElementaryChargePerSecond, elementary_charge_per_second);
    test_uom_current!(
        AtomicUnitOfChargePerSecond,
        atomic_unit_of_charge_per_second
    );
}

// Re-export types for convenience
pub use current::Current;
pub use current::*;
