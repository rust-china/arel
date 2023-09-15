use crate::sub_value::ValueTinyInt;
use std::ops::Neg;

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueTinyInt;
/// let v1: ValueTinyInt = 1.into();
/// assert_eq!(-v1, ValueTinyInt(Some(-1)));
///
/// let v1: ValueTinyInt = None::<i8>.into();
/// assert_eq!(-v1, ValueTinyInt(None));
///```
impl Neg for ValueTinyInt {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self.0 {
            Some(v) => *v *= -1,
            _ => (),
        }
        self
    }
}
