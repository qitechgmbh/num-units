use super::Quantity;
use num_traits::{MulAddAssign, Num};

// MulAddAssign implementation for quantities
// Performs fused multiply-add assignment: *self = *self * a + b
//
// Note: Floating-point MulAddAssign operations may only be available with "std" or "libm" features,
// depending on the implementation in num-traits for the specific numeric type.
impl<V, D, S> Quantity<V, D, S>
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
    use crate::length::Length;
    use crate::scalar::Scalar;

    #[test]
    fn test_mul_add_assign_same_type() {
        let mut a = Scalar::from_base(2.0_f32);
        let b = Scalar::from_base(3.0);
        let c = Scalar::from_base(4.0);

        // a = a * b + c = 2 * 3 + 4 = 10
        a.mul_add_assign(b, c);
        assert!((a.into_base() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_scalar() {
        let mut quantity = Length::from_base(5.0_f32);

        // quantity = quantity * 2.0 + 3.0 = 5 * 2 + 3 = 13
        quantity.mul_add_assign_scalar(2.0, 3.0);
        assert!((quantity.into_base() - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_mixed() {
        let mut base = Length::from_base(4.0_f32);
        let addend = Length::from_base(7.0);

        // base = base * 3.0 + addend = 4 * 3 + 7 = 19
        base.mul_add_assign_mixed(3.0, addend);
        assert!((base.into_base() - 19.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_with_integers() {
        let mut a = Scalar::from_base(6);
        let b = Scalar::from_base(7);
        let c = Scalar::from_base(8);

        // a = a * b + c = 6 * 7 + 8 = 50
        a.mul_add_assign(b, c);
        assert_eq!(*a.base(), 50);
    }

    #[test]
    fn test_mul_add_assign_scalar_integers() {
        let mut quantity = Length::from_base(9);

        // quantity = quantity * 4 + 5 = 9 * 4 + 5 = 41
        quantity.mul_add_assign_scalar(4, 5);
        assert_eq!(*quantity.base(), 41);
    }

    #[test]
    fn test_mul_add_assign_zero() {
        let mut a = Scalar::from_base(5.0_f32);
        let b = Scalar::from_base(0.0);
        let c = Scalar::from_base(3.0);

        // a = a * b + c = 5 * 0 + 3 = 3
        a.mul_add_assign(b, c);
        assert!((a.into_base() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_negative() {
        let mut a = Scalar::from_base(-2.0_f32);
        let b = Scalar::from_base(3.0);
        let c = Scalar::from_base(1.0);

        // a = a * b + c = -2 * 3 + 1 = -5
        a.mul_add_assign(b, c);
        assert!((a.into_base() - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_chain() {
        let mut result = Scalar::from_base(1.0_f32);
        let factor = Scalar::from_base(2.0);
        let addend = Scalar::from_base(3.0);

        // First: result = 1 * 2 + 3 = 5
        result.mul_add_assign(factor, addend);
        assert!((result.into_base() - 5.0).abs() < 1e-10);

        // Second: result = 5 * 2 + 3 = 13
        result.mul_add_assign(factor, addend);
        assert!((result.into_base() - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_assign_precision() {
        // Test that MulAddAssign can potentially provide better precision
        let mut a = Scalar::from_base(1e10_f32);
        let b = Scalar::from_base(1e-10);
        let c = Scalar::from_base(1.0);

        // This should be 1 + 1 = 2
        a.mul_add_assign(b, c);
        assert!((a.into_base() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_mul_add_assign_identity() {
        let mut quantity = Length::from_base(42.0_f32);
        let one = Length::from_base(1.0);
        let zero = Length::from_base(0.0);

        // quantity = quantity * 1 + 0 = 42
        quantity.mul_add_assign(one, zero);
        assert!((quantity.into_base() - 42.0).abs() < 1e-10);
    }

    // Floating-point specific tests - only available with std or libm features
    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn test_mul_add_assign_floating_point_special() {
        let mut a = Scalar::from_base(2.0_f64.sqrt());
        let b = Scalar::from_base(2.0_f64.sqrt());
        let c = Scalar::from_base(1.0);

        // a = sqrt(2) * sqrt(2) + 1 = 2 + 1 = 3
        a.mul_add_assign(b, c);
        assert!((a.into_base() - 3.0).abs() < 1e-10);
    }
}
