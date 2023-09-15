use crate::sub_value::ValueTinyUnsigned;
use std::ops::{Rem, RemAssign};

impl<T> RemAssign<T> for ValueTinyUnsigned
where
    T: Into<ValueTinyUnsigned>,
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
/// use arel::value::sub_value::ValueTinyUnsigned;
/// let v1: ValueTinyUnsigned = 1.into();
/// assert_eq!(v1 % 2, ValueTinyUnsigned(Some(1)));
///
/// let v1: ValueTinyUnsigned = None::<u8>.into();
/// assert_eq!(v1 % 2, ValueTinyUnsigned(Some(0)));
///```
impl<T> Rem<T> for ValueTinyUnsigned
where
    T: Into<ValueTinyUnsigned>,
{
    type Output = Self;
    fn rem(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self %= rhs;
        self
    }
}
