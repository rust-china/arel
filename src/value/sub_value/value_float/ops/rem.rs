use crate::sub_value::ValueFloat;
use std::ops::{Rem, RemAssign};

impl<T> RemAssign<T> for ValueFloat
where
    T: Into<ValueFloat>,
{
    fn rem_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v %= o,
                None => panic!("ops::rem % 0 error!"),
            },
            None => match rhs.0 {
                Some(o) => self.0 = Some(0.0 % o),
                None => panic!("ops::rem % 0 error!"),
            },
        }
    }
}

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueFloat;
/// let v1: ValueFloat = 1.0.into();
/// assert_eq!(v1 % 2.0, ValueFloat(Some(1.0)));
///
/// let v1: ValueFloat = None::<f32>.into();
/// assert_eq!(v1 % 2.0, ValueFloat(Some(0.0)));
///```
impl<T> Rem<T> for ValueFloat
where
    T: Into<ValueFloat>,
{
    type Output = Self;
    fn rem(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self %= rhs;
        self
    }
}
