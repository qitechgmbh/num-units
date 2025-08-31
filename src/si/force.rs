use typenum::*;

// SI derived unit: newton (kilogram meter per second squared)
base_units! {
    Newton: "newton", "N";
    Kilonewton: "kilonewton", "kN";
    Pound: "pound-force", "lbf";
}

// ===== CONVERSION RELATIONSHIPS =====

// Kilonewton to Newton
convert_base_unit! {
    Kilonewton: |newton| newton / 1000.0;
    Newton: |kilonewton| kilonewton * 1000.0;
}

// Pound-force to Newton (1 lbf = 4.448222 N)
convert_base_unit! {
    Pound: |newton| newton / 4.448222;
    Newton: |pound| pound * 4.448222;
}

// Force quantity definition (Mass×Length/Time²)
use super::{SI, SIScale};
quantity!(Force, SI<P1, P1, N2, Z0, Z0, Z0, Z0>, SIScale);
