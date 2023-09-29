use super::ValueSmallInt;

/// # Examples
/// Value<i16>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallInt;
/// let v: ValueSmallInt = 1.into();
/// assert_eq!(v, ValueSmallInt(Some(1)));
///```
impl From<i16> for ValueSmallInt {
    fn from(val: i16) -> Self {
        Self(Some(val))
    }
}

/// # Examples
/// Value<i16>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallInt;
/// let v: ValueSmallInt = (&1).into();
/// assert_eq!(v, ValueSmallInt(Some(1)));
///```
impl<T> From<&T> for ValueSmallInt
where
    T: Into<ValueSmallInt> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<i16>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueSmallInt;
/// let v: ValueSmallInt = Some(1).into();
/// assert_eq!(v, ValueSmallInt(Some(1)));
/// let v: ValueSmallInt = Some(&1).into();
/// assert_eq!(v, ValueSmallInt(Some(1)));
///```
impl<T> From<Option<T>> for ValueSmallInt
where
    T: Into<ValueSmallInt>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueSmallInt(None),
        }
    }
}

// === revert ===
impl TryFrom<ValueSmallInt> for Option<i16> {
    type Error = crate::Error;
    fn try_from(value: ValueSmallInt) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<ValueSmallInt> for i16 {
    type Error = crate::Error;
    fn try_from(value: ValueSmallInt) -> Result<Self, Self::Error> {
        match value.0 {
            Some(v) => Ok(v),
            None => Err(crate::Error::Message("Value is None!".into())),
        }
    }
}
