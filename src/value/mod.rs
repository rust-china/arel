mod active_value;

pub use active_value::ActiveValue;
use std::borrow::Cow;

pub trait Nullable {
    fn null() -> Value;
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Bool(Option<bool>),
    TinyInt(Option<i8>),
    SmallInt(Option<i16>),
    Int(Option<i32>),
    BigInt(Option<i64>),
    TinyUnsigned(Option<u8>),
    SmallUnsigned(Option<u16>),
    Unsigned(Option<u32>),
    BigUnsigned(Option<u64>),
    Float(Option<f32>),
    Double(Option<f64>),
    Char(Option<char>),
    String(Option<Box<String>>),

    #[allow(clippy::box_collection)]
    Bytes(Option<Box<bytes::Bytes>>),
    Array(Option<Box<Vec<Value>>>),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Option<Box<serde_json::Value>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(Option<Box<chrono::DateTime<chrono::FixedOffset>>>),
}

impl From<bool> for Value {
    fn from(val: bool) -> Value {
        Value::Bool(Some(val))
    }
}
impl From<&bool> for Value {
    fn from(val: &bool) -> Value {
        Value::Bool(Some(*val))
    }
}
impl From<Option<bool>> for Value {
    fn from(val: Option<bool>) -> Value {
        Value::Bool(val)
    }
}
impl From<Option<&bool>> for Value {
    fn from(val: Option<&bool>) -> Value {
        Value::Bool(val.copied())
    }
}
impl From<i8> for Value {
    fn from(val: i8) -> Value {
        Value::TinyInt(Some(val))
    }
}
impl From<&i8> for Value {
    fn from(val: &i8) -> Value {
        Value::TinyInt(Some(*val))
    }
}
impl From<Option<i8>> for Value {
    fn from(val: Option<i8>) -> Value {
        Value::TinyInt(val)
    }
}
impl From<Option<&i8>> for Value {
    fn from(val: Option<&i8>) -> Value {
        Value::TinyInt(val.copied())
    }
}
impl From<i16> for Value {
    fn from(val: i16) -> Value {
        Value::SmallInt(Some(val))
    }
}
impl From<&i16> for Value {
    fn from(val: &i16) -> Value {
        Value::SmallInt(Some(*val))
    }
}
impl From<Option<i16>> for Value {
    fn from(val: Option<i16>) -> Value {
        Value::SmallInt(val)
    }
}
impl From<Option<&i16>> for Value {
    fn from(val: Option<&i16>) -> Value {
        Value::SmallInt(val.copied())
    }
}
impl From<i32> for Value {
    fn from(val: i32) -> Value {
        Value::Int(Some(val))
    }
}
impl From<&i32> for Value {
    fn from(val: &i32) -> Value {
        Value::Int(Some(*val))
    }
}
impl From<Option<i32>> for Value {
    fn from(val: Option<i32>) -> Value {
        Value::Int(val)
    }
}
impl From<Option<&i32>> for Value {
    fn from(val: Option<&i32>) -> Value {
        Value::Int(val.copied())
    }
}
impl From<i64> for Value {
    fn from(val: i64) -> Value {
        Value::BigInt(Some(val))
    }
}
impl From<&i64> for Value {
    fn from(val: &i64) -> Value {
        Value::BigInt(Some(*val))
    }
}
impl From<Option<i64>> for Value {
    fn from(val: Option<i64>) -> Value {
        Value::BigInt(val)
    }
}
impl From<Option<&i64>> for Value {
    fn from(val: Option<&i64>) -> Value {
        Value::BigInt(val.copied())
    }
}
impl From<u8> for Value {
    fn from(val: u8) -> Value {
        Value::TinyUnsigned(Some(val))
    }
}
impl From<&u8> for Value {
    fn from(val: &u8) -> Value {
        Value::TinyUnsigned(Some(*val))
    }
}
impl From<Option<u8>> for Value {
    fn from(val: Option<u8>) -> Value {
        Value::TinyUnsigned(val)
    }
}
impl From<Option<&u8>> for Value {
    fn from(val: Option<&u8>) -> Value {
        Value::TinyUnsigned(val.copied())
    }
}
impl From<u16> for Value {
    fn from(val: u16) -> Value {
        Value::SmallUnsigned(Some(val))
    }
}
impl From<&u16> for Value {
    fn from(val: &u16) -> Value {
        Value::SmallUnsigned(Some(*val))
    }
}
impl From<Option<u16>> for Value {
    fn from(val: Option<u16>) -> Value {
        Value::SmallUnsigned(val)
    }
}
impl From<Option<&u16>> for Value {
    fn from(val: Option<&u16>) -> Value {
        Value::SmallUnsigned(val.copied())
    }
}
impl From<u32> for Value {
    fn from(val: u32) -> Value {
        Value::Unsigned(Some(val))
    }
}
impl From<&u32> for Value {
    fn from(val: &u32) -> Value {
        Value::Unsigned(Some(*val))
    }
}
impl From<Option<u32>> for Value {
    fn from(val: Option<u32>) -> Value {
        Value::Unsigned(val)
    }
}
impl From<Option<&u32>> for Value {
    fn from(val: Option<&u32>) -> Value {
        Value::Unsigned(val.copied())
    }
}
impl From<u64> for Value {
    fn from(val: u64) -> Value {
        Value::BigUnsigned(Some(val))
    }
}
impl From<&u64> for Value {
    fn from(val: &u64) -> Value {
        Value::BigUnsigned(Some(*val))
    }
}
impl From<Option<u64>> for Value {
    fn from(val: Option<u64>) -> Value {
        Value::BigUnsigned(val)
    }
}
impl From<Option<&u64>> for Value {
    fn from(val: Option<&u64>) -> Value {
        Value::BigUnsigned(val.copied())
    }
}
impl From<f32> for Value {
    fn from(val: f32) -> Value {
        Value::Float(Some(val))
    }
}
impl From<&f32> for Value {
    fn from(val: &f32) -> Value {
        Value::Float(Some(*val))
    }
}
impl From<Option<f32>> for Value {
    fn from(val: Option<f32>) -> Value {
        Value::Float(val)
    }
}
impl From<Option<&f32>> for Value {
    fn from(val: Option<&f32>) -> Value {
        Value::Float(val.copied())
    }
}
impl From<f64> for Value {
    fn from(val: f64) -> Value {
        Value::Double(Some(val))
    }
}
impl From<&f64> for Value {
    fn from(val: &f64) -> Value {
        Value::Double(Some(*val))
    }
}
impl From<Option<f64>> for Value {
    fn from(val: Option<f64>) -> Value {
        Value::Double(val)
    }
}
impl From<Option<&f64>> for Value {
    fn from(val: Option<&f64>) -> Value {
        Value::Double(val.copied())
    }
}
impl From<char> for Value {
    fn from(val: char) -> Value {
        Value::Char(Some(val))
    }
}
impl From<&char> for Value {
    fn from(val: &char) -> Value {
        Value::Char(Some(*val))
    }
}
impl From<Option<char>> for Value {
    fn from(val: Option<char>) -> Value {
        Value::Char(val)
    }
}
impl From<Option<&char>> for Value {
    fn from(val: Option<&char>) -> Value {
        Value::Char(val.copied())
    }
}
impl From<&str> for Value {
    fn from(val: &str) -> Value {
        let string: String = val.into();
        Value::String(Some(Box::new(string)))
    }
}
impl From<&String> for Value {
    fn from(val: &String) -> Value {
        let string: String = val.into();
        Value::String(Some(Box::new(string)))
    }
}
impl From<String> for Value {
    fn from(val: String) -> Value {
        Value::String(Some(Box::new(val)))
    }
}
impl From<Cow<'_, str>> for Value {
    fn from(val: Cow<'_, str>) -> Value {
        val.into_owned().into()
    }
}
impl From<Option<&str>> for Value {
    fn from(val: Option<&str>) -> Value {
        let string = val.map(|s| Box::new(s.to_string()));
        Value::String(string)
    }
}
impl From<Option<&String>> for Value {
    fn from(val: Option<&String>) -> Value {
        let string = val.map(|s| Box::new(s.to_string()));
        Value::String(string)
    }
}
impl From<Option<String>> for Value {
    fn from(val: Option<String>) -> Value {
        let string = val.map(|s| Box::new(s));
        Value::String(string)
    }
}
impl From<Option<Cow<'_, str>>> for Value {
    fn from(val: Option<Cow<'_, str>>) -> Value {
        let string = val.map(|s| Box::new(s.into_owned()));
        Value::String(string)
    }
}

impl From<&bytes::Bytes> for Value {
    fn from(val: &bytes::Bytes) -> Self {
        Value::Bytes(Some(Box::new(val.clone())))
    }
}
impl From<bytes::Bytes> for Value {
    fn from(val: bytes::Bytes) -> Self {
        Value::Bytes(Some(Box::new(val)))
    }
}
impl From<Option<&bytes::Bytes>> for Value {
    fn from(val: Option<&bytes::Bytes>) -> Self {
        let bytes = val.map(|s| Box::new(s.clone()));
        Value::Bytes(bytes)
    }
}
impl From<Option<bytes::Bytes>> for Value {
    fn from(val: Option<bytes::Bytes>) -> Self {
        let bytes = val.map(|s| Box::new(s));
        Value::Bytes(bytes)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(vals: Vec<T>) -> Self {
        let vals = vals.into_iter().map(|v| v.into()).collect();
        Value::Array(Some(Box::new(vals)))
    }
}
impl<T: Into<Value>> From<Option<Vec<T>>> for Value {
    fn from(values: Option<Vec<T>>) -> Self {
        let vals = values.map(|vals| Box::new(vals.into_iter().map(|v| v.into()).collect()));
        Value::Array(vals)
    }
}

#[cfg(feature = "with-json")]
impl From<&serde_json::Value> for Value {
    fn from(val: &serde_json::Value) -> Self {
        Value::Json(Some(Box::new(val.clone())))
    }
}
#[cfg(feature = "with-json")]
impl From<serde_json::Value> for Value {
    fn from(val: serde_json::Value) -> Self {
        Value::Json(Some(Box::new(val)))
    }
}
#[cfg(feature = "with-json")]
impl From<Option<&serde_json::Value>> for Value {
    fn from(val: Option<&serde_json::Value>) -> Self {
        let val = val.map(|v| Box::new(v.clone()));
        Value::Json(val)
    }
}
#[cfg(feature = "with-json")]
impl From<Option<serde_json::Value>> for Value {
    fn from(val: Option<serde_json::Value>) -> Self {
        let val = val.map(|v| Box::new(v));
        Value::Json(val)
    }
}

#[cfg(feature = "with-chrono")]
impl From<&chrono::DateTime<chrono::Utc>> for Value {
    fn from(val: &chrono::DateTime<chrono::Utc>) -> Self {
        let value = chrono::DateTime::<chrono::FixedOffset>::from(val.clone());
        Value::ChronoDateTime(Some(Box::new(value)))
    }
}
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::Utc>> for Value {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = chrono::DateTime::<chrono::FixedOffset>::from(val);
        Value::ChronoDateTime(Some(Box::new(value)))
    }
}
#[cfg(feature = "with-chrono")]
impl From<&chrono::DateTime<chrono::FixedOffset>> for Value {
    fn from(val: &chrono::DateTime<chrono::FixedOffset>) -> Self {
        Value::ChronoDateTime(Some(Box::new(val.clone())))
    }
}
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::FixedOffset>> for Value {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Value::ChronoDateTime(Some(Box::new(val)))
    }
}
#[cfg(feature = "with-chrono")]
impl From<Option<&chrono::DateTime<chrono::Utc>>> for Value {
    fn from(val: Option<&chrono::DateTime<chrono::Utc>>) -> Self {
        let value = val.map(|v| Box::new(chrono::DateTime::<chrono::FixedOffset>::from(v.clone())));
        Value::ChronoDateTime(value)
    }
}
#[cfg(feature = "with-chrono")]
impl From<Option<chrono::DateTime<chrono::Utc>>> for Value {
    fn from(val: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        let value = val.map(|v| Box::new(chrono::DateTime::<chrono::FixedOffset>::from(v)));
        Value::ChronoDateTime(value)
    }
}
#[cfg(feature = "with-chrono")]
impl From<Option<&chrono::DateTime<chrono::FixedOffset>>> for Value {
    fn from(val: Option<&chrono::DateTime<chrono::FixedOffset>>) -> Self {
        let value = val.map(|v| Box::new(v.clone()));
        Value::ChronoDateTime(value)
    }
}
#[cfg(feature = "with-chrono")]
impl From<Option<chrono::DateTime<chrono::FixedOffset>>> for Value {
    fn from(val: Option<chrono::DateTime<chrono::FixedOffset>>) -> Self {
        let value = val.map(|v| Box::new(v));
        Value::ChronoDateTime(value)
    }
}

impl<T> From<&Option<T>> for Value
where
    for<'a> Option<&'a T>: Into<Value>,
{
    fn from(value: &Option<T>) -> Self {
        let value = match value {
            Some(v) => Some(v),
            None => None,
        };
        value.into()
    }
}

impl Value {
    ///
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::Value;
    /// use chrono::{TimeZone};
    ///
    /// let value: Value = true.into();
    /// assert_eq!(value.to_sql().unwrap().to_sql_string().unwrap(), "1");
    ///
    /// let value: Value = false.into();
    /// assert_eq!(value.to_sql().unwrap().to_sql_string().unwrap(), "0");
    ///
    /// let value: Value = 0.into();
    /// assert_eq!(value.to_sql().unwrap().to_sql_string().unwrap(), "0");
    ///
    /// let value: Value = 0.1.into();
    /// assert_eq!(value.to_sql().unwrap().to_sql_string().unwrap(), "0.1");
    ///
    /// let value: Value = bytes::Bytes::from_static(b"hello").into();
    /// assert_eq!(value.to_sql().unwrap().to_sql_string().unwrap(), r#"?b"hello""#);
    ///
    /// let value: Value = vec![1, 2, 3].into();
    /// assert_eq!(value.to_sql().unwrap().to_sql_string().unwrap(), "(1,2,3)");
    ///
    /// let json: serde_json::Value = serde_json::from_str(r#"{"hello":"world"}"#).unwrap();
    /// let value: Value = json.into();
    /// assert_eq!(value.to_sql().unwrap().to_sql_string().unwrap(), r#"?{"String":"{\"hello\":\"world\"}"}"#);
    ///
    /// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
    /// let value: Value = utc_time.into();
    /// assert_eq!(value.to_sql().unwrap().to_sql_string().unwrap(), r#"?{"String":"2023-12-31 00:00:00 +00:00"}"#);
    /// ```
    pub fn to_sql(&self) -> Option<crate::Sql> {
        let raw_sql_string = match self {
            Value::Bool(val) => match val {
                Some(v) => {
                    if *v {
                        Some(crate::Sql::new("1"))
                    } else {
                        Some(crate::Sql::new("0"))
                    }
                }
                None => None,
            },
            Value::TinyInt(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::SmallInt(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::Int(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::BigInt(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::TinyUnsigned(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::SmallUnsigned(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::Unsigned(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::BigUnsigned(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::Float(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::Double(val) => match val {
                Some(v) => Some(crate::Sql::new(format!("{}", v))),
                None => None,
            },
            Value::Char(val) => match val {
                Some(v) => {
                    let mut sql = crate::Sql::default();
                    sql.push_str_with_prepare_value(sql.prepare_symbol(), *v);
                    Some(sql)
                }
                None => None,
            },
            Value::String(val) => match val {
                Some(v) => {
                    let mut sql = crate::Sql::default();
                    sql.push_str_with_prepare_value(sql.prepare_symbol(), v.to_string());
                    Some(sql)
                }
                None => None,
            },
            Value::Bytes(val) => match val {
                Some(v) => {
                    let mut sql = crate::Sql::default();
                    sql.push_str_with_prepare_value(sql.prepare_symbol(), bytes::Bytes::copy_from_slice(v));
                    Some(sql)
                }
                None => None,
            },
            Value::Array(val) => match val {
                Some(vec) => {
                    let mut sql = crate::Sql::new("(");
                    let vec_sqls: Vec<crate::Sql> = vec.iter().filter_map(|v| v.to_sql()).collect();
                    let vec_sqls_len = vec.len();
                    for (idx, v_sql) in vec_sqls.into_iter().enumerate() {
                        sql.push_sql(v_sql);
                        if idx < vec_sqls_len - 1 {
                            sql.push(',');
                        }
                    }
                    sql.push(')');
                    Some(sql)
                }
                None => None,
            },
            #[cfg(feature = "with-json")]
            Value::Json(val) => match val {
                Some(v) => {
                    let mut sql = crate::Sql::default();
                    sql.push_str_with_prepare_value(sql.prepare_symbol(), serde_json::to_string(v).unwrap());
                    Some(sql)
                }
                None => None,
            },
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(val) => match val {
                Some(v) => {
                    let mut sql = crate::Sql::default();
                    sql.push_str_with_prepare_value(sql.prepare_symbol(), format!("{}", v));
                    Some(sql)
                }
                None => None,
            },
        };
        raw_sql_string
    }
}
