mod mul_i32;

use crate::value::Value;
use std::ops::{Mul, MulAssign};
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let v1: arel::Value<i32> = 1.into();
/// let v2: arel::Value<i32> = 2.into();
/// assert_eq!(v1 * v2, arel::Value::Int(2));
///```
impl<T> Mul<Value<T>> for Value<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Self;
    fn mul(mut self, rhs: Value<T>) -> Self::Output {
        match &mut self {
            Value::TinyInt(v) => match rhs {
                Value::TinyInt(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::SmallInt(v) => match rhs {
                Value::SmallInt(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::Int(v) => match rhs {
                Value::Int(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::BigInt(v) => match rhs {
                Value::BigInt(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::TinyUnsigned(v) => match rhs {
                Value::TinyUnsigned(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::SmallUnsigned(v) => match rhs {
                Value::SmallUnsigned(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::Unsigned(v) => match rhs {
                Value::Unsigned(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::BigUnsigned(v) => match rhs {
                Value::BigUnsigned(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::Float(v) => match rhs {
                Value::Float(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::Double(v) => match rhs {
                Value::Double(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
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
/// let v2: arel::Value<i32> = 2.into();
/// v1 *= v2;
/// assert_eq!(v1, arel::Value::Int(2));
///```
impl<T> MulAssign<Value<T>> for Value<T>
where
    T: Mul<T, Output = T>,
{
    fn mul_assign(&mut self, rhs: Value<T>) {
        match self {
            Value::TinyInt(v) => match rhs {
                Value::TinyInt(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::SmallInt(v) => match rhs {
                Value::SmallInt(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::Int(v) => match rhs {
                Value::Int(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::BigInt(v) => match rhs {
                Value::BigInt(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::TinyUnsigned(v) => match rhs {
                Value::TinyUnsigned(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::SmallUnsigned(v) => match rhs {
                Value::SmallUnsigned(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::Unsigned(v) => match rhs {
                Value::Unsigned(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::BigUnsigned(v) => match rhs {
                Value::BigUnsigned(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::Float(v) => match rhs {
                Value::Float(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            Value::Double(v) => match rhs {
                Value::Double(rhs_v) => {
                    *v *= rhs_v;
                }
                _ => (),
            },
            _ => (),
        }
    }
}
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let v1: arel::Value<i32> = 1.into();
/// let v2: arel::Value<i32> = 2.into();
/// assert_eq!(v1 * &v2, arel::Value::Int(2));
///```
impl<T> Mul<&Value<T>> for Value<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Self;
    fn mul(mut self, rhs: &Value<T>) -> Self::Output {
        match &mut self {
            Value::TinyInt(v) => match rhs {
                Value::TinyInt(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::SmallInt(v) => match rhs {
                Value::SmallInt(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::Int(v) => match rhs {
                Value::Int(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::BigInt(v) => match rhs {
                Value::BigInt(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::TinyUnsigned(v) => match rhs {
                Value::TinyUnsigned(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::SmallUnsigned(v) => match rhs {
                Value::SmallUnsigned(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::Unsigned(v) => match rhs {
                Value::Unsigned(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::BigUnsigned(v) => match rhs {
                Value::BigUnsigned(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::Float(v) => match rhs {
                Value::Float(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::Double(v) => match rhs {
                Value::Double(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
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
/// let v2: arel::Value<i32> = 2.into();
/// v1 *= &v2;
/// assert_eq!(v1, arel::Value::Int(2));
///```
impl<T> MulAssign<&Value<T>> for Value<T>
where
    T: Mul<T, Output = T>,
{
    fn mul_assign(&mut self, rhs: &Value<T>) {
        match self {
            Value::TinyInt(v) => match rhs {
                Value::TinyInt(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::SmallInt(v) => match rhs {
                Value::SmallInt(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::Int(v) => match rhs {
                Value::Int(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::BigInt(v) => match rhs {
                Value::BigInt(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::TinyUnsigned(v) => match rhs {
                Value::TinyUnsigned(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::SmallUnsigned(v) => match rhs {
                Value::SmallUnsigned(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::Unsigned(v) => match rhs {
                Value::Unsigned(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::BigUnsigned(v) => match rhs {
                Value::BigUnsigned(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::Float(v) => match rhs {
                Value::Float(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            Value::Double(v) => match rhs {
                Value::Double(rhs_v) => {
                    *v *= *rhs_v;
                }
                _ => (),
            },
            _ => (),
        }
    }
}
