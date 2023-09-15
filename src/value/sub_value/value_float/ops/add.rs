use crate::sub_value::ValueFloat;
use std::ops::{Add, AddAssign};

impl<T> AddAssign<T> for ValueFloat
where
    T: Into<ValueFloat>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v += o,
                None => *v += 0.0,
            },
            None => match rhs.0 {
                Some(o) => self.0 = Some(0.0 + o),
                None => (),
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
/// assert_eq!(v1 + 2.0, ValueFloat(Some(3.0)));
///
/// let v1: ValueFloat = None::<f32>.into();
/// assert_eq!(v1 + 2.0, ValueFloat(Some(2.0)));
///```
impl<T> Add<T> for ValueFloat
where
    T: Into<ValueFloat>,
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self += rhs;
        self
    }
}
