use super::Quantity;
use num_traits::{ConstZero, Num};

// ConstZero trait implementation for quantities
// Provides a const zero value for quantities at compile time

impl<V, D> ConstZero for Quantity<V, D>
where
    V: Num + ConstZero,
{
    /// The additive identity element.
    const ZERO: Self = Self::from_raw(V::ZERO);
}

// Helper methods for const zero operations
impl<V, D> Quantity<V, D>
where
    V: Num + ConstZero,
{
    /// Creates a const zero quantity at compile time.
    pub const fn const_zero() -> Self {
        Self::from_raw(V::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use crate::length::f32::Length as LengthF32;
    use crate::length::f64::Length as LengthF64;
    use crate::length::i32::Length;
    use crate::length::u32::Length as LengthU32;
    use crate::time::i32::Time;
    use num_traits::{ConstZero, Zero};

    #[test]
    fn test_const_zero_trait() {
        // Test that the ConstZero trait is implemented
        const ZERO_LENGTH: Length = Length::ZERO;
        assert_eq!(*ZERO_LENGTH.raw(), 0);

        const ZERO_TIME: crate::time::f64::Time = crate::time::f64::Time::ZERO;
        assert_eq!(*ZERO_TIME.raw(), 0.0);
    }

    #[test]
    fn test_const_zero_method() {
        const ZERO_LENGTH: Length = Length::const_zero();
        assert_eq!(*ZERO_LENGTH.raw(), 0);

        const ZERO_F64: LengthF64 = LengthF64::const_zero();
        assert_eq!(*ZERO_F64.raw(), 0.0);
    }

    #[test]
    fn test_const_zero_runtime_use() {
        let zero_length = Length::ZERO;
        assert_eq!(*zero_length.raw(), 0);
        assert!(zero_length.is_zero());

        let zero_time = crate::time::f64::Time::ZERO;
        assert_eq!(*zero_time.raw(), 0.0);
        assert!(zero_time.is_zero());
    }

    #[test]
    fn test_const_zero_with_different_types() {
        // Test with different numeric types
        const ZERO_I32: Length = Length::ZERO;
        const ZERO_U32: LengthU32 = LengthU32::ZERO;
        const ZERO_F32: LengthF32 = LengthF32::ZERO;
        const ZERO_F64: LengthF64 = LengthF64::ZERO;

        assert_eq!(*ZERO_I32.raw(), 0i32);
        assert_eq!(*ZERO_U32.raw(), 0u32);
        assert_eq!(*ZERO_F32.raw(), 0.0f32);
        assert_eq!(*ZERO_F64.raw(), 0.0f64);
    }

    #[test]
    fn test_const_zero_preserves_dimension() {
        const ZERO_LENGTH: Length = Length::ZERO;
        const ZERO_TIME: Time = Time::ZERO;

        // These should have different types (different dimensions)
        // This is verified by the type system - if this compiles,
        // the dimensions are correctly preserved
        assert_eq!(*ZERO_LENGTH.raw(), 0);
        assert_eq!(*ZERO_TIME.raw(), 0);
    }

    #[test]
    fn test_const_zero_equals_runtime_zero() {
        const CONST_ZERO: Length = Length::ZERO;
        let runtime_zero = Length::zero();

        assert_eq!(CONST_ZERO, runtime_zero);
    }

    #[test]
    fn test_const_zero_in_const_context() {
        // Test that const zero can be used in const contexts
        const fn const_function() -> Length {
            Length::ZERO
        }

        const RESULT: Length = const_function();
        assert_eq!(*RESULT.raw(), 0);
    }

    #[test]
    fn test_const_zero_array() {
        // Test that const zero can be used to initialize arrays
        const ZERO_ARRAY: [Length; 3] = [Length::ZERO, Length::ZERO, Length::ZERO];

        for zero in &ZERO_ARRAY {
            assert_eq!(*zero.raw(), 0);
        }
    }

    #[test]
    fn test_const_zero_struct_field() {
        struct TestStruct {
            zero_field: Length,
        }

        const TEST_STRUCT: TestStruct = TestStruct {
            zero_field: Length::ZERO,
        };

        assert_eq!(*TEST_STRUCT.zero_field.raw(), 0);
    }

    #[test]
    fn test_const_zero_static() {
        static STATIC_ZERO: Length = Length::ZERO;
        assert_eq!(*STATIC_ZERO.raw(), 0);
    }
}
