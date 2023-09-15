use crate::sub_value::ValueBool;
use std::ops::Not;

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBool;
/// let v1: ValueBool = true.into();
/// assert_eq!(!v1, ValueBool(Some(false)));
///
/// let v1: ValueBool = None::<bool>.into();
/// assert_eq!(!v1, ValueBool(Some(true)));
///```
impl Not for ValueBool {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        match &mut self.0 {
            Some(v) => *v = !*v,
            _ => self.0 = Some(true),
        }
        self
    }
}
