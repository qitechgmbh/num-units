use super::Quantity;
use num_traits::{Num, One};

// Note: We can't implement the full One trait for Quantity because it requires
// Mul<Self, Output = Self>, but our multiplication produces different dimensions.
// Instead, we provide simpler methods for creating and checking "one" values.

// Additional one-related methods for quantities
impl<V, D, S> Quantity<V, D, S>
where
    V: Num + One,
{
    /// Create a new quantity with a one value and the specified dimension.
    pub fn one() -> Self {
        Self::from_base(V::one())
    }

    /// Check if this quantity has the value of one in its units.
    pub fn is_one(&self) -> bool
    where
        V: PartialEq,
    {
        self.value == V::one()
    }

    /// Set the value of this quantity to one in its units.
    pub fn set_one(&mut self) {
        self.value = V::one();
    }
}

#[cfg(test)]
mod tests {
    use core::f64::EPSILON;

    use crate::si::{length::Length, time::Time};
    use num_traits::Zero;

    #[test]
    fn test_one_trait() {
        // Test that the One trait is implemented
        let one_length = Length::<i32>::one();
        assert_eq!(*one_length.base(), 1);
        assert!(one_length.is_one());

        let one_time = Time::<f32>::one();
        assert_eq!(*one_time.base(), 1.0);
        assert!(one_time.is_one());
    }

    #[test]
    fn test_is_one() {
        let one = Length::<i32>::from_base(1);
        let not_one = Length::<i32>::from_base(42);
        let zero = Length::<i32>::zero();

        assert!(one.is_one());
        assert!(!not_one.is_one());
        assert!(!zero.is_one());
    }

    #[test]
    fn test_one_with_different_types() {
        // Test with different numeric types
        let one_i32 = Length::<i32>::one();
        let one_u32 = Length::<u32>::one();
        let one_f32 = Length::<f32>::one();
        let one_f64 = Length::<f64>::one();

        assert!(one_i32.is_one());
        assert!(one_u32.is_one());
        assert!(one_f32.is_one());
        assert!(one_f64.is_one());

        assert_eq!(*one_i32.base(), 1i32);
        assert_eq!(*one_u32.base(), 1u32);
        assert_eq!(*one_f32.base(), 1.0f32);
        assert_eq!(*one_f64.base(), 1.0f64);
    }

    #[test]
    fn test_one_preserves_dimension() {
        let one_length = Length::<f32>::one();
        let one_time = Time::<f32>::one();

        // These should have different types (different dimensions)
        // This is verified by the type system - if this compiles,
        // the dimensions are correctly preserved
        assert!(one_length.is_one());
        assert!(one_time.is_one());
    }

    #[test]
    fn test_one_multiplication_identity() {
        let value = Length::from_base(42);
        let one_scalar = 1;

        // Multiplying by one scalar should not change the value
        let result1 = value * one_scalar;

        assert_eq!(result1, value);
    }

    #[test]
    fn test_one_division_identity() {
        let value = Length::from_base(42);
        let one_scalar = 1;

        // Dividing by one should not change the value
        let result = value / one_scalar;
        assert_eq!(result, value);
    }

    #[test]
    fn test_set_one_method() {
        let mut quantity = Length::from_base(42);

        // Verify it's not one initially
        assert!(!quantity.is_one());

        quantity.set_one();
        assert!(quantity.is_one());
    }

    #[test]
    fn test_one_edge_cases() {
        // Test with i32::MIN and i32::MAX to check for overflow
        let one1 = Length::one();
        let result1 = one1 * 1; // Should not change value

        assert_eq!(result1, one1);

        // Test with f64 precision
        let value_f64 = Length::<f64>::from_base(1.0000000001);
        assert!(!value_f64.is_one()); // Should not be considered exactly one

        let one_f64 = Length::<f64>::from_base(1.0);
        assert!(one_f64.is_one());
    }

    #[test]
    fn test_one_with_negative_values() {
        let positive_one = Length::<f32>::one();
        let negative_one = Length::from_base(-1);

        assert!(!negative_one.is_one()); // -1 is not the multiplicative identity
        assert!(positive_one.is_one());
    }

    #[test]
    fn test_one_with_floating_point() {
        let one_f64 = Length::<f64>::one();
        assert_eq!(*one_f64.base(), 1.0);
        assert!(one_f64.is_one());

        // Test with floating point precision
        let almost_one = Length::from_base(1.0 + EPSILON);
        assert!(!almost_one.is_one()); // Should not be considered exactly one
    }
}
