use super::ValueDouble;

/// # Examples
/// Value<f64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueDouble;
/// let v: ValueDouble = 1.0.into();
/// assert_eq!(v, ValueDouble(Some(1.0)));
///```
impl From<f64> for ValueDouble {
    fn from(val: f64) -> Self {
        ValueDouble(Some(val))
    }
}

/// # Examples
/// Value<f64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueDouble;
/// let v: ValueDouble = (&1.0).into();
/// assert_eq!(v, ValueDouble(Some(1.0)));
///```
impl<T> From<&T> for ValueDouble
where
    T: Into<ValueDouble> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<f64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueDouble;
/// let v: ValueDouble = Some(1.0).into();
/// assert_eq!(v, ValueDouble(Some(1.0)));
/// let v: ValueDouble = Some(&1.0).into();
/// assert_eq!(v, ValueDouble(Some(1.0)));
///```
impl<T> From<Option<T>> for ValueDouble
where
    T: Into<ValueDouble>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueDouble(None),
        }
    }
}
