use super::Quantity;
use core::ops::{Add, Mul};
use num_traits::Num;

// Multiplication: Multiplies values and adds dimensions (D1 + D2), preserve scale
impl<V1, V2, D1, D2, S> Mul<Quantity<V2, D2, S>> for Quantity<V1, D1, S>
where
    V1: Num + Mul<V2>,
    V2: Num,
    <V1 as Mul<V2>>::Output: Num,
    D1: Add<D2>,
{
    type Output = Quantity<<V1 as Mul<V2>>::Output, <D1 as Add<D2>>::Output, S>;

    fn mul(self, rhs: Quantity<V2, D2, S>) -> Self::Output {
        Quantity::from_base(self.value * rhs.value)
    }
}

// Scalar multiplication (quantity * scalar)
impl<V, D, S> Mul<V> for Quantity<V, D, S>
where
    V: Num + Mul<Output = V> + Copy,
{
    type Output = Quantity<V, D, S>;

    fn mul(self, scalar: V) -> Self::Output {
        Quantity::from_base(self.value * scalar)
    }
}

#[cfg(test)]
mod tests {
    use crate::area::Area;
    use crate::length::Length;

    #[test]
    fn test_scalar_multiplication() {
        let length = Length::from_base(5.0);

        let doubled = length * 2.0;
        assert_eq!(*doubled.base(), 10.0);
    }

    #[test]
    fn test_dimensional_multiplication() {
        // Test that multiplication properly adds dimensions
        let length1 = Length::from_base(5.0);
        let length2 = Length::from_base(3.0);

        // Length × Length should give us a quantity with dimension L²M⁰T⁰
        let area = length1 * length2;
        assert_eq!(*area.base(), 15.0);

        // The type should be Area (which is Length²)
        // We can't directly test the type, but if it compiles, the dimensions worked
    }

    #[test]
    fn test_length_multiplication_creates_area() {
        // Create two length quantities
        let width = Length::from_base(3.0); // 3 meters
        let height = Length::from_base(4.0); // 4 meters

        // Multiply them to get area
        let area: Area<f64> = width * height;

        // The result should be 12 square meters
        assert!((area.into_base() - 12.0_f64).abs() < 1e-10_f64);

        // Test with different values
        let width_ft = Length::from_base(3.048); // 10 ft = 3.048 m
        let height_m = Length::from_base(2.0); // 2 m
        let mixed_area: Area<f64> = width_ft * height_m;

        // Expected: 3.048 * 2.0 = 6.096 square meters
        assert!((mixed_area.into_base() - 6.096).abs() < 0.001);
    }
}
