use super::ValueBigUnsigned;

/// # Examples
/// Value<u64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBigUnsigned;
/// let v: ValueBigUnsigned = 1.into();
/// assert_eq!(v, ValueBigUnsigned(Some(1)));
///```
impl From<u64> for ValueBigUnsigned {
    fn from(val: u64) -> Self {
        ValueBigUnsigned(Some(val))
    }
}

/// # Examples
/// Value<u64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBigUnsigned;
/// let v: ValueBigUnsigned = (&1).into();
/// assert_eq!(v, ValueBigUnsigned(Some(1)));
///```
impl<T> From<&T> for ValueBigUnsigned
where
    T: Into<ValueBigUnsigned> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<u64>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBigUnsigned;
/// let v: ValueBigUnsigned = Some(1).into();
/// assert_eq!(v, ValueBigUnsigned(Some(1)));
/// let v: ValueBigUnsigned = Some(&1).into();
/// assert_eq!(v, ValueBigUnsigned(Some(1)));
///```
impl<T> From<Option<T>> for ValueBigUnsigned
where
    T: Into<ValueBigUnsigned>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueBigUnsigned(None),
        }
    }
}
