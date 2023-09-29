use super::ValueBigInt;

/// # Examples
/// Value<i64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBigInt;
/// let v: ValueBigInt = 1.into();
/// assert_eq!(v, ValueBigInt(Some(1)));
///```
impl From<i64> for ValueBigInt {
    fn from(val: i64) -> Self {
        ValueBigInt(Some(val))
    }
}

/// # Examples
/// Value<i64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBigInt;
/// let v: ValueBigInt = (&1).into();
/// assert_eq!(v, ValueBigInt(Some(1)));
///```
impl<T> From<&T> for ValueBigInt
where
    T: Into<ValueBigInt> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<i64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBigInt;
/// let v: ValueBigInt = Some(1).into();
/// assert_eq!(v, ValueBigInt(Some(1)));
/// let v: ValueBigInt = Some(&1).into();
/// assert_eq!(v, ValueBigInt(Some(1)));
///```
impl<T> From<Option<T>> for ValueBigInt
where
    T: Into<ValueBigInt>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueBigInt(None),
        }
    }
}

// === revert ===
impl TryFrom<ValueBigInt> for Option<i64> {
    type Error = crate::Error;
    fn try_from(value: ValueBigInt) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<ValueBigInt> for i64 {
    type Error = crate::Error;
    fn try_from(value: ValueBigInt) -> Result<Self, Self::Error> {
        match value.0 {
            Some(v) => Ok(v),
            None => Err(crate::Error::Message("Value is None!".into())),
        }
    }
}
