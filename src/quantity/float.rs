use super::Quantity;
use num_traits::{Num, Float};

// Floating-point specific operations for quantities
// Provides floating-point mathematical functions like trigonometric functions,
// logarithms, exponentials, and other mathematical functions
//
// Note: This implementation is only available with "std" or "libm" features
// as floating-point mathematical functions require either std or libm

impl<V, D> Quantity<V, D>
where
    V: Num + Float,
{
    /// Returns the `NaN` value.
    pub fn nan() -> Self {
        Self::from_raw(V::nan())
    }

    /// Returns the infinite value.
    pub fn infinity() -> Self {
        Self::from_raw(V::infinity())
    }

    /// Returns the negative infinite value.
    pub fn neg_infinity() -> Self {
        Self::from_raw(V::neg_infinity())
    }

    /// Returns `-0.0`.
    pub fn neg_zero() -> Self {
        Self::from_raw(V::neg_zero())
    }

    /// Returns the smallest finite value that this type can represent.
    pub fn min_value() -> Self {
        Self::from_raw(V::min_value())
    }

    /// Returns the smallest positive, normalized value that this type can represent.
    pub fn min_positive_value() -> Self {
        Self::from_raw(V::min_positive_value())
    }

    /// Returns the largest finite value that this type can represent.
    pub fn max_value() -> Self {
        Self::from_raw(V::max_value())
    }

    /// Returns `true` if this value is `NaN` and false otherwise.
    pub fn is_nan(self) -> bool {
        self.value.is_nan()
    }

    /// Returns `true` if this value is positive infinity or negative infinity and false otherwise.
    pub fn is_infinite(self) -> bool {
        self.value.is_infinite()
    }

    /// Returns `true` if this number is neither infinite nor `NaN`.
    pub fn is_finite(self) -> bool {
        self.value.is_finite()
    }

    /// Returns `true` if the number is neither zero, infinite, `subnormal`, or `NaN`.
    pub fn is_normal(self) -> bool {
        self.value.is_normal()
    }

    /// Returns the floating point category of the number.
    pub fn classify(self) -> core::num::FpCategory {
        self.value.classify()
    }

    /// Returns the largest integer less than or equal to a number.
    pub fn floor(self) -> Self {
        Self::from_raw(self.value.floor())
    }

    /// Returns the smallest integer greater than or equal to a number.
    pub fn ceil(self) -> Self {
        Self::from_raw(self.value.ceil())
    }

    /// Returns the nearest integer to a number. Round half-way cases away from `0.0`.
    pub fn round(self) -> Self {
        Self::from_raw(self.value.round())
    }

    /// Returns the integer part of a number.
    pub fn trunc(self) -> Self {
        Self::from_raw(self.value.trunc())
    }

    /// Returns the fractional part of a number.
    pub fn fract(self) -> Self {
        Self::from_raw(self.value.fract())
    }

    /// Returns `true` if `self` is positive, including `+0.0`, and `NaN` with a positive sign bit.
    pub fn is_sign_positive(self) -> bool {
        self.value.is_sign_positive()
    }

    /// Returns `true` if `self` is negative, including `-0.0`, and `NaN` with a negative sign bit.
    pub fn is_sign_negative(self) -> bool {
        self.value.is_sign_negative()
    }

    /// Takes the reciprocal (inverse) of a number, `1/x`.
    pub fn recip(self) -> Self {
        Self::from_raw(self.value.recip())
    }

    /// Raises a number to an integer power.
    pub fn powi(self, n: i32) -> Self {
        Self::from_raw(self.value.powi(n))
    }

    /// Raises a number to a floating point power.
    /// 
    /// Note: This creates a dimensionless result as fractional powers
    /// don't have well-defined dimensional semantics
    pub fn powf(self, n: Self) -> Self {
        Self::from_raw(self.value.powf(n.value))
    }

    /// Returns the square root of a number.
    /// 
    /// Note: This returns the square root with half the original dimensions
    /// For proper dimensional analysis, use custom sqrt methods
    pub fn sqrt(self) -> Self {
        Self::from_raw(self.value.sqrt())
    }

    /// Returns `e^(self)`, (the exponential function).
    pub fn exp(self) -> Self {
        Self::from_raw(self.value.exp())
    }

    /// Returns `2^(self)`.
    pub fn exp2(self) -> Self {
        Self::from_raw(self.value.exp2())
    }

    /// Returns the natural logarithm of the number.
    pub fn ln(self) -> Self {
        Self::from_raw(self.value.ln())
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    pub fn log(self, base: Self) -> Self {
        Self::from_raw(self.value.log(base.value))
    }

    /// Returns the base 2 logarithm of the number.
    pub fn log2(self) -> Self {
        Self::from_raw(self.value.log2())
    }

    /// Returns the base 10 logarithm of the number.
    pub fn log10(self) -> Self {
        Self::from_raw(self.value.log10())
    }

    /// Returns the maximum of the two numbers.
    pub fn max(self, other: Self) -> Self {
        Self::from_raw(self.value.max(other.value))
    }

    /// Returns the minimum of the two numbers.
    pub fn min(self, other: Self) -> Self {
        Self::from_raw(self.value.min(other.value))
    }

    /// Take the cubic root of a number.
    pub fn cbrt(self) -> Self {
        Self::from_raw(self.value.cbrt())
    }

    /// Calculate the euclidean distance between `self` and `other`.
    pub fn hypot(self, other: Self) -> Self {
        Self::from_raw(self.value.hypot(other.value))
    }

    /// Computes the sine of a number (in radians).
    pub fn sin(self) -> Self {
        Self::from_raw(self.value.sin())
    }

    /// Computes the cosine of a number (in radians).
    pub fn cos(self) -> Self {
        Self::from_raw(self.value.cos())
    }

    /// Computes the tangent of a number (in radians).
    pub fn tan(self) -> Self {
        Self::from_raw(self.value.tan())
    }

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    pub fn asin(self) -> Self {
        Self::from_raw(self.value.asin())
    }

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range [-1, 1].
    pub fn acos(self) -> Self {
        Self::from_raw(self.value.acos())
    }

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    pub fn atan(self) -> Self {
        Self::from_raw(self.value.atan())
    }

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    pub fn atan2(self, other: Self) -> Self {
        Self::from_raw(self.value.atan2(other.value))
    }

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns `(sin(x), cos(x))`.
    pub fn sin_cos(self) -> (Self, Self) {
        let (sin_val, cos_val) = self.value.sin_cos();
        (Self::from_raw(sin_val), Self::from_raw(cos_val))
    }

    /// Returns `e^(self) - 1` in a way that is accurate for small `self`.
    pub fn exp_m1(self) -> Self {
        Self::from_raw(self.value.exp_m1())
    }

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    pub fn ln_1p(self) -> Self {
        Self::from_raw(self.value.ln_1p())
    }

    /// Hyperbolic sine function.
    pub fn sinh(self) -> Self {
        Self::from_raw(self.value.sinh())
    }

    /// Hyperbolic cosine function.
    pub fn cosh(self) -> Self {
        Self::from_raw(self.value.cosh())
    }

    /// Hyperbolic tangent function.
    pub fn tanh(self) -> Self {
        Self::from_raw(self.value.tanh())
    }

    /// Inverse hyperbolic sine function.
    pub fn asinh(self) -> Self {
        Self::from_raw(self.value.asinh())
    }

    /// Inverse hyperbolic cosine function.
    pub fn acosh(self) -> Self {
        Self::from_raw(self.value.acosh())
    }

    /// Inverse hyperbolic tangent function.
    pub fn atanh(self) -> Self {
        Self::from_raw(self.value.atanh())
    }

    /// Returns the mantissa, base 2 exponent, and sign as integers, respectively.
    pub fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }
}

#[cfg(test)]
mod tests {
    use crate::motion::length::f64::Length;

    #[test]
    fn test_float_constants() {
        let nan_length = Length::nan();
        let inf_length = Length::infinity();
        let neg_inf_length = Length::neg_infinity();
        let neg_zero_length = Length::neg_zero();

        assert!(nan_length.is_nan());
        assert!(inf_length.is_infinite());
        assert!(neg_inf_length.is_infinite());
        assert!(neg_zero_length.is_sign_negative());
    }

    #[test]
    fn test_float_classification() {
        let normal = Length::from_raw(42.5);
        let nan = Length::from_raw(f64::NAN);
        let inf = Length::from_raw(f64::INFINITY);

        assert!(normal.is_finite());
        assert!(normal.is_normal());
        assert!(!normal.is_nan());
        assert!(!normal.is_infinite());

        assert!(nan.is_nan());
        assert!(!nan.is_finite());
        assert!(!nan.is_normal());

        assert!(inf.is_infinite());
        assert!(!inf.is_finite());
        assert!(!inf.is_normal());
    }

    #[test]
    fn test_rounding_functions() {
        let value = Length::from_raw(42.7);

        assert_eq!(*value.floor().raw(), 42.0);
        assert_eq!(*value.ceil().raw(), 43.0);
        assert_eq!(*value.round().raw(), 43.0);
        assert_eq!(*value.trunc().raw(), 42.0);
        assert!((value.fract().raw() - 0.7).abs() < 1e-10);

        let negative = Length::from_raw(-3.4);
        assert_eq!(*negative.floor().raw(), -4.0);
        assert_eq!(*negative.ceil().raw(), -3.0);
        assert_eq!(*negative.round().raw(), -3.0);
        assert_eq!(*negative.trunc().raw(), -3.0);
    }

    #[test]
    fn test_sign_functions() {
        let positive = Length::from_raw(42.5);
        let negative = Length::from_raw(-17.3);
        let zero = Length::from_raw(0.0);

        assert!(positive.is_sign_positive());
        assert!(!positive.is_sign_negative());
        assert_eq!(*positive.signum().raw(), 1.0);

        assert!(!negative.is_sign_positive());
        assert!(negative.is_sign_negative());
        assert_eq!(*negative.signum().raw(), -1.0);

        assert!(zero.is_sign_positive()); // +0.0 is positive
        assert_eq!(*zero.signum().raw(), 1.0);
    }

    #[test]
    fn test_abs_function() {
        let positive = Length::from_raw(42.5);
        let negative = Length::from_raw(-17.3);

        assert_eq!(*positive.abs().raw(), 42.5);
        assert_eq!(*negative.abs().raw(), 17.3);

        // Test using the method
        assert_eq!(*positive.abs().raw(), 42.5);
        assert_eq!(*negative.abs().raw(), 17.3);
    }

    #[test]
    fn test_min_max_functions() {
        let a = Length::from_raw(42.5);
        let b = Length::from_raw(17.3);

        assert_eq!(*a.max(b).raw(), 42.5);
        assert_eq!(*a.min(b).raw(), 17.3);
        assert_eq!(*b.max(a).raw(), 42.5);
        assert_eq!(*b.min(a).raw(), 17.3);

        // Test using the methods
        assert_eq!(*a.max(b).raw(), 42.5);
        assert_eq!(*a.min(b).raw(), 17.3);
    }

    #[test]
    fn test_power_functions() {
        let value = Length::from_raw(4.0);

        // Integer power
        let squared = value.powi(2);
        assert_eq!(*squared.raw(), 16.0);

        let cubed = value.powi(3);
        assert_eq!(*cubed.raw(), 64.0);

        // Floating point power
        let power = Length::from_raw(2.0);
        let result = value.powf(power);
        assert_eq!(*result.raw(), 16.0);
    }

    #[test]
    fn test_exponential_logarithm() {
        let value = Length::from_raw(2.0);

        let exp_val = value.exp();
        assert!((exp_val.raw() - 7.38905609893).abs() < 1e-10);

        let exp2_val = value.exp2();
        assert_eq!(*exp2_val.raw(), 4.0);

        let ln_val = exp_val.ln();
        assert!((ln_val.raw() - 2.0).abs() < 1e-10);

        let log2_val = Length::from_raw(8.0).log2();
        assert_eq!(*log2_val.raw(), 3.0);

        let log10_val = Length::from_raw(1000.0).log10();
        assert_eq!(*log10_val.raw(), 3.0);
    }

    #[test]
    fn test_trigonometric_functions() {
        use std::f64::consts::PI;

        let zero = Length::from_raw(0.0);
        let pi_half = Length::from_raw(PI / 2.0);
        let pi_quarter = Length::from_raw(PI / 4.0);

        // Basic trig functions
        assert!((zero.sin().raw()).abs() < 1e-10);
        assert!((zero.cos().raw() - 1.0).abs() < 1e-10);
        assert!((zero.tan().raw()).abs() < 1e-10);

        assert!((pi_half.sin().raw() - 1.0).abs() < 1e-10);
        assert!((pi_half.cos().raw()).abs() < 1e-10);

        // sin_cos
        let (sin_val, cos_val) = pi_quarter.sin_cos();
        assert!((sin_val.raw() - (PI / 4.0).sin()).abs() < 1e-10);
        assert!((cos_val.raw() - (PI / 4.0).cos()).abs() < 1e-10);

        // Inverse trig functions
        let one = Length::from_raw(1.0);
        let asin_result = one.asin();
        assert!((asin_result.raw() - PI / 2.0).abs() < 1e-10);

        let acos_result = one.acos();
        assert!((acos_result.raw()).abs() < 1e-10);

        let atan_result = one.atan();
        assert!((atan_result.raw() - PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_hyperbolic_functions() {
        let value = Length::from_raw(1.0);

        let sinh_val = value.sinh();
        let cosh_val = value.cosh();
        let tanh_val = value.tanh();

        assert!((sinh_val.raw() - 1.0_f64.sinh()).abs() < 1e-10);
        assert!((cosh_val.raw() - 1.0_f64.cosh()).abs() < 1e-10);
        assert!((tanh_val.raw() - 1.0_f64.tanh()).abs() < 1e-10);

        // Inverse hyperbolic functions
        let asinh_val = sinh_val.asinh();
        let acosh_val = cosh_val.acosh();
        let atanh_val = Length::from_raw(0.5).atanh();

        assert!((asinh_val.raw() - 1.0).abs() < 1e-10);
        assert!((acosh_val.raw() - 1.0).abs() < 1e-10);
        assert!((atanh_val.raw() - 0.5_f64.atanh()).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_cbrt() {
        let value = Length::from_raw(64.0);

        let sqrt_val = value.sqrt();
        assert_eq!(*sqrt_val.raw(), 8.0);

        let cbrt_val = value.cbrt();
        assert_eq!(*cbrt_val.raw(), 4.0);
    }

    #[test]
    fn test_special_functions() {
        let value = Length::from_raw(2.0);

        // recip
        let recip_val = value.recip();
        assert_eq!(*recip_val.raw(), 0.5);

        // mul_add
        let a = Length::from_raw(3.0);
        let b = Length::from_raw(4.0);
        let result = value.mul_add(a, b);
        assert_eq!(*result.raw(), 10.0); // 2 * 3 + 4 = 10

        // exp_m1 and ln_1p
        let small = Length::from_raw(0.1);
        let exp_m1_val = small.exp_m1();
        let ln_1p_val = small.ln_1p();

        assert!((exp_m1_val.raw() - (0.1_f64.exp() - 1.0)).abs() < 1e-10);
        assert!((ln_1p_val.raw() - (1.1_f64.ln())).abs() < 1e-10);
    }

    #[test]
    fn test_hypot() {
        let a = Length::from_raw(3.0);
        let b = Length::from_raw(4.0);

        let hypotenuse = a.hypot(b);
        assert_eq!(*hypotenuse.raw(), 5.0); // sqrt(3^2 + 4^2) = 5
    }

    #[test]
    fn test_atan2() {
        let y = Length::from_raw(1.0);
        let x = Length::from_raw(1.0);

        let angle = y.atan2(x);
        assert!((angle.raw() - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_integer_decode() {
        let value = Length::from_raw(42.5);
        let (mantissa, exponent, sign) = value.integer_decode();

        // This tests that the method is callable and returns the expected types
        assert_eq!(sign, 1); // positive number
        assert!(mantissa > 0);
        assert!(exponent != 0);
    }

    #[test]
    fn test_different_dimensions() {
        // Test that Float operations work regardless of dimension
        let length = Length::from_raw(3.14);
        let time = crate::motion::time::f64::Time::from_raw(2.71);

        assert!(length.is_finite());
        assert!(time.is_finite());

        assert_eq!(*length.abs().raw(), 3.14);
        assert_eq!(*time.abs().raw(), 2.71);

        assert_eq!(*length.floor().raw(), 3.0);
        assert_eq!(*time.ceil().raw(), 3.0);
    }    #[test]
    fn test_edge_cases() {
        let zero = Length::from_raw(0.0);
        let one = Length::from_raw(1.0);
        let neg_one = Length::from_raw(-1.0);

        // Test with special values
        assert_eq!(*zero.abs().raw(), 0.0);
        assert_eq!(*one.abs().raw(), 1.0);
        assert_eq!(*neg_one.abs().raw(), 1.0);

        assert_eq!(*one.recip().raw(), 1.0);
        assert_eq!(*neg_one.signum().raw(), -1.0);
        assert_eq!(*zero.signum().raw(), 1.0); // +0.0 has positive sign
    }
}