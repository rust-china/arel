use crate::sub_value::ValueDouble;
use std::ops::Neg;

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueDouble;
/// let v1: ValueDouble = 1.0.into();
/// assert_eq!(-v1, ValueDouble(Some(-1.0)));
///
/// let v1: ValueDouble = None::<f64>.into();
/// assert_eq!(-v1, ValueDouble(None));
///```
impl Neg for ValueDouble {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self.0 {
            Some(v) => *v *= -1.0,
            _ => (),
        }
        self
    }
}
