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
convert_unit! {
    Percent: |unitless| unitless * HECTO;
    Unitless: |percent| percent / HECTO;
}

// Parts per million conversions
convert_unit! {
    PartsPerMillion: |unitless| unitless * MEGA;
    Unitless: |parts_per_million| parts_per_million / MEGA;
}

// Parts per billion conversions
convert_unit! {
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
convert_unit! {
    Revolution: |unitless| unitless;
    Unitless: |revolution| revolution;
}

convert_unit! {
    Radian: |unitless| unitless * 2.0 * std::f64::consts::PI;
    Unitless: |radian| radian / (2.0 * std::f64::consts::PI);
}

convert_unit! {
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

convert_unit! {
    Bit: |unitless| unitless;
    Unitless: |bit| bit;
}

convert_unit! {
    Nibble: |unitless| unitless * 4.0;
    Unitless: |nibble| nibble / 4.0;
}

convert_unit! {
    Byte: |unitless| unitless * OCTA;
    Unitless: |byte| byte / OCTA;
}

convert_unit! {
    Kilobyte: |unitless| unitless * KILO * OCTA;
    Unitless: |kilobyte| kilobyte / (KILO * OCTA);
}

convert_unit! {
    Megabyte: |unitless| unitless * MEGA * OCTA;
    Unitless: |megabyte| megabyte / (MEGA * OCTA);
}

convert_unit! {
    Gigabyte: |unitless| unitless * GIGA * OCTA;
    Unitless: |gigabyte| gigabyte / (GIGA * OCTA);
}

convert_unit! {
    Terabyte: |unitless| unitless * TERA * OCTA;
    Unitless: |terabyte| terabyte / (TERA * OCTA);
}

convert_unit! {
    Petabyte: |unitless| unitless * PETA * OCTA;
    Unitless: |petabyte| petabyte / (PETA * OCTA);
}

convert_unit! {
    Exabyte: |unitless| unitless * EXA * OCTA;
    Unitless: |exabyte| exabyte / (EXA * OCTA);
}

convert_unit! {
    Zettabyte: |unitless| unitless * ZETTA * OCTA;
    Unitless: |zettabyte| zettabyte / (ZETTA * OCTA);
}

convert_unit! {
    Yottabyte: |unitless| unitless * YOTTA * OCTA;
    Unitless: |yottabyte| yottabyte / (YOTTA * OCTA);
}

convert_unit! {
    Kibibyte: |unitless| unitless * KIBI * OCTA;
    Unitless: |kibibyte| kibibyte / (KIBI * OCTA);
}

convert_unit! {
    Mebibyte: |unitless| unitless * MEBI * OCTA;
    Unitless: |mebibyte| mebibyte / (MEBI * OCTA);
}

convert_unit! {
    Gibibyte: |unitless| unitless * GIBI * OCTA;
    Unitless: |gibibyte| gibibyte / (GIBI * OCTA);
}

convert_unit! {
    Tebibyte: |unitless| unitless * TEBI * OCTA;
    Unitless: |tebibyte| tebibyte / (TEBI * OCTA);
}

convert_unit! {
    Pebibyte: |unitless| unitless * PEBI * OCTA;
    Unitless: |pebibyte| pebibyte / (PEBI * OCTA);
}

convert_unit! {
    Exbibyte: |unitless| unitless * EXBI * OCTA;
    Unitless: |exbibyte| exbibyte / (EXBI * OCTA);
}

convert_unit! {
    Zebibyte: |unitless| unitless * ZEBI * OCTA;
    Unitless: |zebibyte| zebibyte / (ZEBI * OCTA);
}

convert_unit! {
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

convert_unit! {
    Apple: |unitless| unitless;
    Unitless: |apple| apple;
}

// Scalar quantity definition (dimensionless)
use super::{SI, SIScale};
quantity!(Scalar, SI<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, SIScale, Unitless);

// Re-export types for convenience
pub use scalar::Scalar;
pub use scalar::*;
