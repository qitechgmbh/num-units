use super::Quantity;
use core::ops::Sub;
use num_traits::Num;

// Subtraction: Only allowed for quantities with the same dimension
impl<V, D> Sub for Quantity<V, D>
where
    V: Num + Sub<Output = V>,
{
    type Output = Quantity<V, D>;

    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::from_raw(self.value - rhs.value)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_same_dimension_subtraction() {
        let length1 = crate::length::f64::Length::from_raw(5.0);
        let length2 = crate::length::f64::Length::from_raw(3.0);

        let diff = length1 - length2;
        assert_eq!(*diff.raw(), 2.0);
    }
}
