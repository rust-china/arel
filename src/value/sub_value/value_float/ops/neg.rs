use crate::sub_value::ValueFloat;
use std::ops::Neg;

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueFloat;
/// let v1: ValueFloat = 1.0.into();
/// assert_eq!(-v1, ValueFloat(Some(-1.0)));
///
/// let v1: ValueFloat = None::<f32>.into();
/// assert_eq!(-v1, ValueFloat(None));
///```
impl Neg for ValueFloat {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self.0 {
            Some(v) => *v *= -1.0,
            _ => (),
        }
        self
    }
}
