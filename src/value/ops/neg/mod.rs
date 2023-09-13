use crate::value::Value;
use std::ops::Neg;
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let v1: arel::Value<i32> = 1.into();
/// assert_eq!(-v1, arel::Value::Int(-1));
///```
impl<T> Neg for Value<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self {
            Value::Int(v) => {
                *v *= -1;
            }
            _ => (),
        }
        self
    }
}
