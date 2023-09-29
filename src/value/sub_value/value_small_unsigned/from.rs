use super::ValueSmallUnsigned;

/// # Examples
/// Value<u16>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallUnsigned;
/// let v: ValueSmallUnsigned = 1.into();
/// assert_eq!(v, ValueSmallUnsigned(Some(1)));
///```
impl From<u16> for ValueSmallUnsigned {
    fn from(val: u16) -> Self {
        ValueSmallUnsigned(Some(val))
    }
}

/// # Examples
/// Value<u16>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallUnsigned;
/// let v: ValueSmallUnsigned = (&1).into();
/// assert_eq!(v, ValueSmallUnsigned(Some(1)));
///```
impl<T> From<&T> for ValueSmallUnsigned
where
    T: Into<ValueSmallUnsigned> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<u16>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallUnsigned;
/// let v: ValueSmallUnsigned = Some(1).into();
/// assert_eq!(v, ValueSmallUnsigned(Some(1)));
/// let v: ValueSmallUnsigned = Some(&1).into();
/// assert_eq!(v, ValueSmallUnsigned(Some(1)));
///```
impl<T> From<Option<T>> for ValueSmallUnsigned
where
    T: Into<ValueSmallUnsigned>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueSmallUnsigned(None),
        }
    }
}

// === revert ===
impl TryFrom<ValueSmallUnsigned> for Option<u16> {
    type Error = crate::Error;
    fn try_from(value: ValueSmallUnsigned) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<ValueSmallUnsigned> for u16 {
    type Error = crate::Error;
    fn try_from(value: ValueSmallUnsigned) -> Result<Self, Self::Error> {
        match value.0 {
            Some(v) => Ok(v),
            None => Err(crate::Error::Message("Value is None!".into())),
        }
    }
}
