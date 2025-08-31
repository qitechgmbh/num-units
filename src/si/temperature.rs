use crate::unit;

// All temperature units
unit! {
    system: crate::si;
    quantity: crate::thermodynamic_temperature;

    // SI base unit
    @kelvin: 1.0; "K", "kelvin", "kelvins";

    // Common temperature scales with offset
    @celsius: 1.0, 273.15; "°C", "celsius", "celsius";
    @fahrenheit: 5.0 / 9.0, 255.37222222222223; "°F", "fahrenheit", "fahrenheit";

    // Other temperature scales
    @rankine: 5.0 / 9.0; "°R", "rankine", "rankine";
    @delisle: -2.0 / 3.0, 559.725; "°De", "delisle", "delisle";
    @newton: 100.0 / 33.0, 273.15; "°N", "newton", "newton";
    @reaumur: 1.25, 273.15; "°Ré", "réaumur", "réaumur";
    @romer: 40.0 / 21.0, 258.8642857142857; "°Rø", "rømer", "rømer";
}
