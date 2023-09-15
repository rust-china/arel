use crate::sub_value::ValueSmallInt;
use std::ops::{Rem, RemAssign};

impl<T> RemAssign<T> for ValueSmallInt
where
    T: Into<ValueSmallInt>,
{
    fn rem_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v %= o,
                None => panic!("ops::rem % 0 error!"),
            },
            None => match rhs.0 {
                Some(o) => self.0 = Some(0 % o),
                None => panic!("ops::rem % 0 error!"),
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
/// assert_eq!(v1 % 2, ValueSmallInt(Some(1)));
///
/// let v1: ValueSmallInt = None::<i16>.into();
/// assert_eq!(v1 % 2, ValueSmallInt(Some(0)));
///```
impl<T> Rem<T> for ValueSmallInt
where
    T: Into<ValueSmallInt>,
{
    type Output = Self;
    fn rem(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self %= rhs;
        self
    }
}
