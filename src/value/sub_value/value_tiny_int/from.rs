use super::ValueTinyInt;

/// # Examples
/// Value<i8>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueTinyInt;
/// let v: ValueTinyInt = 1.into();
/// assert_eq!(v, ValueTinyInt(Some(1)));
///```
impl From<i8> for ValueTinyInt {
    fn from(val: i8) -> Self {
        Self(Some(val))
    }
}

/// # Examples
/// Value<i8>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueTinyInt;
/// let v: ValueTinyInt = (&1).into();
/// assert_eq!(v, ValueTinyInt(Some(1)));
///```
impl<T> From<&T> for ValueTinyInt
where
    T: Into<ValueTinyInt> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<i8>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueTinyInt;
/// let v: ValueTinyInt = Some(1).into();
/// assert_eq!(v, ValueTinyInt(Some(1)));
/// let v: ValueTinyInt = Some(&1).into();
/// assert_eq!(v, ValueTinyInt(Some(1)));
///```
impl<T> From<Option<T>> for ValueTinyInt
where
    T: Into<ValueTinyInt>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueTinyInt(None),
        }
    }
}

// === revert ===
impl TryFrom<ValueTinyInt> for Option<i8> {
    type Error = crate::Error;
    fn try_from(value: ValueTinyInt) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<ValueTinyInt> for i8 {
    type Error = crate::Error;
    fn try_from(value: ValueTinyInt) -> Result<Self, Self::Error> {
        match value.0 {
            Some(v) => Ok(v),
            None => Err(crate::Error::Message("Value is None!".into())),
        }
    }
}
