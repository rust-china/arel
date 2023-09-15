use crate::sub_value::ValueSmallInt;
use std::ops::{Sub, SubAssign};

impl<T> SubAssign<T> for ValueSmallInt
where
    T: Into<ValueSmallInt>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v -= o,
                None => *v -= 0,
            },
            None => match &rhs.0 {
                Some(o) => self.0 = Some(0 - o),
                None => (),
            },
        }
    }
}
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallInt;
/// let v1: ValueSmallInt = 1.into();
/// assert_eq!(v1 - 2, ValueSmallInt(Some(-1)));
///
/// let v1: ValueSmallInt = None::<i16>.into();
/// assert_eq!(v1 - 2, ValueSmallInt(Some(-2)));
///```
impl<T> Sub<T> for ValueSmallInt
where
    T: Into<ValueSmallInt>,
{
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self -= rhs;
        self
    }
}
