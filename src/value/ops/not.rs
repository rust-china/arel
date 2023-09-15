use crate::value::Value;
use std::ops::Not;

impl Not for Value {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        match &mut self {
            Value::Bool(v) => *v = !(v.clone()),
            _ => panic!("ops::not type not support, {:?}", self),
        }
        self
    }
}
