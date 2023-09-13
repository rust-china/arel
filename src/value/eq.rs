use super::Value;

/// # Examples
/// Value<T>
/// ```
/// use arel::prelude::*;
/// let v1: arel::Value<i8> = 1.into();
/// let v2: arel::Value<i8> = 1.into();
/// assert!(v1 == v2);
/// assert!(v1 == 1);
///```

impl<T> PartialEq<T> for Value<T>
where
    T: Into<Value<T>> + Clone,
    Value<T>: PartialEq<Value<T>>,
{
    fn eq(&self, other: &T) -> bool {
        let other_value: Value<T> = other.into();
        *self == other_value
    }
}
/// # Examples
/// Value<T>
/// ```
/// use arel::prelude::*;
/// let v1: arel::Value<i8> = 1.into();
/// assert!(v1 == Some(1));
///```
impl<T> PartialEq<Option<T>> for Value<T>
where
    T: Into<Value<T>> + Clone,
    Value<T>: PartialEq<Value<T>>,
{
    fn eq(&self, other: &Option<T>) -> bool {
        match other {
            Some(t) => {
                let other_value: Value<T> = t.into();
                *self == other_value
            }
            None => *self == Value::Null,
        }
    }
}
