use super::Value;

/// # Examples
/// Value<T>
/// ```
/// use arel::prelude::*;
/// let v1: arel::sub_value::ValueTinyInt = 1.into();
/// let value: arel::Value = v1.clone().into();
/// assert!(value == Some(&v1));
/// assert!(value == Some(v1));
///```
impl<T> PartialEq<Option<T>> for Value
where
    T: Into<Value> + Clone,
{
    fn eq(&self, other: &Option<T>) -> bool {
        match other {
            Some(t) => {
                let other_value: Value = t.into();
                *self == other_value
            }
            None => false,
        }
    }
}
