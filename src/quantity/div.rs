use super::Quantity;
use core::ops::{Div, Sub};
use num_traits::Num;

// Division: Divides values and subtracts dimensions (D1 - D2), preserve scale
impl<V1, V2, D1, D2, S> Div<Quantity<V2, D2, S>> for Quantity<V1, D1, S>
where
    V1: Num + Div<V2>,
    V2: Num,
    <V1 as Div<V2>>::Output: Num,
    D1: Sub<D2>,
{
    type Output = Quantity<<V1 as Div<V2>>::Output, <D1 as Sub<D2>>::Output, S>;

    fn div(self, rhs: Quantity<V2, D2, S>) -> Self::Output {
        Quantity::from_base(self.value / rhs.value)
    }
}

// Scalar division (quantity / scalar)
impl<V, D, S> Div<V> for Quantity<V, D, S>
where
    V: Num + Div<Output = V> + Copy,
{
    type Output = Quantity<V, D, S>;

    fn div(self, scalar: V) -> Self::Output {
        Quantity::from_base(self.value / scalar)
    }
}
