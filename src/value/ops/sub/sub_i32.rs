use crate::value::Value;
use std::ops::{Sub, SubAssign};

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let v1: arel::Value<i32> = 1.into();
/// assert_eq!(v1 - 2, arel::Value::Int(-1));
///```
impl Sub<i32> for Value<i32> {
    type Output = Self;
    fn sub(mut self, rhs: i32) -> Self::Output {
        match &mut self {
            Value::Int(v) => {
                *v -= rhs;
            }
            _ => (),
        }
        self
    }
}
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let mut v1: arel::Value<i32> = 1.into();
/// v1 -= 2;
/// assert_eq!(v1, arel::Value::Int(-1));
///```
impl SubAssign<i32> for Value<i32> {
    fn sub_assign(&mut self, rhs: i32) {
        match self {
            Value::Int(v) => {
                *v -= rhs;
            }
            _ => (),
        }
    }
}
