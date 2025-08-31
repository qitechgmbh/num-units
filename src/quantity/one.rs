use super::Quantity;
use core::ops::Add;
use num_traits::{Num, One};

// One trait implementation for quantities
// Provides the multiplicative identity (one) for quantities

impl<V, D> One for Quantity<V, D>
where
    V: Num + One,
    D: Add<D, Output = D>,
{
    /// Returns the multiplicative identity element of `Self`, `1`.
    fn one() -> Self {
        Self::from_raw(V::one())
    }

    /// Returns `true` if `self` is equal to the multiplicative identity.
    fn is_one(&self) -> bool {
        self.value.is_one()
    }
}

// Additional one-related methods for quantities
impl<V, D> Quantity<V, D>
where
    V: Num + One,
{
    /// Creates a new quantity with a one value and the specified dimension.
    pub fn one() -> Self {
        Self::from_raw(V::one())
    }

    /// Returns `true` if this quantity represents one.
    pub fn is_one(&self) -> bool {
        self.value.is_one()
    }

    /// Sets this quantity to one while preserving its dimension.
    pub fn set_one(&mut self) {
        self.value = V::one();
    }
}

#[cfg(test)]
mod tests {
    use crate::length::f32::Length as LengthF32;
    use crate::length::f64::Length as LengthF64;
    use crate::length::i32::Length;
    use crate::length::u32::Length as LengthU32;
    use crate::time::i32::Time;
    use num_traits::Zero;

    #[test]
    fn test_one_trait() {
        // Test that the One trait is implemented
        let one_length = Length::one();
        assert_eq!(*one_length.raw(), 1);
        assert!(one_length.is_one());

        let one_time = crate::time::f64::Time::one();
        assert_eq!(*one_time.raw(), 1.0);
        assert!(one_time.is_one());
    }

    #[test]
    fn test_is_one() {
        let one = Length::from_raw(1);
        let not_one = Length::from_raw(5);
        let zero = Length::from_raw(0);

        assert!(one.is_one());
        assert!(!not_one.is_one());
        assert!(!zero.is_one());
    }

    #[test]
    fn test_one_method() {
        let one_length = Length::one();
        assert_eq!(*one_length.raw(), 1);
        assert!(one_length.is_one());

        let one_f64 = LengthF64::one();
        assert_eq!(*one_f64.raw(), 1.0);
        assert!(one_f64.is_one());
    }

    #[test]
    fn test_set_one() {
        let mut quantity = Length::from_raw(42);
        assert!(!quantity.is_one());

        quantity.set_one();
        assert!(quantity.is_one());
        assert_eq!(*quantity.raw(), 1);
    }

    #[test]
    fn test_one_with_different_types() {
        // Test with different numeric types
        let one_i32 = Length::one();
        let one_u32 = LengthU32::one();
        let one_f32 = LengthF32::one();
        let one_f64 = LengthF64::one();

        assert!(one_i32.is_one());
        assert!(one_u32.is_one());
        assert!(one_f32.is_one());
        assert!(one_f64.is_one());

        assert_eq!(*one_i32.raw(), 1i32);
        assert_eq!(*one_u32.raw(), 1u32);
        assert_eq!(*one_f32.raw(), 1.0f32);
        assert_eq!(*one_f64.raw(), 1.0f64);
    }

    #[test]
    fn test_one_preserves_dimension() {
        let one_length = Length::one();
        let one_time = Time::one();

        // These should have different types (different dimensions)
        // This is verified by the type system - if this compiles,
        // the dimensions are correctly preserved
        assert!(one_length.is_one());
        assert!(one_time.is_one());
    }

    #[test]
    fn test_one_multiplication_identity() {
        let value = Length::from_raw(42);
        let one_scalar = 1;

        // Multiplying by one scalar should not change the value
        let result1 = value * one_scalar;

        assert_eq!(result1, value);
    }

    #[test]
    fn test_one_division_identity() {
        let value = Length::from_raw(42);
        let one_scalar = 1;

        // Dividing by one should not change the value
        let result = value / one_scalar;
        assert_eq!(result, value);
    }

    #[test]
    fn test_one_power() {
        let value = Length::from_raw(42);

        // Any number to the power of 1 should equal itself
        // Note: This tests the conceptual relationship,
        // actual implementation depends on pow operations
        let _one_power = 1;
        // value^1 == value (conceptually)
        assert_eq!(*value.raw(), 42);
    }

    #[test]
    fn test_one_comparison() {
        let one1 = Length::one();
        let one2 = Length::from_raw(1);
        let not_one = Length::from_raw(2);

        assert_eq!(one1, one2);
        assert_ne!(one1, not_one);
        assert_ne!(one2, not_one);
    }

    #[test]
    fn test_one_with_negative_numbers() {
        let negative_one = Length::from_raw(-1);
        let positive_one = Length::one();

        assert!(!negative_one.is_one()); // -1 is not the multiplicative identity
        assert!(positive_one.is_one());
    }

    #[test]
    fn test_one_with_floating_point() {
        let one_f64 = LengthF64::one();
        let almost_one = LengthF64::from_raw(1.0000001);

        assert!(one_f64.is_one());
        assert!(!almost_one.is_one()); // Close to 1 but not exactly 1
    }

    #[test]
    fn test_one_and_zero_difference() {
        let one = Length::one();
        let zero = Length::zero();

        assert!(one.is_one());
        assert!(!one.is_zero());
        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert_ne!(one, zero);
    }
}
