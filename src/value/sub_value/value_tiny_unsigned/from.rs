use super::ValueTinyUnsigned;

/// # Examples
/// Value<u8>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueTinyUnsigned;
/// let v: ValueTinyUnsigned = 1.into();
/// assert_eq!(v, ValueTinyUnsigned(Some(1)));
///```
impl From<u8> for ValueTinyUnsigned {
    fn from(val: u8) -> Self {
        ValueTinyUnsigned(Some(val))
    }
}

/// # Examples
/// Value<u8>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueTinyUnsigned;
/// let v: ValueTinyUnsigned = (&1).into();
/// assert_eq!(v, ValueTinyUnsigned(Some(1)));
///```
impl<T> From<&T> for ValueTinyUnsigned
where
    T: Into<ValueTinyUnsigned> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<u8>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueTinyUnsigned;
/// let v: ValueTinyUnsigned = Some(1).into();
/// assert_eq!(v, ValueTinyUnsigned(Some(1)));
/// let v: ValueTinyUnsigned = Some(&1).into();
/// assert_eq!(v, ValueTinyUnsigned(Some(1)));
///```
impl<T> From<Option<T>> for ValueTinyUnsigned
where
    T: Into<ValueTinyUnsigned>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueTinyUnsigned(None),
        }
    }
}
