use crate::prefix::{
    EXA, EXBI, GIBI, GIGA, KIBI, KILO, MEBI, MEGA, OCTA, PEBI, PETA, TEBI, TERA, YOBI,
    YOTTA, ZEBI, ZETTA,
};

// Information units
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

crate::convert_matrix! {
    Unitless => Bit, Nibble, Byte, Kilobyte, Megabyte, Gigabyte, Terabyte, Petabyte, Exabyte, Zettabyte,
    Yottabyte, Kibibyte, Mebibyte, Gibibyte, Tebibyte, Pebibyte, Exbibyte, Zebibyte, Yobibyte
}

// Import Unitless from unitless module
use super::scalar::Unitless;