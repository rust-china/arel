use crate::sub_value::ValueDouble;
use std::ops::{Div, DivAssign};

impl<T> DivAssign<T> for ValueDouble
where
    T: Into<ValueDouble>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v /= o,
                None => panic!("ops::div / 0 error!"),
            },
            None => match rhs.0 {
                Some(o) => self.0 = Some(0.0 / o),
                None => panic!("ops::div / 0 error!"),
            },
        }
    }
}

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueDouble;
/// let v1: ValueDouble = 1.0.into();
/// assert_eq!(v1 / 2.0, ValueDouble(Some(0.5)));
///
/// let v1: ValueDouble = None::<f64>.into();
/// assert_eq!(v1 / 2.0, ValueDouble(Some(0.0)));
///```
impl<T> Div<T> for ValueDouble
where
    T: Into<ValueDouble>,
{
    type Output = Self;
    fn div(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self /= rhs;
        self
    }
}
