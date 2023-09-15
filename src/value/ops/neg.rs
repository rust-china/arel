use crate::value::Value;
use std::ops::Neg;

impl Neg for Value {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        match &mut self {
            Value::TinyInt(v) => *v *= -1,
            Value::SmallInt(v) => *v *= -1,
            Value::Int(v) => *v *= -1,
            Value::BigInt(v) => *v *= -1,
            Value::Float(v) => *v *= -1.0,
            Value::Double(v) => *v *= -1.0,
            _ => panic!("ops::neg type not support, {:?}", self),
        }
        self
    }
}
