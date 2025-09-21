# Claude Code Context for num-units

This document provides important context about the num-units repository to help Claude Code understand the codebase structure and development patterns.

## Project Overview

**num-units** is a comprehensive, zero-cost dimensional analysis library for Rust that prevents unit conversion errors at compile time. It's designed for physics simulations, engineering calculations, and any domain where dimensional correctness is critical.

### Key Features
- Type-safe dimensional analysis at compile time
- Zero runtime overhead (all checks are compile-time)
- Support for both `std` and `no_std` environments
- Comprehensive unit conversion system
- ISQ (International System of Quantities) based design

## Architecture

### Core Modules

1. **`quantity`** (`src/quantity/`)
   - Defines the `Quantity<D, U, V>` type representing physical quantities
   - `D`: Dimension (e.g., Length, Time, Mass)
   - `U`: Unit (e.g., Meter, Second, Kilogram)
   - `V`: Value type (e.g., f32, f64, i32)
   - Implements arithmetic operations, trait implementations, and type conversions

2. **`unit`** (`src/unit.rs`)
   - Defines the `Unit` trait and unit type system
   - `FromUnit` trait for unit conversions
   - Unit definition macros

3. **`conversions`** (`src/conversions.rs`)
   - Comprehensive conversion macro system
   - Hierarchical macro structure from high-level to type-specific
   - Support for linear, integer, and matrix conversions

4. **`system`** (`src/system.rs`)
   - System of units definition macros
   - Dimension composition and arithmetic

5. **`isq`** (`src/isq/`)
   - International System of Quantities implementations
   - Pre-defined quantities: Length, Mass, Time, Temperature, Current, etc.
   - Each module defines its dimension, units, and conversions

6. **`prefix`** (`src/prefix.rs`)
   - SI prefixes (KILO, MEGA, MILLI, MICRO, etc.)
   - Used in unit conversions

### Macro System

The library heavily uses macros for code generation:

#### Unit Definition Macros
- `units!` - Define new units for a dimension
- `quantity!` - Define a new quantity type with dimension

#### Conversion Macros (Hierarchical)
```
Top-level (user-facing):
├── convert!           → convert_float! → {convert_f32!, convert_f64!}
├── convert_linear!    → convert!
├── convert_int!       → Direct implementation for integers and floats
├── convert_int_linear!→ convert_int!
└── convert_matrix!    → convert_matrix_float! → transitive conversions
```

#### System Macros
- `system!` - Define a complete system of units
- `ISQ!` - Specific macro for ISQ dimension arithmetic

### Procedural Macros

Located in `num-units-macros/`:
- `Dimension` derive macro for creating new dimensions
- Handles dimension arithmetic at compile time

## Development Patterns

### Adding New Quantities

1. Create a new module in `src/isq/`
2. Define the dimension using `quantity!` macro
3. Define units using `units!` macro
4. Define conversions using appropriate conversion macros
5. Re-export from `src/isq/mod.rs`

Example structure:
```rust
// src/isq/new_quantity.rs
quantity! {
    dimension: NewDimension;
    quantity: NewQuantity;
}

units! {
    BaseUnit: "symbol", "name";
    DerivedUnit: "symbol", "name";
}

convert_linear! {
    DerivedUnit => BaseUnit: conversion_factor;
}
```

### Testing

- Unit tests are in `tests/` directory
- Key test files:
  - `dimension_inference.rs` - Tests dimension calculations
  - `angle_scalar.rs` - Tests angle and scalar conversions
  - `generic_api.rs` - Tests generic API usage
  - `functions.rs` - Tests mathematical functions

### Important Implementation Details

1. **Zero-cost Abstraction**: All dimensional analysis happens at compile time through the type system
2. **Typenum Integration**: Uses `typenum` for compile-time integer arithmetic
3. **No Runtime Overhead**: `Quantity` is a newtype wrapper around the value
4. **Conversion Safety**: All unit conversions are type-checked at compile time

## Common Tasks

### Running Tests
```bash
cargo test              # Run all tests
cargo test --no-default-features  # Test no_std
cargo check            # Check compilation
```

### Adding Conversions
Use the appropriate macro based on conversion type:
- Simple scaling: `convert_linear!`
- Complex conversions: `convert!`
- Integer exact: `convert_int!`
- Generate all combinations: `convert_matrix!`

### Debugging Macro Expansions
```bash
cargo expand           # View expanded macros
```

## Project Structure
```
num-units/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── quantity/           # Quantity type and trait implementations
│   ├── unit.rs            # Unit trait and conversions
│   ├── conversions.rs     # Conversion macro system
│   ├── system.rs          # System of units macros
│   ├── prefix.rs          # SI prefixes
│   └── isq/               # ISQ quantity implementations
│       ├── mod.rs         # Module exports
│       ├── length.rs      # Length dimension
│       ├── mass.rs        # Mass dimension
│       ├── time.rs        # Time dimension
│       └── ...            # Other quantities
├── num-units-macros/      # Procedural macros
│   └── src/lib.rs         # Dimension derive macro
├── tests/                 # Integration tests
├── Cargo.toml            # Package configuration
└── README.md             # User documentation
```

## Dependencies
- `num-traits`: Numeric traits for generic programming
- `typenum`: Type-level numbers for compile-time arithmetic
- `paste`: Token pasting for macro generation
- `num-units-macros`: Internal procedural macros

## Features
- `std` (default): Standard library support
- `libm`: Math functions for no_std environments

## Design Principles
1. **Compile-time Safety**: Catch dimensional errors at compile time
2. **Zero Runtime Cost**: No performance overhead
3. **Ergonomic API**: Easy to use while maintaining safety
4. **Extensibility**: Easy to add new quantities and units
5. **Standards Compliance**: Based on ISQ standards

## Notes for Development
- Always ensure conversions are bidirectional
- Test both std and no_std configurations
- Use appropriate conversion macros for precision requirements
- Document new quantities and units thoroughly
- Maintain consistency with ISQ standards where applicable

## Maintaining This Document

**Important**: This CLAUDE.md file serves as the primary context document for Claude Code when working with the num-units repository. If you make changes to any of the following aspects of the codebase, this document should be updated accordingly:

- Architecture changes (new modules, significant refactoring)
- Macro system modifications or new macro additions
- Changes to the project structure or file organization
- New development patterns or conventions
- Updates to testing strategies
- Changes in dependencies or features
- Modifications to the build process or common tasks

Keeping this document synchronized with the actual codebase ensures that future Claude Code sessions have accurate context and can provide appropriate assistance. When in doubt, update this document if your changes would affect how someone understands or works with the codebase.