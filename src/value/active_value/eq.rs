use crate::{ActiveValue, Value};

/// # Examples
/// ActiveValue<T>
/// ```
/// use arel::prelude::*;
/// let v1: arel::sub_value::ValueTinyInt = 1.into();
/// let mut value = arel::ActiveValue::Unchanged(v1.clone());
/// assert!(value == v1);
///
/// value.set(2);
/// let v2: arel::sub_value::ValueTinyInt = 2.into();
/// assert!(value == v2);
///```
impl<V> PartialEq<V> for ActiveValue<V>
where
    V: Into<Value> + PartialEq + Clone,
{
    fn eq(&self, inner: &V) -> bool {
        match self {
            ActiveValue::Changed(nv, _) => nv == inner,
            ActiveValue::Unchanged(v) => v == inner,
            ActiveValue::NotSet => false,
        }
    }
}
