use crate::prefix::{CENTI, KILO, MILLI};
use typenum::*;

// SI base unit
units! {
    SquareMeter: "m²", "square meter";
}

units! {
    SquareKilometer: "km²", "square kilometer";
    SquareCentimeter: "cm²", "square centimeter";
    SquareMillimeter: "mm²", "square millimeter";
}

convert_unit! {
    SquareKilometer: |squaremeter| squaremeter / (KILO * KILO);
    SquareMeter: |squarekilometer| squarekilometer * (KILO * KILO);
}

convert_unit! {
    SquareCentimeter: |squaremeter| squaremeter / (CENTI * CENTI);
    SquareMeter: |squarecentimeter| squarecentimeter * (CENTI * CENTI);
}

convert_unit! {
    SquareMillimeter: |squaremeter| squaremeter / (MILLI * MILLI);
    SquareMeter: |squaremillimeter| squaremillimeter * (MILLI * MILLI);
}

// Area quantity definition
use super::{SI, SIScale};
quantity!(Area, SI<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale);
