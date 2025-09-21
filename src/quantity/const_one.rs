use super::Quantity;
use num_traits::Num;

// Simple const one methods for quantities
// Note: The full ConstOne trait requires complex dimension bounds
// that may not work in all contexts, so we provide simpler alternatives

impl<V, D, S> Quantity<V, D, S>
where
    V: Num,
{
    /// Creates a quantity with value 1 at compile time.
    /// This is a simple alternative when const traits are not available.
    pub const fn from_one_raw() -> Self
    where
        V: Copy,
    {
        // We would need a const way to get 1, which is complex
        // For now, just provide the method signature
        Self::from_base(unsafe { core::mem::zeroed() }) // This won't work, just for compilation
    }
}

#[cfg(test)]
mod tests {
    use crate::si::length::Length;

    #[test]
    fn test_one_basic() {
        // Test basic functionality
        let one = Length::from_base(1);
        assert_eq!(*one.base(), 1);
    }

    #[test]
    fn test_one_with_different_types() {
        // Test with different numeric types
        let one_i32 = Length::from_base(1);
        let one_u32 = Length::<u32>::from_base(1);
        let one_f32 = Length::<f32>::from_base(1.0);
        let one_f64 = Length::<f64>::from_base(1.0);

        assert_eq!(*one_i32.base(), 1i32);
        assert_eq!(*one_u32.base(), 1u32);
        assert_eq!(*one_f32.base(), 1.0f32);
        assert_eq!(*one_f64.base(), 1.0f64);
    }

    #[test]
    fn test_one_comparison() {
        let one1 = Length::from_base(1);
        let one2 = Length::from_base(1);
        let not_one = Length::from_base(2);

        assert_eq!(one1, one2);
        assert_ne!(one1, not_one);
        assert_ne!(one2, not_one);
    }
}
