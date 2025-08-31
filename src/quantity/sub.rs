use super::Quantity;
use core::ops::Sub;
use num_traits::Num;

// Subtraction: same dimension and scale
impl<V, D, S> Sub for Quantity<V, D, S>
where
    V: Num + Sub<Output = V>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_base(self.value - rhs.value)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_same_dimension_subtraction() {
        let length1 = crate::length::f64::Length::from_base(5.0);
        let length2 = crate::length::f64::Length::from_base(3.0);

        let diff = length1 - length2;
        assert_eq!(*diff.base(), 2.0);
    }
}
