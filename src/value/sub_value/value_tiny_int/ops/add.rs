use crate::sub_value::ValueTinyInt;
use std::ops::{Add, AddAssign};

impl<T> AddAssign<T> for ValueTinyInt
where
    T: Into<ValueTinyInt>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v += o,
                None => *v += 0,
            },
            None => match rhs.0 {
                Some(o) => self.0 = Some(0 + o),
                None => (),
            },
        }
    }
}

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueTinyInt;
/// let v1: ValueTinyInt = 1.into();
/// assert_eq!(v1 + 2, ValueTinyInt(Some(3)));
///
/// let v1: ValueTinyInt = None::<i8>.into();
/// assert_eq!(v1 + 2, ValueTinyInt(Some(2)));
///```
impl<T> Add<T> for ValueTinyInt
where
    T: Into<ValueTinyInt>,
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self += rhs;
        self
    }
}
