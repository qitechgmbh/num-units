use super::Quantity;
use core::ops::Add;
use num_traits::Num;

// Addition: same dimension and scale
impl<V, D, S> Add for Quantity<V, D, S>
where
    V: Num + Add<Output = V>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_base(self.value + rhs.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::length::f64::Length;

    #[test]
    fn test_same_dimension_addition() {
        let length1 = Length::from_base(5.0);
        let length2 = Length::from_base(3.0);

        let sum = length1 + length2;
        assert_eq!(*sum.base(), 8.0);
    }
}
