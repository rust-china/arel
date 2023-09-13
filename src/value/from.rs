use super::Value;
use std::borrow::Cow;

/// # Examples
/// Value<bool>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<bool> = true.into();
/// assert_eq!(v, arel::Value::Bool(true));
///```
impl From<bool> for Value<bool> {
    fn from(val: bool) -> Self {
        Value::Bool(val)
    }
}
/// # Examples
/// Value<i8>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<i8> = 1.into();
/// assert_eq!(v, arel::Value::TinyInt(1));
///```
impl From<i8> for Value<i8> {
    fn from(val: i8) -> Self {
        Value::TinyInt(val)
    }
}
/// # Examples
/// Value<i16>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<i16> = 1.into();
/// assert_eq!(v, arel::Value::SmallInt(1));
///```
impl From<i16> for Value<i16> {
    fn from(val: i16) -> Self {
        Value::SmallInt(val)
    }
}
/// # Examples
/// Value<i32>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<i32> = 1.into();
/// assert_eq!(v, arel::Value::Int(1));
///```
impl From<i32> for Value<i32> {
    fn from(val: i32) -> Self {
        Value::Int(val)
    }
}
/// # Examples
/// Value<i64>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<i64> = 1.into();
/// assert_eq!(v, arel::Value::BigInt(1));
///```
impl From<i64> for Value<i64> {
    fn from(val: i64) -> Self {
        Value::BigInt(val)
    }
}
/// # Examples
/// Value<u8>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<u8> = 1.into();
/// assert_eq!(v, arel::Value::TinyUnsigned(1));
///```
impl From<u8> for Value<u8> {
    fn from(val: u8) -> Self {
        Value::TinyUnsigned(val)
    }
}
/// # Examples
/// Value<u16>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<u16> = 1.into();
/// assert_eq!(v, arel::Value::SmallUnsigned(1));
///```
impl From<u16> for Value<u16> {
    fn from(val: u16) -> Self {
        Value::SmallUnsigned(val)
    }
}
/// # Examples
/// Value<u32>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<u32> = 1.into();
/// assert_eq!(v, arel::Value::Unsigned(1));
///```
impl From<u32> for Value<u32> {
    fn from(val: u32) -> Self {
        Value::Unsigned(val)
    }
}
/// # Examples
/// Value<u64>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<u64> = 1.into();
/// assert_eq!(v, arel::Value::BigUnsigned(1));
///```
impl From<u64> for Value<u64> {
    fn from(val: u64) -> Self {
        Value::BigUnsigned(val)
    }
}
/// # Examples
/// Value<f32>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<f32> = 1.0.into();
/// assert_eq!(v, arel::Value::Float(1.0));
///```
impl From<f32> for Value<f32> {
    fn from(val: f32) -> Self {
        Value::Float(val)
    }
}
/// # Examples
/// Value<f64>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<f64> = 1.0.into();
/// assert_eq!(v, arel::Value::Double(1.0));
///```
impl From<f64> for Value<f64> {
    fn from(val: f64) -> Self {
        Value::Double(val)
    }
}
/// # Examples
/// Value<char>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<char> = 'a'.into();
/// assert_eq!(v, arel::Value::Char('a'));
///```
impl From<char> for Value<char> {
    fn from(val: char) -> Self {
        Value::Char(val)
    }
}
/// # Examples
/// Value<String>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<String> = "abc".into();
/// assert_eq!(v, arel::Value::String("abc".to_string()));
///```
impl From<&str> for Value<String> {
    fn from(val: &str) -> Self {
        let string: String = val.into();
        Value::String(string)
    }
}
/// # Examples
/// Value<String>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<String> = "abc".to_string().into();
/// assert_eq!(v, arel::Value::String("abc".to_string()));
///```
impl From<String> for Value<String> {
    fn from(val: String) -> Self {
        Value::String(val)
    }
}
/// # Examples
/// Value<String>
/// ```
/// use arel::prelude::*;
/// use std::borrow::Cow;
/// let v: arel::Value<String> = Cow::Borrowed("abc").into();
/// assert_eq!(v, arel::Value::String("abc".to_string()));
///```
impl From<Cow<'_, str>> for Value<String> {
    fn from(val: Cow<'_, str>) -> Self {
        val.into_owned().into()
    }
}
/// # Examples
/// Value<bytes::Bytes>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<arel::Bytes> = arel::Bytes::from("abc").into();
/// assert_eq!(v, arel::Value::Bytes(arel::Bytes::from("abc")));
///```
impl From<bytes::Bytes> for Value<bytes::Bytes> {
    fn from(val: bytes::Bytes) -> Self {
        Value::Bytes(val)
    }
}

/// # Examples
/// Value<Vec<T>>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<Vec<i32>> = vec![1,2,3].into();
/// assert_eq!(v, arel::Value::Array(vec![1,2,3]));
///```
impl<T: Into<Value<T>>> From<Vec<T>> for Value<Vec<T>> {
    fn from(vals: Vec<T>) -> Self {
        Value::Array(vals)
    }
}
/// # Examples
/// Value<serde_json::Value>
/// ```
/// use arel::prelude::*;
/// let json: serde_json::Value = serde_json::from_str(r#"{"hello":"world"}"#).unwrap();
/// let v: arel::Value<serde_json::Value> = json.clone().into();
/// assert_eq!(v, arel::Value::Json(json));
///```
#[cfg(feature = "with-json")]
impl From<serde_json::Value> for Value<serde_json::Value> {
    fn from(val: serde_json::Value) -> Self {
        Value::Json(val)
    }
}
/// # Examples
/// Value<chrono::DateTime<chrono::FixedOffset>>
/// ```
/// use arel::prelude::*;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_time = utc_time.clone().into();
/// let v: arel::Value<chrono::DateTime<chrono::FixedOffset>> = utc_time.into();
/// assert_eq!(v, arel::Value::ChronoTimestamp(fixed_time));
///```
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::Utc>> for Value<chrono::DateTime<chrono::FixedOffset>> {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = chrono::DateTime::<chrono::FixedOffset>::from(val);
        Value::ChronoTimestamp(value)
    }
}
/// # Examples
/// Value<chrono::DateTime<chrono::FixedOffset>>
/// ```
/// use arel::prelude::*;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_time: chrono::DateTime<chrono::FixedOffset> = utc_time.clone().into();
/// let v: arel::Value<chrono::DateTime<chrono::FixedOffset>> = fixed_time.clone().into();
/// assert_eq!(v, arel::Value::ChronoTimestamp(fixed_time));
///```
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::FixedOffset>> for Value<chrono::DateTime<chrono::FixedOffset>> {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Value::ChronoTimestamp(val)
    }
}
/// # Examples
/// Value<chrono::NaiveDateTime>
/// ```
/// use arel::prelude::*;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let v: arel::Value<chrono::NaiveDateTime> = utc_time.clone().into();
/// assert_eq!(v, arel::Value::ChronoDateTime(utc_time.naive_local()));
///```
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::Utc>> for Value<chrono::NaiveDateTime> {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = val.naive_local();
        Value::ChronoDateTime(value)
    }
}
/// # Examples
/// Value<chrono::NaiveDateTime>
/// ```
/// use arel::prelude::*;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_time: chrono::DateTime<chrono::FixedOffset> = utc_time.clone().into();
/// let v: arel::Value<chrono::NaiveDateTime> = fixed_time.clone().into();
/// assert_eq!(v, arel::Value::ChronoDateTime(utc_time.naive_local()));
///```
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::FixedOffset>> for Value<chrono::NaiveDateTime> {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let value = val.naive_local();
        Value::ChronoDateTime(value)
    }
}
/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let v: arel::Value<chrono::NaiveDate> = utc_time.clone().into();
/// assert_eq!(v, arel::Value::ChronoDate(utc_time.date_naive()));
///```
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::Utc>> for Value<chrono::NaiveDate> {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = val.date_naive();
        Value::ChronoDate(value)
    }
}
/// # Examples
/// Value<chrono::NaiveTime>
/// ```
/// use arel::prelude::*;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_time: chrono::DateTime<chrono::FixedOffset> = utc_time.clone().into();
/// let v: arel::Value<chrono::NaiveDate> = fixed_time.clone().into();
/// assert_eq!(v, arel::Value::ChronoDate(utc_time.date_naive()));
///```
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::FixedOffset>> for Value<chrono::NaiveDate> {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let value = val.date_naive();
        Value::ChronoDate(value)
    }
}
/// # Examples
/// Value<chrono::NaiveTime>
/// ```
/// use arel::prelude::*;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let v: arel::Value<chrono::NaiveTime> = utc_time.clone().into();
/// assert_eq!(v, arel::Value::ChronoTime(utc_time.time()));
///```
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::Utc>> for Value<chrono::NaiveTime> {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = val.time();
        Value::ChronoTime(value)
    }
}
/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_time: chrono::DateTime<chrono::FixedOffset> = utc_time.clone().into();
/// let v: arel::Value<chrono::NaiveTime> = fixed_time.clone().into();
/// assert_eq!(v, arel::Value::ChronoTime(utc_time.time()));
///```
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::FixedOffset>> for Value<chrono::NaiveTime> {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let value = val.time();
        Value::ChronoTime(value)
    }
}
/// # Examples
/// Value<T>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<i32> = (&1).into();
/// assert_eq!(v, arel::Value::Int(1));
///
/// let v: arel::Value<String> = (&"abc".to_string()).into();
/// assert_eq!(v, arel::Value::String("abc".to_string()));
///
/// let v: arel::Value<Vec<i32>> = (&vec![1,2,3]).into();
/// assert_eq!(v, arel::Value::Array(vec![1,2,3]));
///```
impl<T> From<&T> for Value<T>
where
    T: Clone + Into<Value<T>>,
{
    fn from(value: &T) -> Self {
        value.clone().into()
    }
}
/// # Examples
/// Value<T>
/// ```
/// use arel::prelude::*;
/// let v: arel::Value<i32> = Some(1).into();
/// assert_eq!(v, arel::Value::Int(1));
///
/// let v: arel::Value<String> = Some("abc".to_string()).into();
/// assert_eq!(v, arel::Value::String("abc".to_string()));
///
/// let v: arel::Value<Vec<i32>> = Some(vec![1,2,3]).into();
/// assert_eq!(v, arel::Value::Array(vec![1,2,3]));
///```
impl<U, T> From<Option<T>> for Value<U>
where
    T: Into<Value<U>>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::Null,
        }
    }
}
