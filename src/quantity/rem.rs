use super::Quantity;
use core::ops::Rem;
use num_traits::Num;

// Remainder (modulo) operation for quantities
// Implements the % operator for quantities with the same dimensions
// The remainder operation preserves the dimension of the operands

impl<V, D, S> Rem for Quantity<V, D, S>
where
    V: Num + Rem<Output = V>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_base(self.value % rhs.value)
    }
}

// Remainder with scalar values
impl<V, D, S> Rem<V> for Quantity<V, D, S>
where
    V: Num + Rem<Output = V> + Copy,
{
    type Output = Self;

    fn rem(self, rhs: V) -> Self::Output {
        Self::from_base(self.value % rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::length::Length;

    #[test]
    fn test_remainder_same_dimension() {
        let a = Length::from_base(10);
        let b = Length::from_base(3);

        let result = a % b;
        assert_eq!(*result.base(), 1); // 10 % 3 = 1
    }

    #[test]
    fn test_remainder_with_scalar() {
        let quantity = Length::from_base(17);
        let scalar = 5;

        let result = quantity % scalar;
        assert_eq!(*result.base(), 2); // 17 % 5 = 2
    }

    #[test]
    fn test_remainder_zero_result() {
        let a = Length::from_base(15);
        let b = Length::from_base(5);

        let result = a % b;
        assert_eq!(*result.base(), 0); // 15 % 5 = 0
    }

    #[test]
    fn test_remainder_with_floating_point() {
        let a = Length::from_base(10.5_f32);
        let b = Length::from_base(3.0);

        let result = a % b;
        assert!((result.base() - 1.5).abs() < 1e-10); // 10.5 % 3.0 = 1.5
    }

    #[test]
    fn test_remainder_negative_numbers() {
        let a = Length::from_base(-10);
        let b = Length::from_base(3);

        let result = a % b;
        assert_eq!(*result.base(), -1); // -10 % 3 = -1 (in Rust)

        let c = Length::from_base(10);
        let d = Length::from_base(-3);

        let result2 = c % d;
        assert_eq!(*result2.base(), 1); // 10 % -3 = 1 (in Rust)
    }

    #[test]
    fn test_remainder_larger_divisor() {
        let a = Length::from_base(5);
        let b = Length::from_base(10);

        let result = a % b;
        assert_eq!(*result.base(), 5); // 5 % 10 = 5
    }

    #[test]
    fn test_remainder_one() {
        let a = Length::from_base(17);
        let b = Length::from_base(1);

        let result = a % b;
        assert_eq!(*result.base(), 0); // Any number % 1 = 0
    }

    #[test]
    fn test_remainder_self() {
        let a = Length::from_base(42);

        let result = a % a;
        assert_eq!(*result.base(), 0); // Any number % itself = 0
    }

    #[test]
    fn test_remainder_with_different_types() {
        // Test with unsigned integers
        let a = Length::from_base(25_u32);
        let b = Length::from_base(7_u32);

        let result = a % b;
        assert_eq!(*result.base(), 4); // 25 % 7 = 4

        // Test with i32
        let c = Length::from_base(100_i32);
        let d = Length::from_base(13_i32);

        let result2 = c % d;
        assert_eq!(*result2.base(), 9); // 100 % 13 = 9
    }

    #[test]
    fn test_remainder_preserves_dimension() {
        let length1 = Length::from_base(25);
        let length2 = Length::from_base(7);

        let result = length1 % length2;

        // The result should still be a length
        // This is verified by the type system - if this compiles,
        // the dimensions are correctly preserved
        assert_eq!(*result.base(), 4);
    }

    #[test]
    fn test_remainder_floating_point_edge_cases() {
        use core::f64::INFINITY;

        let a = Length::from_base(7.0);
        let b = Length::from_base(INFINITY);

        let result = a % b;
        assert_eq!(*result.base(), 7.0); // 7.0 % ∞ = 7.0

        let c = Length::from_base(INFINITY);
        let d = Length::from_base(3.0);

        let result2 = c % d;
        assert!(result2.base().is_nan()); // ∞ % 3.0 = NaN
    }
}
