use super::Quantity;
use core::ops::{Add, Mul};
use num_traits::{MulAdd, Num};

// MulAdd implementation for quantities
// Performs fused multiply-add: self * a + b
//
// Note: Floating-point MulAdd operations may only be available with "std" or "libm" features,
// depending on the implementation in num-traits for the specific numeric type.
impl<V, D, S> Quantity<V, D, S>
where
    V: Num + MulAdd,
{
    /// Fused multiply-add with same-type quantities: self * a + b
    /// All quantities must have the same dimension
    pub fn mul_add(self, a: Self, b: Self) -> Self
    where
        V: MulAdd<V, V, Output = V>,
    {
        Quantity::from_base(self.value.mul_add(a.value, b.value))
    }

    /// Fused multiply-add with scalars: self * a + b
    /// Where a and b are scalar values
    pub fn mul_add_scalar(self, a: V, b: V) -> Self
    where
        V: MulAdd<V, V, Output = V>,
    {
        Quantity::from_base(self.value.mul_add(a, b))
    }

    /// Fused multiply-add: (self * multiplier) + addend
    /// Where multiplier is a scalar and addend is a quantity with same dimension
    pub fn mul_add_mixed(self, multiplier: V, addend: Self) -> Self
    where
        V: MulAdd<V, V, Output = V>,
    {
        Quantity::from_base(self.value.mul_add(multiplier, addend.value))
    }
}

// Dimensional MulAdd operations
impl<V, D1, S> Quantity<V, D1, S>
where
    V: Num + MulAdd + Copy,
{
    /// Dimensional fused multiply-add: self * multiplier + addend
    /// Where multiplier has dimension D2, and addend has dimension D1 + D2
    /// Result has dimension D1 + D2
    pub fn mul_add_dimensional<V2, D2, D3, S2, S3>(
        self,
        multiplier: Quantity<V2, D2, S2>,
        addend: Quantity<V, D3, S3>,
    ) -> Quantity<V, D3, S>
    where
        V: Mul<V2>,
        V2: Num,
        <V as Mul<V2>>::Output: Into<V> + Num,
        D1: Add<D2, Output = D3>,
    {
        // For dimensional operations, we use separate multiply and add
        // since MulAdd doesn't work across different types
        let product: V = (self.value * multiplier.value).into();
        Quantity::from_base(product + addend.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::length::Length;
    use crate::scalar::Scalar;

    #[test]
    fn test_mul_add_same_type() {
        let a = Scalar::from_base(2.0_f32);
        let b = Scalar::from_base(3.0);
        let c = Scalar::from_base(4.0);

        // a * b + c = 2 * 3 + 4 = 10
        let result = a.mul_add(b, c);
        assert!((result.into_base() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_scalar() {
        let quantity = Length::from_base(5.0_f32);

        // quantity * 2.0 + 3.0 = 5 * 2 + 3 = 13
        let result = quantity.mul_add_scalar(2.0, 3.0);
        assert!((result.into_base() - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_mixed() {
        let base = Length::from_base(4.0_f32);
        let addend = Length::from_base(7.0);

        // base * 3.0 + addend = 4 * 3 + 7 = 19
        let result = base.mul_add_mixed(3.0, addend);
        assert!((result.into_base() - 19.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_with_integers() {
        let a = Scalar::from_base(6);
        let b = Scalar::from_base(7);
        let c = Scalar::from_base(8);

        // a * b + c = 6 * 7 + 8 = 50
        let result = a.mul_add(b, c);
        assert_eq!(*result.base(), 50);
    }

    #[test]
    fn test_mul_add_scalar_integers() {
        let quantity = Length::from_base(9);

        // quantity * 4 + 5 = 9 * 4 + 5 = 41
        let result = quantity.mul_add_scalar(4, 5);
        assert_eq!(*result.base(), 41);
    }

    #[test]
    fn test_mul_add_zero() {
        let a = Scalar::from_base(5.0_f32);
        let b = Scalar::from_base(0.0);
        let c = Scalar::from_base(3.0);

        // a * b + c = 5 * 0 + 3 = 3
        let result = a.mul_add(b, c);
        assert!((result.into_base() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_negative() {
        let a = Scalar::from_base(-2.0_f32);
        let b = Scalar::from_base(3.0);
        let c = Scalar::from_base(1.0);

        // a * b + c = -2 * 3 + 1 = -5
        let result = a.mul_add(b, c);
        assert!((result.into_base() - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_precision() {
        // Test that MulAdd can potentially provide better precision
        // than separate multiply and add operations
        let a = Scalar::from_base(1e10_f32);
        let b = Scalar::from_base(1e-10);
        let c = Scalar::from_base(1.0);

        // This should be 1 + 1 = 2, but with floating point precision issues
        let result = a.mul_add(b, c);
        assert!((result.into_base() - 2.0).abs() < 1e-9);
    }

    // Floating-point specific tests - only available with std or libm features
    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn test_mul_add_floating_point_special() {
        // Test with floating point edge cases
        let a = Scalar::from_base(2.0_f32.sqrt());
        let b = Scalar::from_base(2.0_f32.sqrt());
        let c = Scalar::from_base(1.0);

        // sqrt(2) * sqrt(2) + 1 = 2 + 1 = 3
        let result = a.mul_add(b, c);
        assert!((result.into_base() - 3.0).abs() < 1e-10);
    }
}
