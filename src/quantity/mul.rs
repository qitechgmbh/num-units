use super::Quantity;
use core::ops::{Add, Mul};
use num_traits::Num;

// Multiplication: Multiplies values and adds dimensions (D1 + D2)
impl<V1, V2, D1, D2> Mul<Quantity<V2, D2>> for Quantity<V1, D1>
where
    V1: Num + Mul<V2>,
    V2: Num,
    <V1 as Mul<V2>>::Output: Num,
    D1: Add<D2>,
{
    type Output = Quantity<<V1 as Mul<V2>>::Output, <D1 as Add<D2>>::Output>;

    fn mul(self, rhs: Quantity<V2, D2>) -> Self::Output {
        Quantity::from_raw(self.value * rhs.value)
    }
}

// Scalar multiplication (quantity * scalar)
impl<V, D> Mul<V> for Quantity<V, D>
where
    V: Num + Mul<Output = V> + Copy,
{
    type Output = Quantity<V, D>;

    fn mul(self, scalar: V) -> Self::Output {
        Quantity::from_raw(self.value * scalar)
    }
}

#[cfg(test)]
mod tests {
    use crate::motion::length::f64::Length;
    use crate::motion::area::f64::Area;

    #[test]
    fn test_scalar_multiplication() {
        let length = Length::from_raw(5.0);

        let doubled = length * 2.0;
        assert_eq!(*doubled.raw(), 10.0);
    }

    #[test]
    fn test_dimensional_multiplication() {
        // Test that multiplication properly adds dimensions
        let length1 = Length::from_raw(5.0);
        let length2 = Length::from_raw(3.0);

        // Length × Length should give us a quantity with dimension L²M⁰T⁰
        let area = length1 * length2;
        assert_eq!(*area.raw(), 15.0);

        // The type should be Area (which is Length²)
        // We can't directly test the type, but if it compiles, the dimensions worked
    }

    #[test]
    fn test_length_multiplication_creates_area() {
        // Create two length quantities
        let width = Length::from_raw(3.0); // 3 meters
        let height = Length::from_raw(4.0); // 4 meters

        // Multiply them to get area
        let area: Area = width * height;

        // The result should be 12 square meters
        assert!((area.into_raw() - 12.0).abs() < 1e-10);

        // Test with different values
        let width_ft = Length::from_raw(3.048); // 10 ft = 3.048 m
        let height_m = Length::from_raw(2.0); // 2 m
        let mixed_area: Area = width_ft * height_m;

        // Expected: 3.048 * 2.0 = 6.096 square meters
        assert!((mixed_area.into_raw() - 6.096).abs() < 0.001);
    }
}
