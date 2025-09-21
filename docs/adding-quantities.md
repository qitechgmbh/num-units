# Developer Guide: Adding New Quantities to num-units

This guide provides step-by-step instructions for adding new quantities to the num-units library, copying conversion factors from the UOM library, and creating compatibility tests to ensure accuracy.

## Table of Contents

1. [Overview](#overview)
2. [Understanding the Architecture](#understanding-the-architecture)
3. [Step-by-Step: Adding a New Quantity](#step-by-step-adding-a-new-quantity)
4. [Copying Values from UOM](#copying-values-from-uom)
5. [Creating UOM Compatibility Tests](#creating-uom-compatibility-tests)
6. [Best Practices](#best-practices)
7. [Examples](#examples)

## Overview

The num-units library is designed to be compatible with the UOM (Units of Measurement) library while providing better type safety and performance. When adding new quantities, we follow these principles:

- **Exact compatibility**: All conversion factors must match UOM exactly
- **Comprehensive coverage**: Include all units that UOM supports
- **Type safety**: Leverage Rust's type system for dimensional analysis
- **Testing**: Every unit must have a compatibility test

## Understanding the Architecture

### Core Components

1. **Quantities**: Physical dimensions like Length, Mass, Time, etc.
2. **Units**: Specific measurement units like Meter, Kilogram, Second, etc.
3. **Conversions**: Mathematical relationships between units
4. **System**: The SI system definition that ties everything together

### File Structure

Each quantity is implemented in its own file under `src/si/`:

```
src/si/
├── mod.rs              # SI system definition and module declarations
├── length.rs           # Length quantity and units
├── mass.rs             # Mass quantity and units
├── time.rs             # Time quantity and units
└── [quantity].rs       # Your new quantity
```

## Step-by-Step: Adding a New Quantity

### Step 1: Create the Quantity File

Create a new file `src/si/[quantity].rs` using this template:

```rust
/// # [Quantity] Units - SI [Quantity] Measurements
///
/// This module defines SI [quantity] units and their conversions. [Quantity] is [description].
///
/// ## Base Unit
///
/// - **[BaseUnit] ([symbol])**: The SI base unit of [quantity]
///
use typenum::*;

// SI base unit
units! {
    [BaseUnit]: "[symbol]", "[name]";
}

units! {
    // Add all your units here with their symbols and names
    [Unit1]: "[symbol1]", "[name1]";
    [Unit2]: "[symbol2]", "[name2]";
    // ... more units
}

// Import necessary prefixes
use crate::prefix::{
    ATTO, CENTI, DECA, DECI, EXA, FEMTO, GIGA, HECTO, KILO, MEGA, MICRO, MILLI, NANO, PETA, PICO,
    TERA, YOCTO, YOTTA, ZEPTO, ZETTA,
};

// Unit conversions using convert_linear!
crate::convert_linear! {
    // SI prefix units (use the prefix constants)
    [PrefixUnit] => [BaseUnit]: [PREFIX_CONSTANT];
    
    // Custom units (use explicit values from UOM)
    [CustomUnit] => [BaseUnit]: [conversion_factor];
    
    // ... more conversions
}

convert_matrix! {
    [BaseUnit] => [Unit1], [Unit2], [Unit3] // List all units here
}

// Quantity definition - you need to determine the dimensional exponents
use super::{ISQ, SiScale};
quantity!([Quantity], ISQ<[L], [M], [T], [I], [TH], [N], [J]>, SiScale, [BaseUnit]);

// Re-export types for convenience
pub use [quantity]::[Quantity];
pub use [quantity]::*;

#[cfg(test)]
mod tests {
    macro_rules! test_uom_[quantity] {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::[quantity],
                uom::si::[quantity],
                [Quantity],
                [Quantity],
                [BaseUnit],
                $num_units_unit,
                [base_unit_lowercase],
                $uom_unit
            );
        };
    }

    // Add tests for every unit
    test_uom_[quantity]!([Unit1], [unit1_lowercase]);
    test_uom_[quantity]!([Unit2], [unit2_lowercase]);
    // ... more tests
}
```

### Step 2: Determine Dimensional Exponents

The dimensional exponents in `ISQ<L, M, T, I, TH, N, J>` represent:
- `L`: Length dimension exponent
- `M`: Mass dimension exponent  
- `T`: Time dimension exponent
- `I`: Electric current dimension exponent
- `TH`: Temperature dimension exponent
- `N`: Amount of substance dimension exponent
- `J`: Luminous intensity dimension exponent

Examples:
- Length: `ISQ<P1, Z0, Z0, Z0, Z0, Z0, Z0>` (Length^1)
- Area: `ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>` (Length^2)
- Volume: `ISQ<P3, Z0, Z0, Z0, Z0, Z0, Z0>` (Length^3)
- Velocity: `ISQ<P1, Z0, N1, Z0, Z0, Z0, Z0>` (Length^1 × Time^-1)
- Force: `ISQ<P1, P1, N2, Z0, Z0, Z0, Z0>` (Length^1 × Mass^1 × Time^-2)

Use `typenum` constants:
- `Z0`: Zero (0)
- `P1`, `P2`, `P3`, etc.: Positive integers (1, 2, 3, ...)
- `N1`, `N2`, `N3`, etc.: Negative integers (-1, -2, -3, ...)

### Step 3: Add to SI Module

Add your quantity to `src/si/mod.rs`:

```rust
// Add to the module declarations section
pub mod [quantity];

// If it's a base unit, add to the system! macro
system! {
    ISQ,
    SiScale,
    L => length::Meter,
    M => mass::Kilogram,
    T => time::Second,
    I => current::Ampere,
    TH => temperature::Kelvin,
    N => amount::Mole,
    J => luminosity::Candela,
    [DIM] => [quantity]::[BaseUnit], // Add this line if it's a new base dimension
}
```

## Copying Values from UOM

### Step 1: Find the UOM Source

Look in the UOM repository for your quantity:
- Repository: https://github.com/iliekturtles/uom
- Path: `src/si/[quantity].rs`

### Step 2: Copy Unit Definitions

From UOM, copy:
1. **Unit names and symbols**: Look for the `unit!` macro calls
2. **Conversion factors**: Look for the `mod [unit]` sections with `super::Conversion` implementations

### Step 3: Extract Conversion Factors

UOM stores conversion factors in this format:

```rust
mod meter {
    super::Conversion! {
        // Base unit
    }
}

mod kilometer {
    super::Conversion! {
        coefficient: 1.0E3,
        constant: 0.0,
    }
}
```

The `coefficient` value is what you need. Convert it to our format:

```rust
// UOM: coefficient: 1.0E3
// num-units: Kilometer => Meter: 1.0E3;
```

### Step 4: Handle Special Cases

- **Prefix units**: Use our prefix constants instead of explicit values
- **Complex conversions**: Some units may have both coefficient and constant (rare)
- **Derived units**: May need to be calculated from base unit relationships

### Example: Copying from UOM Temperature

```rust
// UOM (src/si/thermodynamic_temperature.rs):
mod kelvin { /* base unit */ }
mod degree_celsius {
    super::Conversion! {
        coefficient: 1.0,
        constant: 273.15,
    }
}

// num-units equivalent:
crate::convert_linear! {
    DegreeCelsius => Kelvin: 1.0, 273.15; // coefficient, constant
}
```

## Creating UOM Compatibility Tests

### Step 1: Use the Test Macro

The `test_uom_compatibility!` macro automatically generates tests:

```rust
#[cfg(test)]
mod tests {
    macro_rules! test_uom_[quantity] {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::[quantity],          // num-units module
                uom::si::[quantity],            // UOM module  
                [Quantity],                     // num-units quantity type
                [Quantity],                     // UOM quantity type (usually same name)
                [BaseUnit],                     // num-units base unit type
                $num_units_unit,                // num-units target unit type
                [base_unit_lowercase],          // UOM base unit identifier
                $uom_unit                       // UOM target unit identifier
            );
        };
    }

    // Test each unit
    test_uom_[quantity]!([Unit1], [unit1_uom_name]);
    test_uom_[quantity]!([Unit2], [unit2_uom_name]);
}
```

### Step 2: Map Unit Names

UOM uses snake_case identifiers, while we use PascalCase types:

| num-units Type | UOM Identifier |
|----------------|----------------|
| `Kilometer`    | `kilometer`    |
| `FootSurvey`   | `foot_survey`  |
| `DegreeCelsius`| `degree_celsius`|

### Step 3: Run Tests

```bash
# Run all compatibility tests
cargo test test_uom_

# Run tests for specific quantity
cargo test test_uom_[quantity]

# Run a specific unit test
cargo test test_[unit_name]
```

### Step 4: Debug Test Failures

If tests fail:

1. **Check conversion factors**: Ensure they match UOM exactly
2. **Verify unit names**: Ensure UOM identifiers are correct
3. **Check dimensional analysis**: Ensure the quantity definition is correct
4. **Look for precision issues**: Use the exact same scientific notation as UOM

## Best Practices

### Code Organization

1. **Group related units**: Keep SI prefixes together, imperial units together, etc.
2. **Add comprehensive comments**: Include the actual conversion values in comments
3. **Use consistent naming**: Follow PascalCase for types, snake_case for UOM identifiers
4. **Include documentation**: Add module-level docs explaining the quantity

### Conversion Factors

1. **Match UOM exactly**: Use the exact same scientific notation (e.g., `1.0E-3` not `0.001`)
2. **Include sources**: Add comments with references to standards when available
3. **Use prefix constants**: For SI prefixes, use the constants from `crate::prefix`
4. **Order logically**: Group by magnitude or system (SI, imperial, etc.)

### Testing

1. **Test every unit**: Every unit must have a UOM compatibility test
2. **Use descriptive names**: Test function names should clearly indicate what's being tested
3. **Add edge cases**: Test unusual units or ones with complex conversions
4. **Verify precision**: Ensure floating-point precision matches UOM

### Documentation

1. **Add unit descriptions**: Explain what each unit is used for
2. **Include examples**: Show common usage patterns
3. **Reference standards**: Link to official definitions when available
4. **Explain conversions**: Document any complex or unusual conversion factors

## Examples

### Example 1: Adding Pressure (Complete)

```rust
/// # Pressure Units - SI Pressure Measurements
///
/// This module defines SI pressure units and their conversions. Pressure is force
/// per unit area, with pascal as the SI base unit.
///
/// ## Base Unit
///
/// - **Pascal (Pa)**: The SI base unit of pressure
///
use typenum::*;

// SI base unit
units! {
    Pascal: "Pa", "pascal";
}

units! {
    // SI prefix units
    Kilopascal: "kPa", "kilopascal";
    Megapascal: "MPa", "megapascal";
    Gigapascal: "GPa", "gigapascal";
    
    // Common units
    Bar: "bar", "bar";
    Atmosphere: "atm", "atmosphere";
    Torr: "Torr", "torr";
    PoundsPerSquareInch: "psi", "pounds per square inch";
}

use crate::prefix::{KILO, MEGA, GIGA};

crate::convert_linear! {
    // SI prefix units
    Kilopascal => Pascal: KILO;         // 1 kPa = 1000 Pa
    Megapascal => Pascal: MEGA;         // 1 MPa = 1,000,000 Pa
    Gigapascal => Pascal: GIGA;         // 1 GPa = 1,000,000,000 Pa
    
    // Common units
    Bar => Pascal: 1.0E5;               // 1 bar = 100,000 Pa
    Atmosphere => Pascal: 1.01325E5;    // 1 atm = 101,325 Pa
    Torr => Pascal: 1.333224E2;         // 1 Torr = 133.3224 Pa
    PoundsPerSquareInch => Pascal: 6.894757E3; // 1 psi = 6,894.757 Pa
}

convert_matrix! {
    Pascal => Kilopascal, Megapascal, Gigapascal, Bar, Atmosphere, Torr, PoundsPerSquareInch
}

// Pressure quantity definition (Force per Area = ML^-1T^-2)
use super::{ISQ, SiScale};
quantity!(Pressure, ISQ<N1, P1, N2, Z0, Z0, Z0, Z0>, SiScale, Pascal);

// Re-export types for convenience
pub use pressure::Pressure;
pub use pressure::*;

#[cfg(test)]
mod tests {
    macro_rules! test_uom_pressure {
        ($num_units_unit:ty, $uom_unit:ident) => {
            crate::test_uom_compatibility!(
                crate::si::pressure,
                uom::si::pressure,
                Pressure,
                Pressure,
                Pascal,
                $num_units_unit,
                pascal,
                $uom_unit
            );
        };
    }

    test_uom_pressure!(Kilopascal, kilopascal);
    test_uom_pressure!(Megapascal, megapascal);
    test_uom_pressure!(Gigapascal, gigapascal);
    test_uom_pressure!(Bar, bar);
    test_uom_pressure!(Atmosphere, atmosphere);
    test_uom_pressure!(Torr, torr);
    test_uom_pressure!(PoundsPerSquareInch, pound_force_per_square_inch);
}
```

### Example 2: Finding UOM Values

To find conversion factors in UOM:

1. **Go to the UOM repository**: https://github.com/iliekturtles/uom
2. **Navigate to the quantity**: `src/si/[quantity].rs`
3. **Look for conversion modules**:

```rust
// In UOM src/si/length.rs
mod foot {
    super::Conversion! {
        coefficient: 3.048E-1,  // This is your conversion factor
        constant: 0.0,
    }
}
```

4. **Copy the coefficient**: `3.048E-1` becomes `Foot => Meter: 3.048E-1;`

### Example 3: Debugging Test Failures

If you get a test failure like:

```
Conversion mismatch for kilometer: num-units = 1000.1, UOM = 1000.0
```

1. **Check your conversion factor**: 
   ```rust
   // Wrong:
   Kilometer => Meter: 1000.1;
   
   // Correct (matches UOM exactly):
   Kilometer => Meter: 1.0E3;
   ```

2. **Verify the UOM identifier**:
   ```rust
   // Wrong:
   test_uom_length!(Kilometer, kilometre);
   
   // Correct:
   test_uom_length!(Kilometer, kilometer);
   ```

## Summary

Adding a new quantity to num-units involves:

1. **Create** the quantity file with unit definitions
2. **Copy** exact conversion factors from UOM
3. **Add** the quantity to the SI module
4. **Write** comprehensive compatibility tests
5. **Verify** all tests pass

Following this guide ensures that your new quantity will be fully compatible with UOM while benefiting from num-units' improved type safety and performance.

Remember: **Every conversion factor must match UOM exactly!** This is the key to maintaining compatibility between the libraries.