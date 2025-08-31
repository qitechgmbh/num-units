use crate::prefix::{CENTI, KILO, MILLI};
use typenum::*;

// SI base unit
base_units! {
    SquareMeter: "squaremeter", "m²";
}

base_units! {
    SquareKilometer: "squarekilometer", "km²";
    SquareCentimeter: "squarecentimeter", "cm²";
    SquareMillimeter: "squaremillimeter", "mm²";
}

convert_base_unit! {
    SquareKilometer: |squaremeter| squaremeter / (KILO * KILO);
    SquareMeter: |squarekilometer| squarekilometer * (KILO * KILO);
}

convert_base_unit! {
    SquareCentimeter: |squaremeter| squaremeter / (CENTI * CENTI);
    SquareMeter: |squarecentimeter| squarecentimeter * (CENTI * CENTI);
}

convert_base_unit! {
    SquareMillimeter: |squaremeter| squaremeter / (MILLI * MILLI);
    SquareMeter: |squaremillimeter| squaremillimeter * (MILLI * MILLI);
}

// Area quantity definition
use super::{SI, SIScale};
quantity!(Area, SI<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale);
