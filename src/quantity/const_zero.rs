use super::Quantity;
use num_traits::{ConstZero, Num};

// ConstZero trait implementation for quantities
// Provides a const zero value for quantities at compile time

impl<V, D, S> ConstZero for Quantity<V, D, S>
where
    V: Num + ConstZero,
{
    /// The additive identity element.
    const ZERO: Self = Self::from_base(V::ZERO);
}

// Helper methods for const zero operations
impl<V, D, S> Quantity<V, D, S>
where
    V: Num + ConstZero,
{
    /// Creates a const zero quantity at compile time.
    pub const fn const_zero() -> Self {
        Self::from_base(V::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use crate::si::length::Length;
    use crate::si::time::Time;
    use num_traits::{ConstZero, Zero};

    #[test]
    fn test_const_zero_trait() {
        // Test that the ConstZero trait is implemented
        const ZERO_LENGTH: Length<i32> = Length::ZERO;
        assert_eq!(*ZERO_LENGTH.base(), 0);

        const ZERO_TIME: crate::si::time::Time<f32> = crate::si::time::Time::ZERO;
        assert_eq!(*ZERO_TIME.base(), 0.0);
    }

    #[test]
    fn test_const_zero_method() {
        const ZERO_LENGTH: Length<i32> = Length::const_zero();
        assert_eq!(*ZERO_LENGTH.base(), 0);

        const ZERO_F64: Length<f64> = Length::const_zero();
        assert_eq!(*ZERO_F64.base(), 0.0);
    }

    #[test]
    fn test_const_zero_runtime_use() {
        let zero_length: Length<i32> = Length::ZERO;
        assert_eq!(*zero_length.base(), 0);
        assert!(zero_length.is_zero());

        let zero_time: Time<f32> = Time::ZERO;
        assert_eq!(*zero_time.base(), 0.0);
        assert!(zero_time.is_zero());
    }

    #[test]
    fn test_const_zero_with_different_types() {
        // Test with different numeric types
        const ZERO_I32: Length<i32> = Length::ZERO;
        const ZERO_U32: Length<u32> = Length::ZERO;
        const ZERO_F32: Length<f32> = Length::ZERO;
        const ZERO_F64: Length<f64> = Length::ZERO;

        assert_eq!(*ZERO_I32.base(), 0i32);
        assert_eq!(*ZERO_U32.base(), 0u32);
        assert_eq!(*ZERO_F32.base(), 0.0f32);
        assert_eq!(*ZERO_F64.base(), 0.0f64);
    }

    #[test]
    fn test_const_zero_preserves_dimension() {
        const ZERO_LENGTH: Length<i32> = Length::ZERO;
        const ZERO_TIME: Time<i32> = Time::ZERO;

        // These should have different types (different dimensions)
        // This is verified by the type system - if this compiles,
        // the dimensions are correctly preserved
        assert_eq!(*ZERO_LENGTH.base(), 0);
        assert_eq!(*ZERO_TIME.base(), 0);
    }

    #[test]
    fn test_const_zero_equals_runtime_zero() {
        const CONST_ZERO: Length<i32> = Length::ZERO;
        let runtime_zero = Length::zero();

        assert_eq!(CONST_ZERO, runtime_zero);
    }

    #[test]
    fn test_const_zero_in_const_context() {
        // Test that const zero can be used in const contexts
        const fn const_function() -> Length<i32> {
            Length::ZERO
        }

        const RESULT: Length<i32> = const_function();
        assert_eq!(*RESULT.base(), 0);
    }

    #[test]
    fn test_const_zero_array() {
        // Test that const zero can be used to initialize arrays
        const ZERO_ARRAY: [Length<i32>; 3] = [Length::ZERO, Length::ZERO, Length::ZERO];

        for zero in &ZERO_ARRAY {
            assert_eq!(*zero.base(), 0);
        }
    }

    #[test]
    fn test_const_zero_struct_field() {
        struct TestStruct {
            zero_field: Length<i32>,
        }

        const TEST_STRUCT: TestStruct = TestStruct {
            zero_field: Length::ZERO,
        };

        assert_eq!(*TEST_STRUCT.zero_field.base(), 0);
    }

    #[test]
    fn test_const_zero_static() {
        static STATIC_ZERO: Length<i32> = Length::ZERO;
        assert_eq!(*STATIC_ZERO.base(), 0);
    }
}
