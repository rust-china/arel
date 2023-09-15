use crate::sub_value::ValueDouble;
use std::ops::{Sub, SubAssign};

impl<T> SubAssign<T> for ValueDouble
where
    T: Into<ValueDouble>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match &mut self.0 {
            Some(v) => match rhs.0 {
                Some(o) => *v -= o,
                None => *v -= 0.0,
            },
            None => match &rhs.0 {
                Some(o) => self.0 = Some(0.0 - o),
                None => (),
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
/// assert_eq!(v1 - 2.0, ValueDouble(Some(-1.0)));
///
/// let v1: ValueDouble = None::<f64>.into();
/// assert_eq!(v1 - 2.0, ValueDouble(Some(-2.0)));
///```
impl<T> Sub<T> for ValueDouble
where
    T: Into<ValueDouble>,
{
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self -= rhs;
        self
    }
}
