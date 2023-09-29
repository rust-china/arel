use super::ValueInt;

/// # Examples
/// Value<i32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueInt;
/// let v: ValueInt = 1.into();
/// assert_eq!(v, ValueInt(Some(1)));
///```
impl From<i32> for ValueInt {
    fn from(val: i32) -> Self {
        ValueInt(Some(val))
    }
}

/// # Examples
/// Value<i32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueInt;
/// let v: ValueInt = (&1).into();
/// assert_eq!(v, ValueInt(Some(1)));
///```
impl<T> From<&T> for ValueInt
where
    T: Into<ValueInt> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<i32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueInt;
/// let v: ValueInt = Some(1).into();
/// assert_eq!(v, ValueInt(Some(1)));
/// let v: ValueInt = Some(&1).into();
/// assert_eq!(v, ValueInt(Some(1)));
///```
impl<T> From<Option<T>> for ValueInt
where
    T: Into<ValueInt>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueInt(None),
        }
    }
}

// === revert ===
impl TryFrom<ValueInt> for Option<i32> {
    type Error = crate::Error;
    fn try_from(value: ValueInt) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<ValueInt> for i32 {
    type Error = crate::Error;
    fn try_from(value: ValueInt) -> Result<Self, Self::Error> {
        match value.0 {
            Some(v) => Ok(v),
            None => Err(crate::Error::Message("Value is None!".into())),
        }
    }
}
