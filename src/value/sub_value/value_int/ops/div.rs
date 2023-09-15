use crate::sub_value::ValueInt;
use std::ops::{Div, DivAssign};

impl<T> DivAssign<T> for ValueInt
where
    T: Into<ValueInt>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v /= o,
                None => panic!("ops::div / 0 error!"),
            },
            None => match rhs.0 {
                Some(o) => self.0 = Some(0 / o),
                None => panic!("ops::div / 0 error!"),
            },
        }
    }
}

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueInt;
/// let v1: ValueInt = 1.into();
/// assert_eq!(v1 / 2, ValueInt(Some(0)));
///
/// let v1: ValueInt = None::<i32>.into();
/// assert_eq!(v1 / 2, ValueInt(Some(0)));
///```
impl<T> Div<T> for ValueInt
where
    T: Into<ValueInt>,
{
    type Output = Self;
    fn div(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self /= rhs;
        self
    }
}
