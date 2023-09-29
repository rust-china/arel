use super::ValueString;
use std::borrow::Cow;

/// # Examples
/// Value<String>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueString;
/// let v: ValueString = "1".to_string().into();
/// assert_eq!(v, ValueString(Some("1".to_string())));
///```
impl From<String> for ValueString {
    fn from(val: String) -> Self {
        ValueString(Some(val))
    }
}

/// # Examples
/// Value<&str>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueString;
/// let v: ValueString = "1".to_string().into();
/// assert_eq!(v, ValueString(Some("1".to_string())));
///```
impl From<&str> for ValueString {
    fn from(val: &str) -> Self {
        val.to_string().into()
    }
}

/// # Examples
/// Value<Cow<'_, str>>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueString;
/// let v: ValueString = "1".to_string().into();
/// assert_eq!(v, ValueString(Some("1".to_string())));
///```
impl From<Cow<'_, str>> for ValueString {
    fn from(val: Cow<'_, str>) -> Self {
        val.into_owned().into()
    }
}

/// # Examples
/// Value<String>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueString;
/// let v: ValueString = (&"1".to_string()).into();
/// assert_eq!(v, ValueString(Some("1".to_string())));
///```
impl<T> From<&T> for ValueString
where
    T: Into<ValueString> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<String>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueString;
/// let v: ValueString = Some("1".to_string()).into();
/// assert_eq!(v, ValueString(Some("1".to_string())));
/// let v: ValueString = Some(&"1".to_string()).into();
/// assert_eq!(v, ValueString(Some("1".to_string())));
/// let v: ValueString = Some("2").into();
/// assert_eq!(v, ValueString(Some("2".to_string())));
///```
impl<T> From<Option<T>> for ValueString
where
    T: Into<ValueString>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueString(None),
        }
    }
}

// === revert ===
impl TryFrom<ValueString> for Option<String> {
    type Error = crate::Error;
    fn try_from(value: ValueString) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<ValueString> for String {
    type Error = crate::Error;
    fn try_from(value: ValueString) -> Result<Self, Self::Error> {
        match value.0 {
            Some(v) => Ok(v),
            None => Err(crate::Error::Message("Value is None!".into())),
        }
    }
}
