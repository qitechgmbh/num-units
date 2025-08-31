use super::Quantity;
use core::ops::Add;
use num_traits::Num;

// Addition: Only allowed for quantities with the same dimension
impl<V, D> Add for Quantity<V, D>
where
    V: Num + Add<Output = V>,
{
    type Output = Quantity<V, D>;

    fn add(self, rhs: Self) -> Self::Output {
        Quantity::from_raw(self.value + rhs.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::length::f64::Length;

    #[test]
    fn test_same_dimension_addition() {
        let length1 = Length::from_raw(5.0);
        let length2 = Length::from_raw(3.0);

        let sum = length1 + length2;
        assert_eq!(*sum.raw(), 8.0);
    }
}
