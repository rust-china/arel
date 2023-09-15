use crate::sub_value::ValueInt;
use std::ops::Neg;

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueInt;
/// let v1: ValueInt = 1.into();
/// assert_eq!(-v1, ValueInt(Some(-1)));
///
/// let v1: ValueInt = None::<i32>.into();
/// assert_eq!(-v1, ValueInt(None));
///```
impl Neg for ValueInt {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self.0 {
            Some(v) => *v *= -1,
            _ => (),
        }
        self
    }
}
