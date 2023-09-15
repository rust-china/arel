use super::ValueBool;

/// # Examples
/// Value<bool>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBool;
/// let v: ValueBool = true.into();
/// assert_eq!(v, ValueBool(Some(true)));
///```
impl From<bool> for ValueBool {
    fn from(val: bool) -> Self {
        ValueBool(Some(val))
    }
}

/// # Examples
/// Value<bool>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBool;
/// let v: ValueBool = (&true).into();
/// assert_eq!(v, ValueBool(Some(true)));
///```
impl<T> From<&T> for ValueBool
where
    T: Into<ValueBool> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<bool>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBool;
/// let v: ValueBool = Some(true).into();
/// assert_eq!(v, ValueBool(Some(true)));
/// let v: ValueBool = Some(&true).into();
/// assert_eq!(v, ValueBool(Some(true)));
/// let v: ValueBool = (&Some(true)).into();
/// assert_eq!(v, ValueBool(Some(true)));
/// let v: ValueBool = (&Some(&true)).into();
/// assert_eq!(v, ValueBool(Some(true)));
///```
impl<T> From<Option<T>> for ValueBool
where
    T: Into<ValueBool>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueBool(None),
        }
    }
}
