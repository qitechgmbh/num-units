/// # Information Units - Digital Information Measurements
///
/// This module defines digital information units and their conversions. Information is a
/// dimensionless quantity measured in bits, bytes, and their derivatives.
///
/// ## Important Note
///
/// Due to architectural limitations in num-units (lack of "kind" system like UOM),
/// information units cannot have a separate quantity type. Instead, they are implemented
/// using the existing Unitless base quantity. This means:
/// - Information quantities are treated as dimensionless scalars
/// - UOM compatibility tests cannot be performed for information units
/// - All conversions are done through the Unitless base unit
///
/// ## Base Units
///
/// - **Bit (b)**: The fundamental unit of information
/// - **Byte (B)**: 8 bits
///
/// ## SI/Binary Prefixed Units
///
/// ### Decimal (SI) prefixes:
/// - **Kilobit (kb)**: 10³ bits
/// - **Megabit (Mb)**: 10⁶ bits
/// - **Gigabit (Gb)**: 10⁹ bits
/// - **Terabit (Tb)**: 10¹² bits
/// - **Petabit (Pb)**: 10¹⁵ bits
/// - **Exabit (Eb)**: 10¹⁸ bits
/// - **Zettabit (Zb)**: 10²¹ bits
/// - **Yottabit (Yb)**: 10²⁴ bits
///
/// - **Kilobyte (kB)**: 10³ bytes
/// - **Megabyte (MB)**: 10⁶ bytes
/// - **Gigabyte (GB)**: 10⁹ bytes
/// - **Terabyte (TB)**: 10¹² bytes
/// - **Petabyte (PB)**: 10¹⁵ bytes
/// - **Exabyte (EB)**: 10¹⁸ bytes
/// - **Zettabyte (ZB)**: 10²¹ bytes
/// - **Yottabyte (YB)**: 10²⁴ bytes
///
/// ### Binary (IEC) prefixes:
/// - **Kibibit (Kib)**: 2¹⁰ bits
/// - **Mebibit (Mib)**: 2²⁰ bits
/// - **Gibibit (Gib)**: 2³⁰ bits
/// - **Tebibit (Tib)**: 2⁴⁰ bits
/// - **Pebibit (Pib)**: 2⁵⁰ bits
/// - **Exbibit (Eib)**: 2⁶⁰ bits
/// - **Zebibit (Zib)**: 2⁷⁰ bits
/// - **Yobibit (Yib)**: 2⁸⁰ bits
///
/// - **Kibibyte (KiB)**: 2¹⁰ bytes
/// - **Mebibyte (MiB)**: 2²⁰ bytes
/// - **Gibibyte (GiB)**: 2³⁰ bytes
/// - **Tebibyte (TiB)**: 2⁴⁰ bytes
/// - **Pebibyte (PiB)**: 2⁵⁰ bytes
/// - **Exbibyte (EiB)**: 2⁶⁰ bytes
/// - **Zebibyte (ZiB)**: 2⁷⁰ bytes
/// - **Yobibyte (YiB)**: 2⁸⁰ bytes
///
/// ## Other Units
///
/// - **Octet (o)**: 8 bits (synonym for byte)
/// - **Nibble**: 4 bits
/// - **Crumb**: 2 bits
/// - **Shannon (Sh)**: 1 bit (synonym for bit)
/// - **Natural Unit of Information (nat)**: ln(2) bits ≈ 0.693 bits
/// - **Trit**: log₂(3) bits ≈ 1.585 bits
/// - **Hartley (Hart)**: log₂(10) bits ≈ 3.322 bits
/// - **Deciban**: log₂(10)/10 bits ≈ 0.332 bits
///
/// ## Usage
///
/// ```rust,ignore
/// use num_units::si::scalar::Unitless;
/// use num_units::si::information::{Bit, Byte, Kilobyte, Gibibyte};
///
/// // Create information quantities as dimensionless scalars
/// let bits = Unitless::from::<Bit>(1024.0);
/// let bytes = Unitless::from::<Byte>(128.0);
/// let kb = Unitless::from::<Kilobyte>(1.5);
/// let gib = Unitless::from::<Gibibyte>(2.0);
///
/// // Convert between units
/// let bits_from_bytes = bytes.to::<Bit>();     // 1024.0 bits
/// let bytes_from_kb = kb.to::<Byte>();         // 1500.0 bytes
/// let mb_from_gib = gib.to::<Megabyte>();      // 2048.0 MB
///
/// // Arithmetic operations
/// let total = bits + bytes_from_kb;            // Automatic conversion
/// ```
///
/// ## Architecture
///
/// Information units are implemented as dimensionless quantities using Unitless as the base.
/// This is a limitation of the num-units framework which lacks UOM's "kind" system for
/// distinguishing quantities with identical dimensions.
use crate::prefix::*;

// Base-2 information units (bits)
units! {
    Yobibit: "Yib", "yobibit";
    Yottabit: "Yb", "yottabit";
    Zebibit: "Zib", "zebibit";
    Zettabit: "Zb", "zettabit";
    Exbibit: "Eib", "exbibit";
    Exabit: "Eb", "exabit";
    Pebibit: "Pib", "pebibit";
    Petabit: "Pb", "petabit";
    Tebibit: "Tib", "tibibit";
    Terabit: "Tb", "terabit";
    Gibibit: "Gib", "gibibit";
    Gigabit: "Gb", "gigabit";
    Mebibit: "Mib", "mebibit";
    Megabit: "Mb", "megabit";
    Kibibit: "Kib", "kibibit";
    Kilobit: "kb", "kilobit";
    Bit: "b", "bit";
}

// Base-2 information units (bytes)
units! {
    Yobibyte: "YiB", "yobibyte";
    Yottabyte: "YB", "yottabyte";
    Zebibyte: "ZiB", "zebibyte";
    Zettabyte: "ZB", "zettabyte";
    Exbibyte: "EiB", "exbibyte";
    Exabyte: "EB", "exabyte";
    Pebibyte: "PiB", "pebibyte";
    Petabyte: "PB", "petabyte";
    Tebibyte: "TiB", "tibibyte";
    Terabyte: "TB", "terabyte";
    Gibibyte: "GiB", "gibibyte";
    Gigabyte: "GB", "gigabyte";
    Mebibyte: "MiB", "mebibyte";
    Megabyte: "MB", "megabyte";
    Kibibyte: "KiB", "kibibyte";
    Kilobyte: "kB", "kilobyte";
    Byte: "B", "byte";
}

// Other information units
units! {
    Octet: "o", "octet";
    Nibble: "nibble", "nibble";
    Crumb: "crumb", "crumb";
    Word: "word", "word";
}

// ===== CONVERSION RELATIONSHIPS =====

// Information unit conversions using convert_linear! with exact UOM coefficients
// All conversions are to Unitless (dimensionless base)
crate::convert_linear! {
    // Base-2 bits
    Yobibit => Unitless: YOBI;
    Yottabit => Unitless: YOTTA;
    Zebibit => Unitless: ZEBI;
    Zettabit => Unitless: ZETTA;
    Exbibit => Unitless: EXBI;
    Exabit => Unitless: EXA;
    Pebibit => Unitless: PEBI;
    Petabit => Unitless: PETA;
    Tebibit => Unitless: TEBI;
    Terabit => Unitless: TERA;
    Gibibit => Unitless: GIBI;
    Gigabit => Unitless: GIGA;
    Mebibit => Unitless: MEBI;
    Megabit => Unitless: MEGA;
    Kibibit => Unitless: KIBI;
    Kilobit => Unitless: KILO;
    Bit => Unitless: ONE;

    // Base-2 bytes - exact UOM coefficients
    Yobibyte => Unitless: YOBI * BYTE;
    Yottabyte => Unitless: YOTTA * BYTE;
    Zebibyte => Unitless: ZEBI * BYTE;
    Zettabyte => Unitless: ZETTA * BYTE;
    Exbibyte => Unitless: EXBI * BYTE;
    Exabyte => Unitless: EXA * BYTE;
    Pebibyte => Unitless: PEBI * BYTE;
    Petabyte => Unitless: PETA * BYTE;
    Tebibyte => Unitless: TEBI * BYTE;
    Terabyte => Unitless: TERA * BYTE;
    Gibibyte => Unitless: GIBI * BYTE;
    Gigabyte => Unitless: GIGA * BYTE;
    Mebibyte => Unitless: MEBI * BYTE;
    Megabyte => Unitless: MEGA * BYTE;
    Kibibyte => Unitless: KIBI * BYTE;
    Kilobyte => Unitless: KILO * BYTE;
    Byte => Unitless: BYTE;

    // Base-2 other units
    Octet => Unitless: BYTE;
    Nibble => Unitless: TETRA;
    Crumb => Unitless: DUO;
    Word => Unitless: WORD;
}

crate::convert_matrix! {
    Unitless => Yobibit, Yottabit, Zebibit, Zettabit, Exbibit, Exabit, Pebibit, Petabit, Tebibit, Terabit, Gibibit, Gigabit, Mebibit, Megabit, Kibibit, Kilobit, Bit, Yobibyte, Yottabyte, Zebibyte, Zettabyte, Exbibyte, Exabyte, Pebibyte, Petabyte, Tebibyte, Terabyte, Gibibyte, Gigabyte, Mebibyte, Megabyte, Kibibyte, Kilobyte, Byte, Octet, Nibble, Crumb, Shannon, NaturalUnitOfInformation, Trit, Hartley, Deciban
}

// Import Unitless from scalar module
use super::scalar::Unitless;
