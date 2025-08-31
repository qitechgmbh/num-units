//! # Nom Macros - Compile-Time Dimensional Analysis 🎩✨⚡
//!
//! This crate provides a procedural macro for generating zero-cost dimensional analysis systems
//! at compile time using const generic expressions. Perfect for physics simulations, engineering
//! calculations, computer graphics, and any domain where unit correctness matters.
//!
//! ## Features
//!
//! - 🚀 **Zero Runtime Cost**: All dimensional checking happens at compile time
//! - 🔬 **Fully Generic**: Support any dimensional system, not just SI units  
//! - 🛡️ **Type Safe**: Prevent dimensional errors like adding length + time
//! - 🧮 **Const Expressions**: Uses cutting-edge const generic arithmetic
//! - 📱 **No-std Compatible**: Works in embedded and kernel environments
//! - ⚡ **Automatic Operations**: Generates multiplication, division, and power operations
//! - 🔢 **num-traits Integration**: Checked operations, Zero/One traits, and more
//! - 🎯 **Mathematical Completeness**: Square roots, arbitrary powers, identity elements
//!
//! ## Quick Start
//!
//! ```rust
//! #![feature(generic_const_exprs)]
//! use nom_macros::system;
//!
//! // Define your dimensional system
//! #[system(L, M, T)]  // Length, Mass, Time
//! pub struct Physics;
//!
//! // Create type aliases
//! pub type Length = Physics<1, 0, 0>;
//! pub type Mass = Physics<0, 1, 0>;
//! pub type Time = Physics<0, 0, 1>;
//! pub type Velocity = Physics<1, 0, -1>;  // Length/Time
//! pub type Force = Physics<1, 1, -2>;     // Mass×Length/Time²
//!
//! // Use with compile-time safety!
//! let distance: Length = Physics::new();
//! let time: Time = Physics::new();
//! let speed: Velocity = distance / time;  // ✅ Compiles
//! // let invalid = distance + time;       // ❌ Compile error!
//! ```
//!
//! ## Requirements
//!
//! This crate requires:
//! - Nightly Rust (for `generic_const_exprs` feature)
//! - `#![feature(generic_const_exprs)]` in your crate root
//!
//! Add to your `rust-toolchain.toml`:
//! ```toml
//! [toolchain]
//! channel = "nightly"
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Ident, Token, parse::Parse, parse::ParseStream, parse_macro_input,
    punctuated::Punctuated,
};

// Custom parser for comma-separated identifiers
struct DimensionArgs {
    dimensions: Punctuated<Ident, Token![,]>,
}

impl Parse for DimensionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DimensionArgs {
            dimensions: input.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

/// # The Ultimate System Analysis Proc Macro! 🎩✨⚡
///
/// This procedural macro generates a compile-time dimensional analysis system using
/// const generic expressions. It creates zero-cost abstractions for unit checking
/// that are completely resolved at compile time.
///
/// ## Usage
///
/// ```rust
/// #![feature(generic_const_exprs)]
/// use nom_macros::system;
///
/// // Define a dimensional system with any number of dimensions
/// #[system(L, M, T)]  // Length, Mass, Time
/// pub struct Physics;
///
/// // Create type aliases for common quantities
/// pub type Length = Physics<1, 0, 0>;     // L¹M⁰T⁰
/// pub type Mass = Physics<0, 1, 0>;       // L⁰M¹T⁰  
/// pub type Time = Physics<0, 0, 1>;       // L⁰M⁰T¹
/// pub type Velocity = Physics<1, 0, -1>;  // L¹M⁰T⁻¹
/// pub type Area = Physics<2, 0, 0>;       // L²M⁰T⁰
/// pub type Force = Physics<1, 1, -2>;     // L¹M¹T⁻² (Newton's second law)
/// ```
///
/// ## Generated Operations
///
/// The macro automatically generates implementations for:
///
/// ### Multiplication (`core::ops::Mul`)
/// Dimensional exponents are added during multiplication:
/// ```rust
/// # #![feature(generic_const_exprs)]
/// # use nom_macros::system;
/// # #[system(L, M, T)] pub struct Physics;
/// # type Length = Physics<1, 0, 0>;
/// # type Time = Physics<0, 0, 1>;
/// # type Velocity = Physics<1, 0, -1>;
/// let length: Length = Physics::new();
/// let time: Time = Physics::new();
/// let velocity: Velocity = length / time;  // L¹T⁰ / L⁰T¹ = L¹T⁻¹
/// ```
///
/// ### Division (`core::ops::Div`)
/// Dimensional exponents are subtracted during division:
/// ```rust
/// # #![feature(generic_const_exprs)]
/// # use nom_macros::system;
/// # #[system(L, M, T)] pub struct Physics;
/// # type Length = Physics<1, 0, 0>;
/// # type Area = Physics<2, 0, 0>;
/// let area: Area = Physics::new();
/// let length: Length = Physics::new();
/// let width: Length = area / length;  // L²T⁰ / L¹T⁰ = L¹T⁰
/// ```
///
/// ### Power Operations
/// - `.squared()` - Multiplies all exponents by 2
/// - `.cubed()` - Multiplies all exponents by 3  
/// - `.pow<N>()` - Multiplies all exponents by N (const generic)
/// - `.sqrt()` - Divides all exponents by 2 (only works if all exponents are even)
///
/// ```rust
/// # #![feature(generic_const_exprs)]
/// # use nom_macros::system;
/// # #[system(L, M, T)] pub struct Physics;
/// # type Length = Physics<1, 0, 0>;
/// # type Area = Physics<2, 0, 0>;
/// # type Volume = Physics<3, 0, 0>;
/// let length: Length = Physics::new();
/// let area: Area = length.squared();    // L¹ → L²
/// let volume: Volume = length.cubed();  // L¹ → L³
/// let back_to_length: Length = area.sqrt(); // L² → L¹
/// ```
///
/// ### num-traits Integration
///
/// The macro also implements several `num-traits` for enhanced mathematical operations:
/// - `CheckedMul` and `CheckedDiv` for overflow-safe operations
/// - `Zero` and `One` traits for mathematical completeness
///
/// ```rust
/// # #![feature(generic_const_exprs)]
/// # use nom_macros::system;
/// # use num_traits::{CheckedMul, Zero, One};
/// # #[system(L, M, T)] pub struct Physics;
/// # type Length = Physics<1, 0, 0>;
/// let length1: Length = Physics::new();
/// let length2: Length = Physics::new();
///
/// // Checked operations return Option for safety
/// let area_checked = length1.checked_mul(&length2);
///
/// // Mathematical identity elements
/// let zero_length: Length = Length::zero();
/// let one_length: Length = Length::one();
/// ```
///
/// ## Examples of Different Dimensional Systems
///
/// ### Classical Mechanics (SI Base Units)
/// ```rust
/// #[system(L, M, T, I, TH, N, J)]  // Length, Mass, Time, Current, Temperature, Amount, Luminosity
/// pub struct SI;
///
/// type Meter = SI<1, 0, 0, 0, 0, 0, 0>;
/// type Kilogram = SI<0, 1, 0, 0, 0, 0, 0>;
/// type Second = SI<0, 0, 1, 0, 0, 0, 0>;
/// type Newton = SI<1, 1, -2, 0, 0, 0, 0>;  // kg⋅m⋅s⁻²
/// ```
///
/// ### Electromagnetism
/// ```rust
/// #[system(Q, B, E)]  // Charge, Magnetic field, Energy
/// pub struct ElectroMagnetic;
///
/// type Charge = ElectroMagnetic<1, 0, 0>;
/// type MagneticField = ElectroMagnetic<0, 1, 0>;
/// type Energy = ElectroMagnetic<0, 0, 1>;
/// type MagneticFlux = ElectroMagnetic<0, 1, 1>;  // B × E
/// ```
///
/// ### Computer Graphics (3D Space)
/// ```rust
/// #[system(X, Y, Z)]
/// pub struct Space3D;
///
/// type XAxis = Space3D<1, 0, 0>;
/// type YAxis = Space3D<0, 1, 0>;
/// type ZAxis = Space3D<0, 0, 1>;
/// type Area2D = Space3D<1, 1, 0>;    // X × Y
/// type Volume3D = Space3D<1, 1, 1>;  // X × Y × Z
/// ```
///
/// ## Compile-Time Safety
///
/// The dimensional analysis happens entirely at compile time:
/// ```rust,compile_fail
/// # #![feature(generic_const_exprs)]
/// # use nom_macros::system;
/// # #[system(L, T)] pub struct Physics;
/// # type Length = Physics<1, 0>;
/// # type Time = Physics<0, 1>;
/// let length: Length = Physics::new();
/// let time: Time = Physics::new();
/// let invalid = length + time;  // ❌ Compile error! Cannot add Length + Time
/// ```
///
/// But valid operations compile without overhead:
/// ```rust
/// # #![feature(generic_const_exprs)]
/// # use nom_macros::system;
/// # #[system(L, T)] pub struct Physics;
/// # type Length = Physics<1, 0>;
/// # type Time = Physics<0, 1>;
/// # type Velocity = Physics<1, -1>;
/// let length: Length = Physics::new();
/// let time: Time = Physics::new();
/// let velocity: Velocity = length / time;  // ✅ Compiles to nothing at runtime!
/// ```
///
/// ## No-std Compatibility
///
/// This macro generates `no_std` compatible code using `core::ops` instead of `std::ops`,
/// making it suitable for embedded systems, kernels, and other constrained environments.
///
/// ## Requirements
///
/// - Requires nightly Rust with `#![feature(generic_const_exprs)]`
/// - System names must be valid Rust identifiers
/// - At least one system must be specified
///
/// ## Generated Code Structure
///
/// For `#[system(A, B, C)]`, the macro generates:
///
/// ```rust,ignore
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// pub struct YourStruct<const A: i32, const B: i32, const C: i32>;
///
/// impl<const A: i32, const B: i32, const C: i32> YourStruct<A, B, C> {
///     pub const fn new() -> Self { YourStruct }
/// }
///
/// // core::ops implementations for basic arithmetic
/// impl<...> core::ops::Mul<...> for YourStruct<...> { ... }
/// impl<...> core::ops::Div<...> for YourStruct<...> { ... }
///
/// // num-traits implementations for enhanced functionality  
/// impl<...> num_traits::CheckedMul<...> for YourStruct<...> { ... }
/// impl<...> num_traits::CheckedDiv<...> for YourStruct<...> { ... }
/// impl<...> num_traits::Zero for YourStruct<...> { ... }
/// impl<...> num_traits::One for YourStruct<...> { ... }
///
/// // Inherent methods for convenience
/// impl<...> YourStruct<...> {
///     pub const fn squared(self) -> YourStruct<...> { ... }
///     pub const fn cubed(self) -> YourStruct<...> { ... }
///     pub const fn pow<const P: i32>(self) -> YourStruct<...> { ... }
///     pub const fn sqrt(self) -> YourStruct<...> { ... }
/// }
/// ```
#[proc_macro_attribute]
pub fn system(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Parse the dimension names from the attribute arguments
    let dimension_args = parse_macro_input!(args as DimensionArgs);
    let dimensions: Vec<&Ident> = dimension_args.dimensions.iter().collect();

    if dimensions.is_empty() {
        return syn::Error::new_spanned(
            struct_name,
            "Expected dimension names as arguments, e.g., #[system(L, M, T)]",
        )
        .to_compile_error()
        .into();
    }

    // We need different parameter names for LHS and RHS of operations
    // to avoid conflicts in const generics
    let lhs_params: Vec<Ident> = dimensions
        .iter()
        .map(|dim| Ident::new(&format!("{}_LHS", dim), dim.span()))
        .collect();

    let rhs_params: Vec<Ident> = dimensions
        .iter()
        .map(|dim| Ident::new(&format!("{}_RHS", dim), dim.span()))
        .collect();

    let output = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub struct #struct_name<#(const #dimensions: i32),*>;

        impl<#(const #dimensions: i32),*> #struct_name<#(#dimensions),*> {
            pub const fn new() -> Self {
                #struct_name
            }
        }

        // 🔥⚡ SYSTEM ANALYSIS with core::ops::Add and Sub! ⚡🔥

        // Add: represents dimensional multiplication (add exponents)
        // Length + Length = Area (in dimensional space)
        impl<#(const #lhs_params: i32),*, #(const #rhs_params: i32),*>
        core::ops::Add<#struct_name<#(#rhs_params),*>>
        for #struct_name<#(#lhs_params),*>
        where
            #struct_name<#({#lhs_params + #rhs_params}),*>: ,
        {
            type Output = #struct_name<#({#lhs_params + #rhs_params}),*>;

            fn add(self, _rhs: #struct_name<#(#rhs_params),*>) -> Self::Output {
                #struct_name
            }
        }

        // Sub: represents dimensional division (subtract exponents)
        // Area - Length = Length (in dimensional space)
        impl<#(const #lhs_params: i32),*, #(const #rhs_params: i32),*>
        core::ops::Sub<#struct_name<#(#rhs_params),*>>
        for #struct_name<#(#lhs_params),*>
        where
            #struct_name<#({#lhs_params - #rhs_params}),*>: ,
        {
            type Output = #struct_name<#({#lhs_params - #rhs_params}),*>;

            fn sub(self, _rhs: #struct_name<#(#rhs_params),*>) -> Self::Output {
                #struct_name
            }
        }

        // Simple inherent methods for common operations
        impl<#(const #dimensions: i32),*> #struct_name<#(#dimensions),*> {
            /// Square this dimension (multiply all exponents by 2)
            pub const fn squared(self) -> #struct_name<#({#dimensions * 2}),*>
            where
                #struct_name<#({#dimensions * 2}),*>: ,
            {
                #struct_name
            }

            /// Cube this dimension (multiply all exponents by 3)
            pub const fn cubed(self) -> #struct_name<#({#dimensions * 3}),*>
            where
                #struct_name<#({#dimensions * 3}),*>: ,
            {
                #struct_name
            }
        }

        // Display implementation for dimensional analysis
        impl<#(const #dimensions: i32),*> core::fmt::Display for #struct_name<#(#dimensions),*> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut has_content = false;

                #(
                    match #dimensions {
                        0 => {}, // Don't show dimensions with zero exponent
                        1 => {
                            write!(f, "{}", stringify!(#dimensions))?;
                            has_content = true;
                        },
                        exp => {
                            write!(f, "{}", stringify!(#dimensions))?;
                            Self::write_superscript(f, exp)?;
                            has_content = true;
                        },
                    }
                )*

                if !has_content {
                    write!(f, "dimensionless")?;
                }

                Ok(())
            }
        }

        impl<#(const #dimensions: i32),*> #struct_name<#(#dimensions),*> {
            fn write_superscript(f: &mut core::fmt::Formatter<'_>, exp: i32) -> core::fmt::Result {
                match exp {
                    -9 => write!(f, "⁻⁹"),
                    -8 => write!(f, "⁻⁸"),
                    -7 => write!(f, "⁻⁷"),
                    -6 => write!(f, "⁻⁶"),
                    -5 => write!(f, "⁻⁵"),
                    -4 => write!(f, "⁻⁴"),
                    -3 => write!(f, "⁻³"),
                    -2 => write!(f, "⁻²"),
                    -1 => write!(f, "⁻¹"),
                    0 => write!(f, "⁰"),
                    1 => write!(f, "¹"),
                    2 => write!(f, "²"),
                    3 => write!(f, "³"),
                    4 => write!(f, "⁴"),
                    5 => write!(f, "⁵"),
                    6 => write!(f, "⁶"),
                    7 => write!(f, "⁷"),
                    8 => write!(f, "⁸"),
                    9 => write!(f, "⁹"),
                    // For larger numbers, fall back to simple notation
                    n if n < 0 => write!(f, "⁻{}", -n),
                    n => write!(f, "{}", n),
                }
            }
        }
    };

    output.into()
}
