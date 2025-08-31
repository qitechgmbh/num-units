use super::Quantity;
use core::ops::Rem;
use num_traits::Num;

// Remainder (modulo) operation for quantities
// Implements the % operator for quantities with the same dimensions
// The remainder operation preserves the dimension of the operands

impl<V, D> Rem for Quantity<V, D>
where
    V: Num + Rem<Output = V>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.value % rhs.value)
    }
}

// Remainder with scalar values
impl<V, D> Rem<V> for Quantity<V, D>
where
    V: Num + Rem<Output = V> + Copy,
{
    type Output = Self;

    fn rem(self, rhs: V) -> Self::Output {
        Self::from_raw(self.value % rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::length::i32::Length;
    use crate::length::u32::Length as LengthU32;
    use crate::length::i64::Length as LengthI64;
    use crate::length::f64::Length as LengthF64;

    #[test]
    fn test_remainder_same_dimension() {
        let a = Length::from_raw(10);
        let b = Length::from_raw(3);

        let result = a % b;
        assert_eq!(*result.raw(), 1); // 10 % 3 = 1
    }

    #[test]
    fn test_remainder_with_scalar() {
        let quantity = Length::from_raw(17);
        let scalar = 5;

        let result = quantity % scalar;
        assert_eq!(*result.raw(), 2); // 17 % 5 = 2
    }

    #[test]
    fn test_remainder_zero_result() {
        let a = Length::from_raw(15);
        let b = Length::from_raw(5);

        let result = a % b;
        assert_eq!(*result.raw(), 0); // 15 % 5 = 0
    }

    #[test]
    fn test_remainder_with_floating_point() {
        let a = LengthF64::from_raw(10.5);
        let b = LengthF64::from_raw(3.0);

        let result = a % b;
        assert!((result.raw() - 1.5).abs() < 1e-10); // 10.5 % 3.0 = 1.5
    }

    #[test]
    fn test_remainder_negative_numbers() {
        let a = Length::from_raw(-10);
        let b = Length::from_raw(3);

        let result = a % b;
        assert_eq!(*result.raw(), -1); // -10 % 3 = -1 (in Rust)

        let c = Length::from_raw(10);
        let d = Length::from_raw(-3);

        let result2 = c % d;
        assert_eq!(*result2.raw(), 1); // 10 % -3 = 1 (in Rust)
    }

    #[test]
    fn test_remainder_larger_divisor() {
        let a = Length::from_raw(5);
        let b = Length::from_raw(10);

        let result = a % b;
        assert_eq!(*result.raw(), 5); // 5 % 10 = 5
    }

    #[test]
    fn test_remainder_one() {
        let a = Length::from_raw(17);
        let b = Length::from_raw(1);

        let result = a % b;
        assert_eq!(*result.raw(), 0); // Any number % 1 = 0
    }

    #[test]
    fn test_remainder_self() {
        let a = Length::from_raw(42);

        let result = a % a;
        assert_eq!(*result.raw(), 0); // Any number % itself = 0
    }

    #[test]
    fn test_remainder_with_different_types() {
        // Test with unsigned integers
        let a = LengthU32::from_raw(25);
        let b = LengthU32::from_raw(7);

        let result = a % b;
        assert_eq!(*result.raw(), 4); // 25 % 7 = 4

        // Test with i64
        let c = LengthI64::from_raw(100);
        let d = LengthI64::from_raw(13);

        let result2 = c % d;
        assert_eq!(*result2.raw(), 9); // 100 % 13 = 9
    }

    #[test]
    fn test_remainder_preserves_dimension() {
        let length1 = Length::from_raw(25);
        let length2 = Length::from_raw(7);

        let result = length1 % length2;

        // The result should still be a length
        // This is verified by the type system - if this compiles,
        // the dimensions are correctly preserved
        assert_eq!(*result.raw(), 4);
    }

    #[test]
    fn test_remainder_floating_point_edge_cases() {
        use core::f64;

        let a = LengthF64::from_raw(7.0);
        let b = LengthF64::from_raw(f64::INFINITY);

        let result = a % b;
        assert_eq!(*result.raw(), 7.0); // 7.0 % ∞ = 7.0

        let c = LengthF64::from_raw(f64::INFINITY);
        let d = LengthF64::from_raw(3.0);

        let result2 = c % d;
        assert!(result2.raw().is_nan()); // ∞ % 3.0 = NaN
    }
}