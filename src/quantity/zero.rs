use super::Quantity;
use num_traits::{Num, Zero};

// num-traits support for Zero
impl<V, D> Zero for Quantity<V, D>
where
    V: Num + Zero,
{
    fn zero() -> Self {
        Quantity::from_raw(V::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion::length::f64::Length;

    #[test]
    fn test_zero() {
        let zero_length = Length::zero();
        assert!(zero_length.is_zero());
        assert_eq!(*zero_length.raw(), 0.0);
    }
}
