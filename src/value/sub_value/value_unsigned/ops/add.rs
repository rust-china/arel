use crate::sub_value::ValueUnsigned;
use std::ops::{Add, AddAssign};

impl<T> AddAssign<T> for ValueUnsigned
where
    T: Into<ValueUnsigned>,
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
/// use arel::value::sub_value::ValueUnsigned;
/// let v1: ValueUnsigned = 1.into();
/// assert_eq!(v1 + 2, ValueUnsigned(Some(3)));
///
/// let v1: ValueUnsigned = None::<u32>.into();
/// assert_eq!(v1 + 2, ValueUnsigned(Some(2)));
///```
impl<T> Add<T> for ValueUnsigned
where
    T: Into<ValueUnsigned>,
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self += rhs;
        self
    }
}
