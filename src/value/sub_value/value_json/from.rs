use super::ValueJson;

/// # Examples
/// Value<serde_json::Value>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueJson;
/// let json: serde_json::Value = serde_json::from_str(r#"{"hello":"world"}"#).unwrap();
/// let v: ValueJson = json.clone().into();
/// assert_eq!(v, ValueJson(Some(json)));
///```
impl From<serde_json::Value> for ValueJson {
    fn from(val: serde_json::Value) -> Self {
        ValueJson(Some(val))
    }
}

/// # Examples
/// Value<serde_json::Value>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueJson;
/// let json: serde_json::Value = serde_json::from_str(r#"{"hello":"world"}"#).unwrap();
/// let v: ValueJson = (&json.clone()).into();
/// assert_eq!(v, ValueJson(Some(json)));
///```
impl<T> From<&T> for ValueJson
where
    T: Into<ValueJson> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<serde_json::Value>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueJson;
/// let json: serde_json::Value = serde_json::from_str(r#"{"hello":"world"}"#).unwrap();
/// let v: ValueJson = Some(json.clone()).into();
/// assert_eq!(v, ValueJson(Some(json.clone())));
/// let v: ValueJson = Some(&json.clone()).into();
/// assert_eq!(v, ValueJson(Some(json)));
///```
impl<T> From<Option<T>> for ValueJson
where
    T: Into<ValueJson>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueJson(None),
        }
    }
}
