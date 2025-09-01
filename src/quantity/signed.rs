use super::Quantity;
use num_traits::Signed;

// Signed implementations
impl<V, D, S> Quantity<V, D, S>
where
    V: Signed,
{
    pub fn abs(&self) -> Self {
        Quantity::from_base(self.value.abs())
    }

    pub fn abs_sub(self, other: &Self) -> Self {
        Quantity::from_base(self.value.abs_sub(&other.value))
    }

    pub fn signum(&self) -> Self {
        Quantity::from_base(self.value.signum())
    }

    pub fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.value.is_negative()
    }
}

#[cfg(test)]
mod tests {
    use crate::length::Length;

    #[test]
    fn test_absolute_value() {
        let negative_length = Length::from_base(-5.0);
        let abs_length = negative_length.abs();
        assert_eq!(*abs_length.base(), 5.0);
    }

    #[test]
    fn test_signed_implementations() {
        // Test with floating point
        let positive_length = Length::from_base(5.0);
        let negative_length = Length::from_base(-3.0);
        let zero_length = Length::from_base(0.0);

        // Test abs()
        assert_eq!(positive_length.abs().into_base(), 5.0);
        assert_eq!(negative_length.abs().into_base(), 3.0);
        assert_eq!(zero_length.abs().into_base(), 0.0);

        // Test signum() - note that IEEE 754 defines signum(+0.0) = +1.0
        assert_eq!(positive_length.signum().into_base(), 1.0);
        assert_eq!(negative_length.signum().into_base(), -1.0);
        assert_eq!(zero_length.signum().into_base(), 1.0); // +0.0 has signum +1.0

        // Test is_positive() and is_negative() - note that +0.0 is positive, -0.0 is negative
        assert!(positive_length.is_positive());
        assert!(!positive_length.is_negative());

        assert!(!negative_length.is_positive());
        assert!(negative_length.is_negative());

        assert!(zero_length.is_positive()); // +0.0 is positive
        assert!(!zero_length.is_negative()); // +0.0 is not negative

        // Test abs_sub()
        let length1 = Length::from_base(10.0);
        let length2 = Length::from_base(3.0);

        let abs_diff1 = length1.abs_sub(&length2);
        let abs_diff2 = length2.abs_sub(&length1);

        assert_eq!(abs_diff1.into_base(), 7.0); // max(10 - 3, 0) = 7
        assert_eq!(abs_diff2.into_base(), 0.0); // max(3 - 10, 0) = 0

        // Test with integer types
        let int_positive = Length::<i32>::from_base(42);
        let int_negative = Length::<i32>::from_base(-17);

        assert_eq!(int_positive.abs().into_base(), 42);
        assert_eq!(int_negative.abs().into_base(), 17);

        assert!(int_positive.is_positive());
        assert!(int_negative.is_negative());

        assert_eq!(int_positive.signum().into_base(), 1);
        assert_eq!(int_negative.signum().into_base(), -1);
    }
}
