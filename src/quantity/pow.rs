use super::Quantity;
use core::ops::Mul;
use num_traits::{Num, Pow};

// Pow implementation
// Raises a quantity to the power of another quantity, potentially with different value types
//
// Note: Floating-point power operations (f32^i32, f64^f64, etc.) are only available
// when the "std" or "libm" features are enabled, as they depend on the corresponding
// implementations in num-traits.
impl<V1, V2, D1, D2, S1, S2> Pow<Quantity<V2, D2, S2>> for Quantity<V1, D1, S1>
where
    V1: Num + Pow<V2>,
    V2: Num,
    <V1 as Pow<V2>>::Output: Num,
    D1: Mul<D2>,
{
    type Output = Quantity<<V1 as Pow<V2>>::Output, <D1 as Mul<D2>>::Output, S1>;

    fn pow(self, rhs: Quantity<V2, D2, S2>) -> Self::Output {
        Quantity::from_base(self.value.pow(rhs.value))
    }
}

// Generic scalar power implementation
//
// This provides power operations like:
// - quantity^u32 (always available for integer bases)
// - quantity^i32 (for integer base types, always available)
// - f32_quantity^f32 (only with "std" or "libm" features)
// - f64_quantity^f64 (only with "std" or "libm" features)
//
// The actual availability depends on what the underlying numeric type V supports
// in the num-traits Pow implementations.
impl<V, D, S, E> Pow<E> for Quantity<V, D, S>
where
    V: Num + Pow<E>,
    <V as Pow<E>>::Output: Num,
    E: Num,
{
    type Output = Quantity<<V as Pow<E>>::Output, D, S>;

    fn pow(self, exp: E) -> Self::Output {
        Quantity::from_base(self.value.pow(exp))
    }
}

#[cfg(test)]
mod tests {
    use crate::si::length::Length;
    use num_traits::Pow; // Import the Pow trait to use the .pow() method

    #[test]
    fn test_scalar_power_with_integers() {
        // Test with integer types that support Pow with scalar exponents
        let length = Length::from_base(3);

        // Test with u32 exponent (i32 implements Pow<u32>)
        let squared: Length<_> = length.pow(2u32);
        assert_eq!(*squared.base(), 9);

        // Test with another working combination
        let cubed: Length<_> = length.pow(3u32);
        assert_eq!(*cubed.base(), 27);
    }

    // Floating-point power tests - only available with std or libm features
    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn test_float_power_operations() {
        // f32 tests
        let length_f32 = Length::from_base(2.0_f32);

        // Test f32^i32
        let squared_f32 = length_f32.pow(2i32);
        assert!((squared_f32.into_base() - 4.0).abs() < 1e-6);

        // Test f32^f32
        let sqrt_result_f32 = length_f32.pow(0.5f32);
        assert!((sqrt_result_f32.into_base() - 1.4142135).abs() < 1e-6);

        // f64 tests
        let length_f64 = Length::from_base(3.0);

        // Test f64^i32
        let cubed_f64: Length<f64> = length_f64.pow(3i32);
        assert!((cubed_f64.into_base() - 27.0).abs() < 1e-10);

        // Test f64^f64
        let sqrt_result_f64 = length_f64.pow(0.5f64);
        assert!((sqrt_result_f64.into_base() - 1.7320508075688772).abs() < 1e-10);

        // Mixed float power: f64^f32
        let length_f64_mixed = Length::from_base(4.0);
        let result_mixed: Length<f64> = length_f64_mixed.pow(1.5f32);
        assert!((result_mixed.into_base() - 8.0).abs() < 1e-10);
    }
}
