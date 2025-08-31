use super::Quantity;
use core::ops::{Div, Sub};
use num_traits::Num;

// Division: Divides values and subtracts dimensions (D1 - D2)
impl<V1, V2, D1, D2> Div<Quantity<V2, D2>> for Quantity<V1, D1>
where
    V1: Num + Div<V2>,
    V2: Num,
    <V1 as Div<V2>>::Output: Num,
    D1: Sub<D2>,
{
    type Output = Quantity<<V1 as Div<V2>>::Output, <D1 as Sub<D2>>::Output>;

    fn div(self, rhs: Quantity<V2, D2>) -> Self::Output {
        Quantity::from_raw(self.value / rhs.value)
    }
}

// Scalar division (quantity / scalar)
impl<V, D> Div<V> for Quantity<V, D>
where
    V: Num + Div<Output = V> + Copy,
{
    type Output = Quantity<V, D>;

    fn div(self, scalar: V) -> Self::Output {
        Quantity::from_raw(self.value / scalar)
    }
}
