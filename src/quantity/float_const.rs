use super::Quantity;
use num_traits::{Num, FloatConst};

// FloatConst implementation for quantities
// Provides mathematical constants like π, e, ln(2), etc.
//
// Note: This implementation is only available with "std" or "libm" features
// as floating-point mathematical constants require either std or libm

impl<V, D> FloatConst for Quantity<V, D>
where
    V: Num + FloatConst,
{
    /// Returns Euler's number (e).
    fn E() -> Self {
        Self::from_raw(V::E())
    }

    /// Returns 1/π.
    fn FRAC_1_PI() -> Self {
        Self::from_raw(V::FRAC_1_PI())
    }

    /// Returns 1/√2.
    fn FRAC_1_SQRT_2() -> Self {
        Self::from_raw(V::FRAC_1_SQRT_2())
    }

    /// Returns 2/π.
    fn FRAC_2_PI() -> Self {
        Self::from_raw(V::FRAC_2_PI())
    }

    /// Returns 2/√π.
    fn FRAC_2_SQRT_PI() -> Self {
        Self::from_raw(V::FRAC_2_SQRT_PI())
    }

    /// Returns π/2.
    fn FRAC_PI_2() -> Self {
        Self::from_raw(V::FRAC_PI_2())
    }

    /// Returns π/3.
    fn FRAC_PI_3() -> Self {
        Self::from_raw(V::FRAC_PI_3())
    }

    /// Returns π/4.
    fn FRAC_PI_4() -> Self {
        Self::from_raw(V::FRAC_PI_4())
    }

    /// Returns π/6.
    fn FRAC_PI_6() -> Self {
        Self::from_raw(V::FRAC_PI_6())
    }

    /// Returns π/8.
    fn FRAC_PI_8() -> Self {
        Self::from_raw(V::FRAC_PI_8())
    }

    /// Returns the natural logarithm of 10.
    fn LN_10() -> Self {
        Self::from_raw(V::LN_10())
    }

    /// Returns the natural logarithm of 2.
    fn LN_2() -> Self {
        Self::from_raw(V::LN_2())
    }

    /// Returns the base 10 logarithm of e.
    fn LOG10_E() -> Self {
        Self::from_raw(V::LOG10_E())
    }

    /// Returns the base 2 logarithm of e.
    fn LOG2_E() -> Self {
        Self::from_raw(V::LOG2_E())
    }

    /// Returns π.
    fn PI() -> Self {
        Self::from_raw(V::PI())
    }

    /// Returns √2.
    fn SQRT_2() -> Self {
        Self::from_raw(V::SQRT_2())
    }

    /// Returns 2π.
    fn TAU() -> Self {
        Self::from_raw(V::TAU())
    }
}

// Additional convenience methods for mathematical constants
impl<V, D> Quantity<V, D>
where
    V: Num + FloatConst,
{
    /// Returns Euler's number (e).
    pub fn e() -> Self {
        Self::E()
    }

    /// Returns π (pi).
    pub fn pi() -> Self {
        Self::PI()
    }

    /// Returns τ (tau) = 2π.
    pub fn tau() -> Self {
        Self::TAU()
    }

    /// Returns π/2 (90 degrees in radians).
    pub fn half_pi() -> Self {
        Self::FRAC_PI_2()
    }

    /// Returns π/4 (45 degrees in radians).
    pub fn quarter_pi() -> Self {
        Self::FRAC_PI_4()
    }

    /// Returns √2.
    pub fn sqrt_2() -> Self {
        Self::SQRT_2()
    }

    /// Returns 1/√2.
    pub fn inv_sqrt_2() -> Self {
        Self::FRAC_1_SQRT_2()
    }

    /// Returns the natural logarithm of 2.
    pub fn ln_2() -> Self {
        Self::LN_2()
    }

    /// Returns the natural logarithm of 10.
    pub fn ln_10() -> Self {
        Self::LN_10()
    }

    /// Returns the base 2 logarithm of e.
    pub fn log2_e() -> Self {
        Self::LOG2_E()
    }

    /// Returns the base 10 logarithm of e.
    pub fn log10_e() -> Self {
        Self::LOG10_E()
    }
}

#[cfg(test)]
mod tests {
    use crate::motion::length::f64::Length;
    use crate::motion::length::f32::Length as LengthF32;
    use num_traits::FloatConst;

    #[test]
    fn test_float_const_trait() {
        // Test that the FloatConst trait is implemented
        let pi = Length::PI();
        let e = Length::E();
        let tau = Length::TAU();

        assert!((pi.raw() - std::f64::consts::PI).abs() < 1e-10);
        assert!((e.raw() - std::f64::consts::E).abs() < 1e-10);
        assert!((tau.raw() - std::f64::consts::TAU).abs() < 1e-10);
    }

    #[test]
    fn test_pi_constants() {
        let pi = Length::PI();
        let half_pi = Length::FRAC_PI_2();
        let quarter_pi = Length::FRAC_PI_4();
        let pi_3 = Length::FRAC_PI_3();
        let pi_6 = Length::FRAC_PI_6();
        let pi_8 = Length::FRAC_PI_8();

        assert!((pi.raw() - std::f64::consts::PI).abs() < 1e-10);
        assert!((half_pi.raw() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
        assert!((quarter_pi.raw() - std::f64::consts::FRAC_PI_4).abs() < 1e-10);
        assert!((pi_3.raw() - std::f64::consts::FRAC_PI_3).abs() < 1e-10);
        assert!((pi_6.raw() - std::f64::consts::FRAC_PI_6).abs() < 1e-10);
        assert!((pi_8.raw() - std::f64::consts::FRAC_PI_8).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_pi_constants() {
        let inv_pi = Length::FRAC_1_PI();
        let two_pi = Length::FRAC_2_PI();
        let two_sqrt_pi = Length::FRAC_2_SQRT_PI();

        assert!((inv_pi.raw() - std::f64::consts::FRAC_1_PI).abs() < 1e-10);
        assert!((two_pi.raw() - std::f64::consts::FRAC_2_PI).abs() < 1e-10);
        assert!((two_sqrt_pi.raw() - std::f64::consts::FRAC_2_SQRT_PI).abs() < 1e-10);
    }

    #[test]
    fn test_e_constants() {
        let e = Length::E();
        let log2_e = Length::LOG2_E();
        let log10_e = Length::LOG10_E();

        assert!((e.raw() - std::f64::consts::E).abs() < 1e-10);
        assert!((log2_e.raw() - std::f64::consts::LOG2_E).abs() < 1e-10);
        assert!((log10_e.raw() - std::f64::consts::LOG10_E).abs() < 1e-10);
    }

    #[test]
    fn test_logarithm_constants() {
        let ln_2 = Length::LN_2();
        let ln_10 = Length::LN_10();

        assert!((ln_2.raw() - std::f64::consts::LN_2).abs() < 1e-10);
        assert!((ln_10.raw() - std::f64::consts::LN_10).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_constants() {
        let sqrt_2 = Length::SQRT_2();
        let inv_sqrt_2 = Length::FRAC_1_SQRT_2();

        assert!((sqrt_2.raw() - std::f64::consts::SQRT_2).abs() < 1e-10);
        assert!((inv_sqrt_2.raw() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-10);
    }

    #[test]
    fn test_convenience_methods() {
        let pi = Length::pi();
        let e = Length::e();
        let tau = Length::tau();
        let half_pi = Length::half_pi();
        let quarter_pi = Length::quarter_pi();
        let sqrt_2 = Length::sqrt_2();
        let inv_sqrt_2 = Length::inv_sqrt_2();
        let ln_2 = Length::ln_2();
        let ln_10 = Length::ln_10();
        let log2_e = Length::log2_e();
        let log10_e = Length::log10_e();

        assert!((pi.raw() - std::f64::consts::PI).abs() < 1e-10);
        assert!((e.raw() - std::f64::consts::E).abs() < 1e-10);
        assert!((tau.raw() - std::f64::consts::TAU).abs() < 1e-10);
        assert!((half_pi.raw() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
        assert!((quarter_pi.raw() - std::f64::consts::FRAC_PI_4).abs() < 1e-10);
        assert!((sqrt_2.raw() - std::f64::consts::SQRT_2).abs() < 1e-10);
        assert!((inv_sqrt_2.raw() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-10);
        assert!((ln_2.raw() - std::f64::consts::LN_2).abs() < 1e-10);
        assert!((ln_10.raw() - std::f64::consts::LN_10).abs() < 1e-10);
        assert!((log2_e.raw() - std::f64::consts::LOG2_E).abs() < 1e-10);
        assert!((log10_e.raw() - std::f64::consts::LOG10_E).abs() < 1e-10);
    }

    #[test]
    fn test_dimensional_consistency() {
        // Test that constants maintain their dimensional type
        let pi_length = Length::pi();
        let _e_length = Length::e();

        // These should have the same dimension as their type parameter
        // The actual dimensional analysis is verified by the type system
        assert_eq!(pi_length.raw(), &std::f64::consts::PI);
    }

    #[test]
    fn test_tau_vs_2pi() {
        let tau = Length::TAU();
        let pi = Length::PI();

        // τ should equal 2π
        assert!((tau.raw() - 2.0 * pi.raw()).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_relationships() {
        let sqrt_2 = Length::SQRT_2();
        let inv_sqrt_2 = Length::FRAC_1_SQRT_2();

        // √2 * (1/√2) should equal 1
        assert!((sqrt_2.raw() * inv_sqrt_2.raw() - 1.0).abs() < 1e-10);

        let pi = Length::PI();
        let inv_pi = Length::FRAC_1_PI();

        // π * (1/π) should equal 1
        assert!((pi.raw() * inv_pi.raw() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mathematical_relationships() {
        let _e = Length::E();
        let ln_2 = Length::LN_2();
        let ln_10 = Length::LN_10();
        let log2_e = Length::LOG2_E();
        let log10_e = Length::LOG10_E();

        // Verify some mathematical relationships
        // log2(e) * ln(2) should equal 1
        assert!((log2_e.raw() * ln_2.raw() - 1.0).abs() < 1e-10);

        // log10(e) * ln(10) should equal 1
        assert!((log10_e.raw() * ln_10.raw() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_different_float_types() {
        // Test with f32
        let pi_f32 = LengthF32::PI();
        let e_f32 = LengthF32::E();

        assert!((pi_f32.raw() - std::f32::consts::PI).abs() < 1e-6);
        assert!((e_f32.raw() - std::f32::consts::E).abs() < 1e-6);

        // Test with f64
        let pi_f64 = Length::PI();
        let e_f64 = Length::E();

        assert!((pi_f64.raw() - std::f64::consts::PI).abs() < 1e-10);
        assert!((e_f64.raw() - std::f64::consts::E).abs() < 1e-10);
    }
}