use crate::{ActiveValue, Value};
use std::ops::Neg;

impl<V> Neg for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Neg<Output = V> + Default,
{
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self {
            ActiveValue::Changed(nv, ov) => self = ActiveValue::Changed(-*nv, ov.clone()),
            ActiveValue::Unchanged(v) => self = ActiveValue::Changed(-*v, Box::new(self.clone())),
            ActiveValue::NotSet => self = ActiveValue::Changed(-V::default(), Box::new(ActiveValue::NotSet)),
        };
        self
    }
}
