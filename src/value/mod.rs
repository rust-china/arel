mod active_value;

pub use active_value::ActiveValue;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Bool(bool),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    TinyUnsigned(u8),
    SmallUnsigned(u16),
    Unsigned(u32),
    BigUnsigned(u64),
    Float(f32),
    Double(f64),
    Char(char),
    String(Box<String>),

    #[allow(clippy::box_collection)]
    Bytes(Box<bytes::Bytes>),
    Array(Box<Vec<Value>>),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Box<serde_json::Value>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(Box<chrono::DateTime<chrono::FixedOffset>>),

    Null,
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Bool(val)
    }
}

impl From<i8> for Value {
    fn from(val: i8) -> Self {
        Value::TinyInt(val)
    }
}

impl From<i16> for Value {
    fn from(val: i16) -> Self {
        Value::SmallInt(val)
    }
}

impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Value::Int(val)
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Value::BigInt(val)
    }
}

impl From<u8> for Value {
    fn from(val: u8) -> Self {
        Value::TinyUnsigned(val)
    }
}

impl From<u16> for Value {
    fn from(val: u16) -> Self {
        Value::SmallUnsigned(val)
    }
}

impl From<u32> for Value {
    fn from(val: u32) -> Self {
        Value::Unsigned(val)
    }
}

impl From<u64> for Value {
    fn from(val: u64) -> Self {
        Value::BigUnsigned(val)
    }
}

impl From<f32> for Value {
    fn from(val: f32) -> Self {
        Value::Float(val)
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Double(val)
    }
}

impl From<char> for Value {
    fn from(val: char) -> Self {
        Value::Char(val)
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        let string: String = val.into();
        Value::String(Box::new(string))
    }
}
impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::String(Box::new(val))
    }
}
impl From<Cow<'_, str>> for Value {
    fn from(val: Cow<'_, str>) -> Self {
        val.into_owned().into()
    }
}

impl From<bytes::Bytes> for Value {
    fn from(val: bytes::Bytes) -> Self {
        Value::Bytes(Box::new(val))
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(vals: Vec<T>) -> Self {
        let vals = vals.into_iter().map(|v| v.into()).collect();
        Value::Array(Box::new(vals))
    }
}

#[cfg(feature = "with-json")]
impl From<serde_json::Value> for Value {
    fn from(val: serde_json::Value) -> Self {
        Value::Json(Box::new(val))
    }
}

#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::Utc>> for Value {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = chrono::DateTime::<chrono::FixedOffset>::from(val);
        Value::ChronoDateTime(Box::new(value))
    }
}
#[cfg(feature = "with-chrono")]
impl From<chrono::DateTime<chrono::FixedOffset>> for Value {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Value::ChronoDateTime(Box::new(val))
    }
}

impl<T> From<&T> for Value
where
    T: Clone + Into<Value>,
{
    fn from(value: &T) -> Self {
        value.clone().into()
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

impl Value {
    pub fn try_get_i32(&self) -> anyhow::Result<i32> {
        match self {
            Self::TinyInt(v) => Ok(*v as i32),
            Self::SmallInt(v) => Ok(*v as i32),
            Self::Int(v) => Ok(*v),
            Self::BigInt(v) => Ok(*v as i32),
            Self::TinyUnsigned(v) => Ok(*v as i32),
            Self::SmallUnsigned(v) => Ok(*v as i32),
            Self::Unsigned(v) => Ok(*v as i32),
            Self::BigUnsigned(v) => Ok(*v as i32),
            Self::Null => Ok(0),
            _ => Err(anyhow::anyhow!("value type {:?} cant not to i32", self)),
        }
    }
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
            Value::Bool(val) => {
                if *val {
                    Some(crate::Sql::new("1"))
                } else {
                    Some(crate::Sql::new("0"))
                }
            }
            Value::TinyInt(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::SmallInt(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::Int(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::BigInt(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::TinyUnsigned(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::SmallUnsigned(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::Unsigned(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::BigUnsigned(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::Float(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::Double(val) => Some(crate::Sql::new(format!("{}", val))),
            Value::Char(val) => {
                let mut sql = crate::Sql::default();
                sql.push_str_with_prepare_value(sql.prepare_symbol(), *val);
                Some(sql)
            }
            Value::String(val) => {
                let mut sql = crate::Sql::default();
                sql.push_str_with_prepare_value(sql.prepare_symbol(), val.to_string());
                Some(sql)
            }
            Value::Bytes(val) => {
                let mut sql = crate::Sql::default();
                sql.push_str_with_prepare_value(sql.prepare_symbol(), bytes::Bytes::copy_from_slice(val));
                Some(sql)
            }
            Value::Array(val) => {
                let mut sql = crate::Sql::new("(");
                let vec_sqls: Vec<crate::Sql> = val.iter().filter_map(|v| v.to_sql()).collect();
                let vec_sqls_len = val.len();
                for (idx, v_sql) in vec_sqls.into_iter().enumerate() {
                    sql.push_sql(v_sql);
                    if idx < vec_sqls_len - 1 {
                        sql.push(',');
                    }
                }
                sql.push(')');
                Some(sql)
            }
            #[cfg(feature = "with-json")]
            Value::Json(val) => {
                let mut sql = crate::Sql::default();
                sql.push_str_with_prepare_value(sql.prepare_symbol(), serde_json::to_string(val).unwrap());
                Some(sql)
            }
            #[cfg(feature = "with-chrono")]
            Value::ChronoDateTime(val) => {
                let mut sql = crate::Sql::default();
                sql.push_str_with_prepare_value(sql.prepare_symbol(), format!("{}", val));
                Some(sql)
            }
            Value::Null => None,
        };
        raw_sql_string
    }
}
