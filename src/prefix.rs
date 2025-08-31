/// # Unit Prefixes - SI Prefix Constants
///
/// This module defines all standard SI unit prefixes as compile-time constants.
/// These prefixes are used throughout the dimensional analysis system for
/// creating derived units like kilometers, milligrams, microseconds, etc.
///
/// ## SI Prefixes
///
/// The International System of Units (SI) defines a set of prefixes for
/// powers of ten. These prefixes allow expressing very large or very small
/// quantities conveniently.
///
/// ## Usage
///
/// ```rust
/// use num_units::prefix::{KILO, MILLI, MICRO};
///
/// // Use in unit conversions
/// let kilometers = meters / KILO;  // Convert meters to kilometers
/// let milligrams = grams / MILLI;  // Convert grams to milligrams
/// let microseconds = seconds / MICRO;  // Convert seconds to microseconds
/// ```
///
/// ## Available Prefixes
///
/// ### Decimal Prefixes (10^n)
/// - **Yocto (y)**: 10^-24
/// - **Zepto (z)**: 10^-21
/// - **Atto (a)**: 10^-18
/// - **Femto (f)**: 10^-15
/// - **Pico (p)**: 10^-12
/// - **Nano (n)**: 10^-9
/// - **Micro (μ)**: 10^-6
/// - **Milli (m)**: 10^-3
/// - **Centi (c)**: 10^-2
/// - **Deci (d)**: 10^-1
///
/// ### Unit Prefixes (10^0)
/// - **Unit**: 10^0 (no prefix)
///
/// ### Decimal Multipliers (10^n)
/// - **Deca (da)**: 10^1
/// - **Hecto (h)**: 10^2
/// - **Kilo (k)**: 10^3
/// - **Mega (M)**: 10^6
/// - **Giga (G)**: 10^9
/// - **Tera (T)**: 10^12
/// - **Peta (P)**: 10^15
/// - **Exa (E)**: 10^18
/// - **Zetta (Z)**: 10^21
/// - **Yotta (Y)**: 10^24
/// - **Ronna (R)**: 10^27
/// - **Quetta (Q)**: 10^30

// Sub-yocto prefixes (very small)
pub const YOCTO: f64 = 1e-24;
pub const ZEPTO: f64 = 1e-21;
pub const ATTO: f64 = 1e-18;
pub const FEMTO: f64 = 1e-15;
pub const PICO: f64 = 1e-12;
pub const NANO: f64 = 1e-9;
pub const MICRO: f64 = 1e-6;
pub const MILLI: f64 = 1e-3;
pub const CENTI: f64 = 1e-2;
pub const DECI: f64 = 1e-1;

// Unit (no prefix)
pub const UNIT: f64 = 1e0;

// Decimal multipliers (large)
pub const DECA: f64 = 1e1;
pub const HECTO: f64 = 1e2;
pub const KILO: f64 = 1e3;
pub const MEGA: f64 = 1e6;
pub const GIGA: f64 = 1e9;
pub const TERA: f64 = 1e12;
pub const PETA: f64 = 1e15;
pub const EXA: f64 = 1e18;
pub const ZETTA: f64 = 1e21;
pub const YOTTA: f64 = 1e24;
pub const RONNA: f64 = 1e27;
pub const QUECCA: f64 = 1e30;
