use super::Quantity;
use num_traits::{Num, MulAddAssign};

// MulAddAssign implementation for quantities
// Performs fused multiply-add assignment: *self = *self * a + b
//
// Note: Floating-point MulAddAssign operations may only be available with "std" or "libm" features,
// depending on the implementation in num-traits for the specific numeric type.
impl<V, D> Quantity<V, D>
where
    V: Num + MulAddAssign + Copy,
{
    /// Fused multiply-add assignment with same-type quantities: self = self * a + b
    /// All quantities must have the same dimension
    pub fn mul_add_assign(&mut self, a: Self, b: Self)
    where
        V: MulAddAssign<V, V>,
    {
        self.value.mul_add_assign(a.value, b.value);
    }

    /// Fused multiply-add assignment with scalars: self = self * a + b
    /// Where a and b are scalar values
    pub fn mul_add_assign_scalar(&mut self, a: V, b: V)
    where
        V: MulAddAssign<V, V>,
    {
        self.value.mul_add_assign(a, b);
    }

    /// Fused multiply-add assignment: self = self * multiplier + addend
    /// Where multiplier is a scalar and addend is a quantity with same dimension
    pub fn mul_add_assign_mixed(&mut self, multiplier: V, addend: Self)
    where
        V: MulAddAssign<V, V>,
    {
        self.value.mul_add_assign(multiplier, addend.value);
    }
}

#[cfg(test)]
mod tests {
    use crate::length::f64::Length;
    use crate::length::i32::Length as LengthI32;
    use crate::scalar::f64::Scalar;
    use crate::scalar::i32::Scalar as ScalarI32;

    #[test]
    fn test_mul_add_assign_same_type() {
        let mut a = Scalar::from_raw(2.0);
        let b = Scalar::from_raw(3.0);
        let c = Scalar::from_raw(4.0);

        // a = a * b + c = 2 * 3 + 4 = 10
        a.mul_add_assign(b, c);
        assert!((a.into_raw() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_scalar() {
        let mut quantity = Length::from_raw(5.0);

        // quantity = quantity * 2.0 + 3.0 = 5 * 2 + 3 = 13
        quantity.mul_add_assign_scalar(2.0, 3.0);
        assert!((quantity.into_raw() - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_mixed() {
        let mut base = Length::from_raw(4.0);
        let addend = Length::from_raw(7.0);

        // base = base * 3.0 + addend = 4 * 3 + 7 = 19
        base.mul_add_assign_mixed(3.0, addend);
        assert!((base.into_raw() - 19.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_with_integers() {
        let mut a = ScalarI32::from_raw(6);
        let b = ScalarI32::from_raw(7);
        let c = ScalarI32::from_raw(8);

        // a = a * b + c = 6 * 7 + 8 = 50
        a.mul_add_assign(b, c);
        assert_eq!(*a.raw(), 50);
    }

    #[test]
    fn test_mul_add_assign_scalar_integers() {
        let mut quantity = LengthI32::from_raw(9);

        // quantity = quantity * 4 + 5 = 9 * 4 + 5 = 41
        quantity.mul_add_assign_scalar(4, 5);
        assert_eq!(*quantity.raw(), 41);
    }

    #[test]
    fn test_mul_add_assign_zero() {
        let mut a = Scalar::from_raw(5.0);
        let b = Scalar::from_raw(0.0);
        let c = Scalar::from_raw(3.0);

        // a = a * b + c = 5 * 0 + 3 = 3
        a.mul_add_assign(b, c);
        assert!((a.into_raw() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_negative() {
        let mut a = Scalar::from_raw(-2.0);
        let b = Scalar::from_raw(3.0);
        let c = Scalar::from_raw(1.0);

        // a = a * b + c = -2 * 3 + 1 = -5
        a.mul_add_assign(b, c);
        assert!((a.into_raw() - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_chain() {
        let mut result = Scalar::from_raw(1.0);
        let factor = Scalar::from_raw(2.0);
        let addend = Scalar::from_raw(3.0);

        // First: result = 1 * 2 + 3 = 5
        result.mul_add_assign(factor, addend);
        assert!((result.into_raw() - 5.0).abs() < 1e-10);

        // Second: result = 5 * 2 + 3 = 13
        result.mul_add_assign(factor, addend);
        assert!((result.into_raw() - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_precision() {
        // Test that MulAddAssign can potentially provide better precision
        let mut a = Scalar::from_raw(1e10);
        let b = Scalar::from_raw(1e-10);
        let c = Scalar::from_raw(1.0);

        // This should be 1 + 1 = 2
        a.mul_add_assign(b, c);
        assert!((a.into_raw() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_mul_add_assign_identity() {
        let mut quantity = Length::from_raw(42.0);
        let one = Length::from_raw(1.0);
        let zero = Length::from_raw(0.0);

        // quantity = quantity * 1 + 0 = 42
        quantity.mul_add_assign(one, zero);
        assert!((quantity.into_raw() - 42.0).abs() < 1e-10);
    }

    // Floating-point specific tests - only available with std or libm features
    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn test_mul_add_assign_floating_point_special() {
        let mut a = Scalar::from_raw(f64::sqrt(2.0));
        let b = Scalar::from_raw(f64::sqrt(2.0));
        let c = Scalar::from_raw(1.0);

        // a = sqrt(2) * sqrt(2) + 1 = 2 + 1 = 3
        a.mul_add_assign(b, c);
        assert!((a.into_raw() - 3.0).abs() < 1e-10);
    }
}