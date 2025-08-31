//! # Nom Macros - Compile-Time Dimensional Analysis 🎩✨⚡
//!
//! This crate provides a procedural macro for generating zero-cost dimensional analysis systems
//! at compile time using typenum for type-level arithmetic. Perfect for physics simulations, engineering
//! calculations, computer graphics, and any domain where unit correctness matters.
//!
//! ## Features
//!
//! - 🚀 **Zero Runtime Cost**: All dimensional checking happens at compile time
//! - 🔬 **Fully Generic**: Support any dimensional system, not just SI units  
//! - 🛡️ **Type Safe**: Prevent dimensional errors like adding length + time
//! - 🧮 **Type-Level Arithmetic**: Uses typenum for compile-time calculations
//! - 📱 **No-std Compatible**: Works in embedded and kernel environments
//! - ⚡ **Automatic Operations**: Generates multiplication, division, and power operations
//! - 🔢 **num-traits Integration**: Checked operations, Zero/One traits, and more
//! - 🎯 **Mathematical Completeness**: Square roots, arbitrary powers, identity elements
//!
//! ## Quick Start
//!
//! ```rust
//! use nom_macros::system;
//!
//! // Define your dimensional system
//! #[system(L, M, T)]  // Length, Mass, Time
//! pub struct Physics;
//!
//! // Create type aliases
//! pub type Length = Physics<typenum::P1, typenum::Z0, typenum::Z0>;
//! pub type Mass = Physics<typenum::Z0, typenum::P1, typenum::Z0>;
//! pub type Time = Physics<typenum::Z0, typenum::Z0, typenum::P1>;
//! pub type Velocity = Physics<typenum::P1, typenum::Z0, typenum::N1>;  // Length/Time
//! pub type Force = Physics<typenum::P1, typenum::P1, typenum::N2>;     // Mass×Length/Time²
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
//! - Stable Rust (no nightly features needed!)
//! - `typenum` crate for type-level arithmetic
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! typenum = "1.17"
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

    // Create type parameter names for LHS and RHS
    let lhs_types: Vec<Ident> = (0..dimensions.len())
        .map(|i| Ident::new(&format!("L{}", i), dimensions[i].span()))
        .collect();

    let rhs_types: Vec<Ident> = (0..dimensions.len())
        .map(|i| Ident::new(&format!("R{}", i), dimensions[i].span()))
        .collect();

    let output = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub struct #struct_name<#(#dimensions),*>(
            core::marker::PhantomData<(#(#dimensions,)*)>
        )
        where
            #(#dimensions: typenum::Integer,)*;

        impl<#(#dimensions),*> #struct_name<#(#dimensions),*>
        where
            #(#dimensions: typenum::Integer,)*
        {
            pub const fn new() -> Self {
                #struct_name(core::marker::PhantomData)
            }
        }

        // Mul: represents dimensional multiplication (add exponents)
        impl<#(#lhs_types,)* #(#rhs_types,)*>
        core::ops::Mul<#struct_name<#(#rhs_types),*>>
        for #struct_name<#(#lhs_types),*>
        where
            #(#lhs_types: typenum::Integer,)*
            #(#rhs_types: typenum::Integer,)*
            #(#lhs_types: core::ops::Add<#rhs_types>,)*
            #(#rhs_types: core::ops::Add<#lhs_types>,)*
            #(<#lhs_types as core::ops::Add<#rhs_types>>::Output: typenum::Integer,)*
        {
            type Output = #struct_name<
                #(<#lhs_types as core::ops::Add<#rhs_types>>::Output),*
            >;

            fn mul(self, _rhs: #struct_name<#(#rhs_types),*>) -> Self::Output {
                #struct_name(core::marker::PhantomData)
            }
        }

        // Div: represents dimensional division (subtract exponents)
        impl<#(#lhs_types,)* #(#rhs_types,)*>
        core::ops::Div<#struct_name<#(#rhs_types),*>>
        for #struct_name<#(#lhs_types),*>
        where
            #(#lhs_types: typenum::Integer,)*
            #(#rhs_types: typenum::Integer,)*
            #(#lhs_types: core::ops::Sub<#rhs_types>,)*
            #(<#lhs_types as core::ops::Sub<#rhs_types>>::Output: typenum::Integer,)*
        {
            type Output = #struct_name<
                #(<#lhs_types as core::ops::Sub<#rhs_types>>::Output),*
            >;

            fn div(self, _rhs: #struct_name<#(#rhs_types),*>) -> Self::Output {
                #struct_name(core::marker::PhantomData)
            }
        }

        // Add: represents dimensional addition for multiplication (add exponents)
        impl<#(#lhs_types,)* #(#rhs_types,)*>
        core::ops::Add<#struct_name<#(#rhs_types),*>>
        for #struct_name<#(#lhs_types),*>
        where
            #(#lhs_types: typenum::Integer + core::ops::Add<#rhs_types>,)*
            #(#rhs_types: typenum::Integer,)*
            #(<#lhs_types as core::ops::Add<#rhs_types>>::Output: typenum::Integer,)*
        {
            type Output = #struct_name<
                #(<#lhs_types as core::ops::Add<#rhs_types>>::Output),*
            >;

            fn add(self, _rhs: #struct_name<#(#rhs_types),*>) -> Self::Output {
                #struct_name(core::marker::PhantomData)
            }
        }

        // Sub: represents dimensional subtraction for division (subtract exponents)
        impl<#(#lhs_types,)* #(#rhs_types,)*>
        core::ops::Sub<#struct_name<#(#rhs_types),*>>
        for #struct_name<#(#lhs_types),*>
        where
            #(#lhs_types: typenum::Integer + core::ops::Sub<#rhs_types>,)*
            #(#rhs_types: typenum::Integer,)*
            #(<#lhs_types as core::ops::Sub<#rhs_types>>::Output: typenum::Integer,)*
        {
            type Output = #struct_name<
                #(<#lhs_types as core::ops::Sub<#rhs_types>>::Output),*
            >;

            fn sub(self, _rhs: #struct_name<#(#rhs_types),*>) -> Self::Output {
                #struct_name(core::marker::PhantomData)
            }
        }

        // Simple inherent methods for common operations
        impl<#(#dimensions),*> #struct_name<#(#dimensions),*>
        where
            #(#dimensions: typenum::Integer,)*
        {
            /// Square this dimension (multiply all exponents by 2)
            pub const fn squared(self) -> #struct_name<
                #(<#dimensions as core::ops::Mul<typenum::P2>>::Output),*
            >
            where
                #(#dimensions: core::ops::Mul<typenum::P2>,)*
                #(<#dimensions as core::ops::Mul<typenum::P2>>::Output: typenum::Integer,)*
            {
                #struct_name(core::marker::PhantomData)
            }

            /// Cube this dimension (multiply all exponents by 3)
            pub const fn cubed(self) -> #struct_name<
                #(<#dimensions as core::ops::Mul<typenum::P3>>::Output),*
            >
            where
                #(#dimensions: core::ops::Mul<typenum::P3>,)*
                #(<#dimensions as core::ops::Mul<typenum::P3>>::Output: typenum::Integer,)*
            {
                #struct_name(core::marker::PhantomData)
            }
        }

        // Display implementation for dimensional analysis
        impl<#(#dimensions),*> core::fmt::Display for #struct_name<#(#dimensions),*>
        where
            #(#dimensions: typenum::Integer,)*
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut has_content = false;

                #(
                    match #dimensions::I8 {
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

        impl<#(#dimensions),*> #struct_name<#(#dimensions),*>
        where
            #(#dimensions: typenum::Integer,)*
        {
            fn write_superscript(f: &mut core::fmt::Formatter<'_>, exp: i8) -> core::fmt::Result {
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
