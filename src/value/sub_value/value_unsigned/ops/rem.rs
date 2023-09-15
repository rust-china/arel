use crate::sub_value::ValueUnsigned;
use std::ops::{Rem, RemAssign};

impl<T> RemAssign<T> for ValueUnsigned
where
    T: Into<ValueUnsigned>,
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
/// use arel::value::sub_value::ValueUnsigned;
/// let v1: ValueUnsigned = 1.into();
/// assert_eq!(v1 % 2, ValueUnsigned(Some(1)));
///
/// let v1: ValueUnsigned = None::<u32>.into();
/// assert_eq!(v1 % 2, ValueUnsigned(Some(0)));
///```
impl<T> Rem<T> for ValueUnsigned
where
    T: Into<ValueUnsigned>,
{
    type Output = Self;
    fn rem(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self %= rhs;
        self
    }
}
