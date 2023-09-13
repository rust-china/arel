use crate::value::Value;
use std::ops::Not;
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let v1: arel::Value<bool> = true.into();
/// assert_eq!(!v1, arel::Value::Bool(false));
///```
impl<T> Not for Value<T>
where
    T: Not<Output = T>,
{
    type Output = Self;
    fn not(mut self) -> Self::Output {
        match &mut self {
            Value::Bool(v) => {
                *v = !*v;
            }
            _ => (),
        }
        self
    }
}
