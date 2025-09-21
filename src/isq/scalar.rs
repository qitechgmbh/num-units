use typenum::*;

// Dimensionless scalar quantities
units! {
    Unitless: "", "unitless";
}

// Ratios and percentages
units! {
    Percent: "%", "percent";
    PartsPerMillion: "ppm", "parts per million";
    PartsPerBillion: "ppb", "parts per billion";
}

// Unit conversions using convert_linear! with multiple conversions
crate::convert_linear! {
    Percent => Unitless: 1.0 / HECTO;              // 1% = 0.01 unitless
    PartsPerMillion => Unitless: 1.0 / MEGA;       // 1 ppm = 0.000001 unitless
    PartsPerBillion => Unitless: 1.0 / GIGA;       // 1 ppb = 0.000000001 unitless
}

// Angular units (dimensionless)
units! {
    Revolution: "rev", "revolution";
    Radian: "rad", "radian";
    Degree: "°", "degree";
}

convert_matrix! {
    Unitless => Percent, PartsPerMillion, PartsPerBillion
}

// Angular conversions using convert_linear!
crate::convert_linear! {
    Revolution => Unitless: 1.0;                       // 1 revolution = 1 unitless
    Radian => Unitless: 1.0 / (2.0 * std::f64::consts::PI);  // 1 radian = 1/(2π) revolutions
    Degree => Unitless: 1.0 / 360.0;                   // 1 degree = 1/360 revolutions
}

convert_matrix! {
    Unitless => Revolution, Radian, Degree
}

// Information
units! {
    Bit: "b", "bit";
    Nibble: "nb", "nibble";

    Byte: "B", "byte";
    Kilobyte: "kB", "kilobyte";
    Megabyte: "MB", "megabyte";
    Gigabyte: "GB", "gigabyte";
    Terabyte: "TB", "terabyte";
    Petabyte: "PB", "petabyte";
    Exabyte: "EB", "exabyte";
    Zettabyte: "ZB", "zettabyte";
    Yottabyte: "YB", "yottabyte";
    Kibibyte: "KiB", "kibibyte";
    Mebibyte: "MiB", "mebibyte";
    Gibibyte: "GiB", "gibibyte";
    Tebibyte: "TiB", "tebibyte";
    Pebibyte: "PiB", "pebibyte";
    Exbibyte: "EiB", "exbibyte";
    Zebibyte: "ZiB", "zebibyte";
    Yobibyte: "YiB", "yobibyte";
}

// Information unit conversions using convert_linear!
crate::convert_linear! {
    Bit => Unitless: 1.0;                      // 1 bit = 1 unitless
    Nibble => Unitless: 1.0 / 4.0;             // 1 nibble = 4 bits
    Byte => Unitless: 1.0 / OCTA;              // 1 byte = 8 bits
    Kilobyte => Unitless: 1.0 / (KILO * OCTA); // 1 kB = 8000 bits
    Megabyte => Unitless: 1.0 / (MEGA * OCTA); // 1 MB = 8,000,000 bits
    Gigabyte => Unitless: 1.0 / (GIGA * OCTA); // 1 GB = 8,000,000,000 bits
    Terabyte => Unitless: 1.0 / (TERA * OCTA); // 1 TB = 8,000,000,000,000 bits
    Petabyte => Unitless: 1.0 / (PETA * OCTA); // 1 PB
    Exabyte => Unitless: 1.0 / (EXA * OCTA);   // 1 EB
    Zettabyte => Unitless: 1.0 / (ZETTA * OCTA); // 1 ZB
    Yottabyte => Unitless: 1.0 / (YOTTA * OCTA); // 1 YB
    Kibibyte => Unitless: 1.0 / (KIBI * OCTA); // 1 KiB = 8192 bits
    Mebibyte => Unitless: 1.0 / (MEBI * OCTA); // 1 MiB
    Gibibyte => Unitless: 1.0 / (GIBI * OCTA); // 1 GiB
    Tebibyte => Unitless: 1.0 / (TEBI * OCTA); // 1 TiB
    Pebibyte => Unitless: 1.0 / (PEBI * OCTA); // 1 PiB
    Exbibyte => Unitless: 1.0 / (EXBI * OCTA); // 1 EiB
    Zebibyte => Unitless: 1.0 / (ZEBI * OCTA); // 1 ZiB
    Yobibyte => Unitless: 1.0 / (YOBI * OCTA); // 1 YiB
}

convert_matrix! {
    Unitless => Bit, Nibble, Byte, Kilobyte, Megabyte, Gigabyte, Terabyte, Petabyte, Exabyte, Zettabyte,
    Yottabyte, Kibibyte, Mebibyte, Gibibyte, Tebibyte, Pebibyte, Exbibyte, Zebibyte, Yobibyte
}

use crate::prefix::{
    EXA, EXBI, GIBI, GIGA, HECTO, KIBI, KILO, MEBI, MEGA, OCTA, PEBI, PETA, TEBI, TERA, YOBI,
    YOTTA, ZEBI, ZETTA,
};

// Apples
units! {
    Apple: "", "apple";
}

// Apple unit conversion
crate::convert_linear! {
    Apple => Unitless: 1.0;                    // 1 apple = 1 unitless
}

// Scalar quantity definition (dimensionless)
use super::{ISQ, SiScale};
quantity!(Scalar, ISQ<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, Unitless);

// Re-export types for convenience
pub use scalar::Scalar;
pub use scalar::*;
