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
