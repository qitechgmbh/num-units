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

// Percentage conversions
convert! {
    Percent: |unitless| unitless * HECTO;
    Unitless: |percent| percent / HECTO;
}

// Parts per million conversions
convert! {
    PartsPerMillion: |unitless| unitless * MEGA;
    Unitless: |parts_per_million| parts_per_million / MEGA;
}

// Parts per billion conversions
convert! {
    PartsPerBillion: |unitless| unitless * GIGA;
    Unitless: |parts_per_billion| parts_per_billion / GIGA;
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

// Angular conversions
convert! {
    Revolution: |unitless| unitless;
    Unitless: |revolution| revolution;
}

convert! {
    Radian: |unitless| unitless * 2.0 * std::f64::consts::PI;
    Unitless: |radian| radian / (2.0 * std::f64::consts::PI);
}

convert! {
    Degree: |unitless| unitless * 360.0;
    Unitless: |degree| degree / 360.0;
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

convert! {
    Bit: |unitless| unitless;
    Unitless: |bit| bit;
}

convert! {
    Nibble: |unitless| unitless * 4.0;
    Unitless: |nibble| nibble / 4.0;
}

convert! {
    Byte: |unitless| unitless * OCTA;
    Unitless: |byte| byte / OCTA;
}

convert! {
    Kilobyte: |unitless| unitless * KILO * OCTA;
    Unitless: |kilobyte| kilobyte / (KILO * OCTA);
}

convert! {
    Megabyte: |unitless| unitless * MEGA * OCTA;
    Unitless: |megabyte| megabyte / (MEGA * OCTA);
}

convert! {
    Gigabyte: |unitless| unitless * GIGA * OCTA;
    Unitless: |gigabyte| gigabyte / (GIGA * OCTA);
}

convert! {
    Terabyte: |unitless| unitless * TERA * OCTA;
    Unitless: |terabyte| terabyte / (TERA * OCTA);
}

convert! {
    Petabyte: |unitless| unitless * PETA * OCTA;
    Unitless: |petabyte| petabyte / (PETA * OCTA);
}

convert! {
    Exabyte: |unitless| unitless * EXA * OCTA;
    Unitless: |exabyte| exabyte / (EXA * OCTA);
}

convert! {
    Zettabyte: |unitless| unitless * ZETTA * OCTA;
    Unitless: |zettabyte| zettabyte / (ZETTA * OCTA);
}

convert! {
    Yottabyte: |unitless| unitless * YOTTA * OCTA;
    Unitless: |yottabyte| yottabyte / (YOTTA * OCTA);
}

convert! {
    Kibibyte: |unitless| unitless * KIBI * OCTA;
    Unitless: |kibibyte| kibibyte / (KIBI * OCTA);
}

convert! {
    Mebibyte: |unitless| unitless * MEBI * OCTA;
    Unitless: |mebibyte| mebibyte / (MEBI * OCTA);
}

convert! {
    Gibibyte: |unitless| unitless * GIBI * OCTA;
    Unitless: |gibibyte| gibibyte / (GIBI * OCTA);
}

convert! {
    Tebibyte: |unitless| unitless * TEBI * OCTA;
    Unitless: |tebibyte| tebibyte / (TEBI * OCTA);
}

convert! {
    Pebibyte: |unitless| unitless * PEBI * OCTA;
    Unitless: |pebibyte| pebibyte / (PEBI * OCTA);
}

convert! {
    Exbibyte: |unitless| unitless * EXBI * OCTA;
    Unitless: |exbibyte| exbibyte / (EXBI * OCTA);
}

convert! {
    Zebibyte: |unitless| unitless * ZEBI * OCTA;
    Unitless: |zebibyte| zebibyte / (ZEBI * OCTA);
}

convert! {
    Yobibyte: |unitless| unitless * YOBI * OCTA;
    Unitless: |yobibyte| yobibyte / (YOBI * OCTA);
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

convert! {
    Apple: |unitless| unitless;
    Unitless: |apple| apple;
}

// Scalar quantity definition (dimensionless)
use super::{ISQ, SiScale};
quantity!(Scalar, ISQ<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, SiScale, Unitless);

// Re-export types for convenience
pub use scalar::Scalar;
pub use scalar::*;
