use crate::sub_value::ValueSmallUnsigned;
use std::ops::{Mul, MulAssign};

impl<T> MulAssign<T> for ValueSmallUnsigned
where
    T: Into<ValueSmallUnsigned>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v *= o,
                None => *v *= 0,
            },
            None => match rhs.0 {
                Some(o) => self.0 = Some(0 * o),
                None => (),
            },
        }
    }
}

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallUnsigned;
/// let v1: ValueSmallUnsigned = 1.into();
/// assert_eq!(v1 * 2, ValueSmallUnsigned(Some(2)));
///
/// let v1: ValueSmallUnsigned = None::<u16>.into();
/// assert_eq!(v1 * 2, ValueSmallUnsigned(Some(0)));
///```
impl<T> Mul<T> for ValueSmallUnsigned
where
    T: Into<ValueSmallUnsigned>,
{
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self *= rhs;
        self
    }
}
