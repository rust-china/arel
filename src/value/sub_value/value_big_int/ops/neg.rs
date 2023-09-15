use crate::sub_value::ValueBigInt;
use std::ops::Neg;

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBigInt;
/// let v1: ValueBigInt = 1.into();
/// assert_eq!(-v1, ValueBigInt(Some(-1)));
///
/// let v1: ValueBigInt = None::<i64>.into();
/// assert_eq!(-v1, ValueBigInt(None));
///```
impl Neg for ValueBigInt {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self.0 {
            Some(v) => *v *= -1,
            _ => (),
        }
        self
    }
}
