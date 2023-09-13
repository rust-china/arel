use crate::value::Value;
use std::ops::{Mul, MulAssign};

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let v1: arel::Value<i32> = 1.into();
/// assert_eq!(v1 * 2, arel::Value::Int(2));
///```
impl Mul<i32> for Value<i32> {
    type Output = Self;
    fn mul(mut self, rhs: i32) -> Self::Output {
        match &mut self {
            Value::Int(v) => {
                *v *= rhs;
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
/// v1 *= 2;
/// assert_eq!(v1, arel::Value::Int(2));
///```
impl MulAssign<i32> for Value<i32> {
    fn mul_assign(&mut self, rhs: i32) {
        match self {
            Value::Int(v) => {
                *v *= rhs;
            }
            _ => (),
        }
    }
}
