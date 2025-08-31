use super::Quantity;
use core::ops::{Add, Mul};
use num_traits::{Num, MulAdd};

// MulAdd implementation for quantities
// Performs fused multiply-add: self * a + b
//
// Note: Floating-point MulAdd operations may only be available with "std" or "libm" features,
// depending on the implementation in num-traits for the specific numeric type.
impl<V, D> Quantity<V, D>
where
    V: Num + MulAdd + Copy,
{
    /// Fused multiply-add with same-type quantities: self * a + b
    /// All quantities must have the same dimension
    pub fn mul_add(self, a: Self, b: Self) -> Self
    where
        V: MulAdd<V, V, Output = V>,
    {
        Quantity::from_raw(self.value.mul_add(a.value, b.value))
    }

    /// Fused multiply-add with scalars: self * a + b
    /// Where a and b are scalar values
    pub fn mul_add_scalar(self, a: V, b: V) -> Self
    where
        V: MulAdd<V, V, Output = V>,
    {
        Quantity::from_raw(self.value.mul_add(a, b))
    }

    /// Fused multiply-add: (self * multiplier) + addend
    /// Where multiplier is a scalar and addend is a quantity with same dimension
    pub fn mul_add_mixed(self, multiplier: V, addend: Self) -> Self
    where
        V: MulAdd<V, V, Output = V>,
    {
        Quantity::from_raw(self.value.mul_add(multiplier, addend.value))
    }
}

// Dimensional MulAdd operations
impl<V, D1> Quantity<V, D1>
where
    V: Num + MulAdd + Copy,
{
    /// Dimensional fused multiply-add: self * multiplier + addend
    /// Where multiplier has dimension D2, and addend has dimension D1 + D2
    /// Result has dimension D1 + D2
    pub fn mul_add_dimensional<V2, D2, D3>(
        self,
        multiplier: Quantity<V2, D2>,
        addend: Quantity<V, D3>,
    ) -> Quantity<V, D3>
    where
        V: Mul<V2>,
        V2: Num,
        <V as Mul<V2>>::Output: Into<V> + Num,
        D1: Add<D2, Output = D3>,
    {
        // For dimensional operations, we use separate multiply and add
        // since MulAdd doesn't work across different types
        let product: V = (self.value * multiplier.value).into();
        Quantity::from_raw(product + addend.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::motion::length::f64::Length;
    use crate::motion::length::i32::Length as LengthI32;
    use crate::motion::scalar::f64::Scalar;
    use crate::motion::scalar::i32::Scalar as ScalarI32;

    #[test]
    fn test_mul_add_same_type() {
        let a = Scalar::from_raw(2.0);
        let b = Scalar::from_raw(3.0);
        let c = Scalar::from_raw(4.0);

        // a * b + c = 2 * 3 + 4 = 10
        let result = a.mul_add(b, c);
        assert!((result.into_raw() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_scalar() {
        let quantity = Length::from_raw(5.0);

        // quantity * 2.0 + 3.0 = 5 * 2 + 3 = 13
        let result = quantity.mul_add_scalar(2.0, 3.0);
        assert!((result.into_raw() - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_mixed() {
        let base = Length::from_raw(4.0);
        let addend = Length::from_raw(7.0);

        // base * 3.0 + addend = 4 * 3 + 7 = 19
        let result = base.mul_add_mixed(3.0, addend);
        assert!((result.into_raw() - 19.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_with_integers() {
        let a = ScalarI32::from_raw(6);
        let b = ScalarI32::from_raw(7);
        let c = ScalarI32::from_raw(8);

        // a * b + c = 6 * 7 + 8 = 50
        let result = a.mul_add(b, c);
        assert_eq!(*result.raw(), 50);
    }

    #[test]
    fn test_mul_add_scalar_integers() {
        let quantity = LengthI32::from_raw(9);

        // quantity * 4 + 5 = 9 * 4 + 5 = 41
        let result = quantity.mul_add_scalar(4, 5);
        assert_eq!(*result.raw(), 41);
    }

    #[test]
    fn test_mul_add_zero() {
        let a = Scalar::from_raw(5.0);
        let b = Scalar::from_raw(0.0);
        let c = Scalar::from_raw(3.0);

        // a * b + c = 5 * 0 + 3 = 3
        let result = a.mul_add(b, c);
        assert!((result.into_raw() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_negative() {
        let a = Scalar::from_raw(-2.0);
        let b = Scalar::from_raw(3.0);
        let c = Scalar::from_raw(1.0);

        // a * b + c = -2 * 3 + 1 = -5
        let result = a.mul_add(b, c);
        assert!((result.into_raw() - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_mul_add_precision() {
        // Test that MulAdd can potentially provide better precision
        // than separate multiply and add operations
        let a = Scalar::from_raw(1e10);
        let b = Scalar::from_raw(1e-10);
        let c = Scalar::from_raw(1.0);

        // This should be 1 + 1 = 2, but with floating point precision issues
        let result = a.mul_add(b, c);
        assert!((result.into_raw() - 2.0).abs() < 1e-9);
    }

    // Floating-point specific tests - only available with std or libm features
    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn test_mul_add_floating_point_special() {
        // Test with floating point edge cases
        let a = Scalar::from_raw(f64::sqrt(2.0));
        let b = Scalar::from_raw(f64::sqrt(2.0));
        let c = Scalar::from_raw(1.0);

        // sqrt(2) * sqrt(2) + 1 = 2 + 1 = 3
        let result = a.mul_add(b, c);
        assert!((result.into_raw() - 3.0).abs() < 1e-10);
    }
}