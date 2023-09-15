use super::ValueUnsigned;

/// # Examples
/// Value<u32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueUnsigned;
/// let v: ValueUnsigned = 1.into();
/// assert_eq!(v, ValueUnsigned(Some(1)));
///```
impl From<u32> for ValueUnsigned {
    fn from(val: u32) -> Self {
        ValueUnsigned(Some(val))
    }
}

/// # Examples
/// Value<u32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueUnsigned;
/// let v: ValueUnsigned = (&1).into();
/// assert_eq!(v, ValueUnsigned(Some(1)));
///```
impl<T> From<&T> for ValueUnsigned
where
    T: Into<ValueUnsigned> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<u32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueUnsigned;
/// let v: ValueUnsigned = Some(1).into();
/// assert_eq!(v, ValueUnsigned(Some(1)));
/// let v: ValueUnsigned = Some(&1).into();
/// assert_eq!(v, ValueUnsigned(Some(1)));
///```
impl<T> From<Option<T>> for ValueUnsigned
where
    T: Into<ValueUnsigned>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueUnsigned(None),
        }
    }
}
