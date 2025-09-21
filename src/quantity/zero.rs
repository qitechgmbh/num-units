use super::Quantity;
use num_traits::{Num, Zero};

// num-traits support for Zero
impl<V, D, S> Zero for Quantity<V, D, S>
where
    V: Num + Zero,
{
    fn zero() -> Self {
        Quantity::from_base(V::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::si::length::Length;

    #[test]
    fn test_zero() {
        let zero_length = Length::<f64>::zero();
        assert!(zero_length.is_zero());
        assert_eq!(*zero_length.base(), 0.0);
    }
}
