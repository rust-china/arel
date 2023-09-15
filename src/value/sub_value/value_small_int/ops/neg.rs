use crate::sub_value::ValueSmallInt;
use std::ops::Neg;

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallInt;
/// let v1: ValueSmallInt = 1.into();
/// assert_eq!(-v1, ValueSmallInt(Some(-1)));
///
/// let v1: ValueSmallInt = None::<i16>.into();
/// assert_eq!(-v1, ValueSmallInt(None));
///```
impl Neg for ValueSmallInt {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self.0 {
            Some(v) => *v *= -1,
            _ => (),
        }
        self
    }
}
