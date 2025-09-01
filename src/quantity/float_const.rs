use super::Quantity;
use num_traits::{FloatConst, Num};

// FloatConst implementation for quantities
// Provides mathematical constants like π, e, ln(2), etc.
//
// Note: This implementation is only available with "std" or "libm" features
// as floating-point mathematical constants require either std or libm

impl<V, D, S> FloatConst for Quantity<V, D, S>
where
    V: FloatConst + Num,
{
    /// Returns Euler's number (e).
    fn E() -> Self {
        Self::from_base(V::E())
    }

    /// Returns 1/π.
    fn FRAC_1_PI() -> Self {
        Self::from_base(V::FRAC_1_PI())
    }

    /// Returns 1/√2.
    fn FRAC_1_SQRT_2() -> Self {
        Self::from_base(V::FRAC_1_SQRT_2())
    }

    /// Returns 2/π.
    fn FRAC_2_PI() -> Self {
        Self::from_base(V::FRAC_2_PI())
    }

    /// Returns 2/√π.
    fn FRAC_2_SQRT_PI() -> Self {
        Self::from_base(V::FRAC_2_SQRT_PI())
    }

    /// Returns π/2.
    fn FRAC_PI_2() -> Self {
        Self::from_base(V::FRAC_PI_2())
    }

    /// Returns π/3.
    fn FRAC_PI_3() -> Self {
        Self::from_base(V::FRAC_PI_3())
    }

    /// Returns π/4.
    fn FRAC_PI_4() -> Self {
        Self::from_base(V::FRAC_PI_4())
    }

    /// Returns π/6.
    fn FRAC_PI_6() -> Self {
        Self::from_base(V::FRAC_PI_6())
    }

    /// Returns π/8.
    fn FRAC_PI_8() -> Self {
        Self::from_base(V::FRAC_PI_8())
    }

    /// Returns the natural logarithm of 10.
    fn LN_10() -> Self {
        Self::from_base(V::LN_10())
    }

    /// Returns the natural logarithm of 2.
    fn LN_2() -> Self {
        Self::from_base(V::LN_2())
    }

    /// Returns the base 10 logarithm of e.
    fn LOG10_E() -> Self {
        Self::from_base(V::LOG10_E())
    }

    /// Returns the base 2 logarithm of e.
    fn LOG2_E() -> Self {
        Self::from_base(V::LOG2_E())
    }

    /// Returns π.
    fn PI() -> Self {
        Self::from_base(V::PI())
    }

    /// Returns √2.
    fn SQRT_2() -> Self {
        Self::from_base(V::SQRT_2())
    }

    /// Returns 2π.
    fn TAU() -> Self {
        Self::from_base(V::TAU())
    }
}

impl<V, D, S> Quantity<V, D, S>
where
    V: FloatConst + Num,
{
    /// Returns Euler's number (e).
    pub fn e(&self) -> Self {
        Self::from_base(V::E())
    }

    /// Returns 1/π.
    pub fn frac_1_pi(&self) -> Self {
        Self::from_base(V::FRAC_1_PI())
    }

    /// Returns 1/√2.
    pub fn frac_1_sqrt_2(&self) -> Self {
        Self::from_base(V::FRAC_1_SQRT_2())
    }

    /// Returns 2/π.
    pub fn frac_2_pi(&self) -> Self {
        Self::from_base(V::FRAC_2_PI())
    }

    /// Returns 2/√π.
    pub fn frac_2_sqrt_pi(&self) -> Self {
        Self::from_base(V::FRAC_2_SQRT_PI())
    }

    /// Returns π/2.
    pub fn frac_pi_2(&self) -> Self {
        Self::from_base(V::FRAC_PI_2())
    }

    /// Returns π/3.
    pub fn frac_pi_3(&self) -> Self {
        Self::from_base(V::FRAC_PI_3())
    }

    /// Returns π/4.
    pub fn frac_pi_4(&self) -> Self {
        Self::from_base(V::FRAC_PI_4())
    }

    /// Returns π/6.
    pub fn frac_pi_6(&self) -> Self {
        Self::from_base(V::FRAC_PI_6())
    }

    /// Returns π/8.
    pub fn frac_pi_8(&self) -> Self {
        Self::from_base(V::FRAC_PI_8())
    }

    /// Returns the natural logarithm of 10.
    pub fn ln_10(&self) -> Self {
        Self::from_base(V::LN_10())
    }

    /// Returns the natural logarithm of 2.
    pub fn ln_2(&self) -> Self {
        Self::from_base(V::LN_2())
    }

    /// Returns the base 10 logarithm of e.
    pub fn log10_e(&self) -> Self {
        Self::from_base(V::LOG10_E())
    }

    /// Returns the base 2 logarithm of e.
    pub fn log2_e(&self) -> Self {
        Self::from_base(V::LOG2_E())
    }

    /// Returns π.
    pub fn pi(&self) -> Self {
        Self::from_base(V::PI())
    }

    /// Returns √2.
    pub fn sqrt_2(&self) -> Self {
        Self::from_base(V::SQRT_2())
    }

    /// Returns 2π.
    pub fn tau(&self) -> Self {
        Self::from_base(V::TAU())
    }
}

#[cfg(test)]
mod tests {
    use crate::length::Length;
    use num_traits::FloatConst;

    #[test]
    fn test_float_const_trait() {
        // Test that the FloatConst trait is implemented
        let pi = Length::PI();
        let e = Length::E();
        let tau = Length::TAU();

        assert!((pi.base() - std::f32::consts::PI).abs() < 1e-10);
        assert!((e.base() - std::f32::consts::E).abs() < 1e-10);
        assert!((tau.base() - std::f32::consts::TAU).abs() < 1e-10);
    }

    #[test]
    fn test_pi_constants() {
        let pi = Length::PI();
        let half_pi = Length::FRAC_PI_2();
        let quarter_pi = Length::FRAC_PI_4();
        let pi_3 = Length::FRAC_PI_3();
        let pi_6 = Length::FRAC_PI_6();
        let pi_8 = Length::FRAC_PI_8();

        assert!((pi.base() - std::f32::consts::PI).abs() < 1e-10);
        assert!((half_pi.base() - std::f32::consts::FRAC_PI_2).abs() < 1e-10);
        assert!((quarter_pi.base() - std::f32::consts::FRAC_PI_4).abs() < 1e-10);
        assert!((pi_3.base() - std::f32::consts::FRAC_PI_3).abs() < 1e-10);
        assert!((pi_6.base() - std::f32::consts::FRAC_PI_6).abs() < 1e-10);
        assert!((pi_8.base() - std::f32::consts::FRAC_PI_8).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_pi_constants() {
        let inv_pi = Length::FRAC_1_PI();
        let two_pi = Length::FRAC_2_PI();
        let two_sqrt_pi = Length::FRAC_2_SQRT_PI();

        assert!((inv_pi.base() - std::f32::consts::FRAC_1_PI).abs() < 1e-10);
        assert!((two_pi.base() - std::f32::consts::FRAC_2_PI).abs() < 1e-10);
        assert!((two_sqrt_pi.base() - std::f32::consts::FRAC_2_SQRT_PI).abs() < 1e-10);
    }

    #[test]
    fn test_e_constants() {
        let e = Length::E();
        let log2_e = Length::LOG2_E();
        let log10_e = Length::LOG10_E();

        assert!((e.base() - std::f32::consts::E).abs() < 1e-10);
        assert!((log2_e.base() - std::f32::consts::LOG2_E).abs() < 1e-10);
        assert!((log10_e.base() - std::f32::consts::LOG10_E).abs() < 1e-10);
    }

    #[test]
    fn test_logarithm_constants() {
        let ln_2 = Length::LN_2();
        let ln_10 = Length::LN_10();

        assert!((ln_2.base() - std::f32::consts::LN_2).abs() < 1e-10);
        assert!((ln_10.base() - std::f32::consts::LN_10).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_constants() {
        let sqrt_2 = Length::SQRT_2();
        let inv_sqrt_2 = Length::FRAC_1_SQRT_2();

        assert!((sqrt_2.base() - std::f32::consts::SQRT_2).abs() < 1e-10);
        assert!((inv_sqrt_2.base() - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-10);
    }

    #[test]
    fn test_instacne_consts() {
        let x = Length::from_base(1.0);
        assert_eq!(*x.e().base(), f64::E());
        assert_eq!(*x.frac_1_pi().base(), 1.0 / std::f64::consts::PI);
        assert_eq!(*x.frac_2_pi().base(), 2.0 / std::f64::consts::PI);
        assert_eq!(*x.frac_pi_2().base(), std::f64::consts::FRAC_PI_2);
        assert_eq!(*x.frac_pi_3().base(), std::f64::consts::FRAC_PI_3);
        assert_eq!(*x.frac_pi_4().base(), std::f64::consts::FRAC_PI_4);
        assert_eq!(*x.frac_pi_6().base(), std::f64::consts::FRAC_PI_6);
        assert_eq!(*x.frac_pi_8().base(), std::f64::consts::FRAC_PI_8);
        assert_eq!(*x.ln_10().base(), std::f64::consts::LN_10);
        assert_eq!(*x.ln_2().base(), std::f64::consts::LN_2);
        assert_eq!(*x.log10_e().base(), std::f64::consts::LOG10_E);
    }
}
