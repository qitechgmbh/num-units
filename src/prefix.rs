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
/// ```rust,ignore
/// use num_units::prefix::{KILO, MILLI, MICRO};
///
/// // Use in unit conversions
/// let meters = 1000.0;
/// let grams = 1000.0;
/// let seconds = 1000000.0;
/// let kilometers = meters / KILO;  // Convert meters to kilometers
/// let milligrams = grams / MILLI;  // Convert grams to milligrams
/// let microseconds = seconds / MICRO;  // Convert seconds to microseconds
/// ```

// Sub-yocto prefixes (very small)
pub const YOCTO: f64 = 1e-24;
pub const ZEPTO: f64 = 1e-21;
pub const ATTO: f64 = 1e-18;
pub const FEMTO: f64 = 1e-15;
pub const PICO: f64 = 1e-12;
pub const NANO: f64 = 1e-9;
pub const MICRO: f64 = 1e-6;
pub const DECIMILLI: f64 = 1e-4;
pub const MILLI: f64 = 1e-3;
pub const CENTI: f64 = 1e-2;
pub const DECI: f64 = 1e-1;

// Unit (no prefix)
pub const ONE: f64 = 1e0;

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

// Binary multipliers (powers of 1024)
pub const KIBI: f64 = 1024.0; // 2^10
pub const MEBI: f64 = 1_048_576.0; // 2^20
pub const GIBI: f64 = 1_073_741_824.0; // 2^30
pub const TEBI: f64 = 1_099_511_627_776.0; // 2^40
pub const PEBI: f64 = 1_125_899_906842_624.0; // 2^50
pub const EXBI: f64 = 1_152_921_504606_846_976.0; // 2^60
pub const ZEBI: f64 = 1_180_591_620717_411_303_424.0; // 2^70
pub const YOBI: f64 = 1_208_925_819614_629_174_706_176.0; // 2^80

// Latin prefixes (very large)
pub const MONO: f64 = 1.0;
pub const DUO: f64 = 2.0;
pub const TRI: f64 = 3.0;
pub const TETRA: f64 = 4.0;
pub const PENTA: f64 = 5.0;
pub const HEXA: f64 = 6.0;
pub const HEPTA: f64 = 7.0;
pub const OCTA: f64 = 8.0;
pub const ENNA: f64 = 9.0;

// Scales
pub const MILLION: f64 = 1e6;
pub const BILLION: f64 = 1e9;
pub const TRILLION: f64 = 1e12;
pub const QUADRILLION: f64 = 1e15;
pub const QUINTILLION: f64 = 1e18;
pub const SEXTILLION: f64 = 1e21;
pub const SEPTILLION: f64 = 1e24;
pub const OCTILLION: f64 = 1e27;
pub const NONILLION: f64 = 1e30;
pub const DECILLION: f64 = 1e33;

// Time
pub const MINUTE: f64 = 60.0; // 60 seconds
pub const HOUR: f64 = 3600.0; // 60 minutes
pub const DAY: f64 = 86400.0; // 24 hours

// Information
pub const BIT: f64 = 1.0; // 1 bit
pub const BYTE: f64 = 8.0; // 8 bits
pub const WORD: f64 = 16.0; // 16 bits (2 bytes)
