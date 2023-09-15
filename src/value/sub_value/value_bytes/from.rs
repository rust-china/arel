use super::ValueBytes;

/// # Examples
/// Value<bytes::Bytes>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBytes;
/// let v: ValueBytes = arel::Bytes::from("abc").into();
/// assert_eq!(v, ValueBytes(Some(arel::Bytes::from("abc").into())));
///```
impl From<bytes::Bytes> for ValueBytes {
    fn from(val: bytes::Bytes) -> Self {
        ValueBytes(Some(val))
    }
}

/// # Examples
/// Value<bytes::Bytes>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBytes;
/// let v: ValueBytes = (&arel::Bytes::from("abc")).into();
/// assert_eq!(v, ValueBytes(Some(arel::Bytes::from("abc").into())));
///```
impl<T> From<&T> for ValueBytes
where
    T: Into<ValueBytes> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<bytes::Bytes>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueBytes;
/// let v: ValueBytes = Some(arel::Bytes::from("abc")).into();
/// assert_eq!(v, ValueBytes(Some(arel::Bytes::from("abc").into())));
/// let v: ValueBytes = Some(&arel::Bytes::from("abc")).into();
/// assert_eq!(v, ValueBytes(Some(arel::Bytes::from("abc").into())));
///```
impl<T> From<Option<T>> for ValueBytes
where
    T: Into<ValueBytes>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueBytes(None),
        }
    }
}
