use crate::value::Value;
use std::ops::{Add, AddAssign};

impl<T> AddAssign<T> for Value
where
    T: Into<Value>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs: Self = rhs.into();
        match self {
            Value::TinyInt(v) => match rhs {
                Value::TinyInt(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            Value::SmallInt(v) => match rhs {
                Value::SmallInt(rhs_v) => {
                    *v += rhs_v;
                }
                _ => (),
            },
            Value::Int(v) => match rhs {
                Value::Int(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            Value::BigInt(v) => match rhs {
                Value::BigInt(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            Value::TinyUnsigned(v) => match rhs {
                Value::TinyUnsigned(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            Value::SmallUnsigned(v) => match rhs {
                Value::SmallUnsigned(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            Value::Unsigned(v) => match rhs {
                Value::Unsigned(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            #[cfg(any(feature = "mysql"))]
            Value::BigUnsigned(v) => match rhs {
                Value::BigUnsigned(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            Value::Float(v) => match rhs {
                Value::Float(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            Value::Double(v) => match rhs {
                Value::Double(rhs_v) => {
                    *v += rhs_v;
                }
                _ => panic!("ops::add type incompatible error, {:?} + {:?}", self, rhs),
            },
            _ => panic!("ops::add type not support, {:?} + {:?}", self, rhs),
        }
    }
}

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let sub_v1: arel::sub_value::ValueInt = 1.into();
/// let sub_v2: arel::sub_value::ValueInt = 2.into();
/// let v1: arel::Value = sub_v1.into();
/// let v2: arel::Value = sub_v2.into();
/// assert_eq!(v1 + v2, arel::Value::Int(3.into()));
///```

impl<T> Add<T> for Value
where
    T: Into<Value>,
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self += rhs;
        self
    }
}
