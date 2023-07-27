use std::borrow::Cow;

pub trait Nullable {
    fn null() -> Value;
}

#[derive(Clone, Debug, PartialEq)]
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
    Bytes(Option<Box<Vec<u8>>>),

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
impl From<i8> for Value {
    fn from(val: i8) -> Value {
        Value::TinyInt(Some(val))
    }
}
impl From<i16> for Value {
    fn from(val: i16) -> Value {
        Value::SmallInt(Some(val))
    }
}
impl From<i32> for Value {
    fn from(val: i32) -> Value {
        Value::Int(Some(val))
    }
}
impl From<i64> for Value {
    fn from(val: i64) -> Value {
        Value::BigInt(Some(val))
    }
}
impl From<u8> for Value {
    fn from(val: u8) -> Value {
        Value::TinyUnsigned(Some(val))
    }
}
impl From<u16> for Value {
    fn from(val: u16) -> Value {
        Value::SmallUnsigned(Some(val))
    }
}
impl From<u32> for Value {
    fn from(val: u32) -> Value {
        Value::Unsigned(Some(val))
    }
}
impl From<u64> for Value {
    fn from(val: u64) -> Value {
        Value::BigUnsigned(Some(val))
    }
}
impl From<f32> for Value {
    fn from(val: f32) -> Value {
        Value::Float(Some(val))
    }
}
impl From<f64> for Value {
    fn from(val: f64) -> Value {
        Value::Double(Some(val))
    }
}
impl From<char> for Value {
    fn from(val: char) -> Value {
        Value::Char(Some(val))
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

impl From<&Vec<u8>> for Value {
    fn from(val: &Vec<u8>) -> Self {
        Value::Bytes(Some(Box::new(val.clone())))
    }
}

impl From<Vec<u8>> for Value {
    fn from(val: Vec<u8>) -> Self {
        Value::Bytes(Some(Box::new(val)))
    }
}

impl From<&serde_json::Value> for Value {
    fn from(val: &serde_json::Value) -> Self {
        Value::Json(Some(Box::new(val.clone())))
    }
}

impl From<serde_json::Value> for Value {
    fn from(val: serde_json::Value) -> Self {
        Value::Json(Some(Box::new(val)))
    }
}

impl From<&chrono::DateTime<chrono::Utc>> for Value {
    fn from(val: &chrono::DateTime<chrono::Utc>) -> Self {
        let value = chrono::DateTime::<chrono::FixedOffset>::from(val.clone());
        Value::ChronoDateTime(Some(Box::new(value)))
    }
}
impl From<chrono::DateTime<chrono::Utc>> for Value {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = chrono::DateTime::<chrono::FixedOffset>::from(val);
        Value::ChronoDateTime(Some(Box::new(value)))
    }
}
impl From<&chrono::DateTime<chrono::FixedOffset>> for Value {
    fn from(val: &chrono::DateTime<chrono::FixedOffset>) -> Self {
        Value::ChronoDateTime(Some(Box::new(val.clone())))
    }
}
impl From<chrono::DateTime<chrono::FixedOffset>> for Value {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Value::ChronoDateTime(Some(Box::new(val)))
    }
}

impl Value {
    ///
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use chrono::{TimeZone};
    /// let value: Value = true.into();
    /// assert_eq!(value.to_sql().to_sql_string().unwrap(), "TRUE");
    /// let value: Value = false.into();
    /// assert_eq!(value.to_sql().to_sql_string().unwrap(), "FALSE");
    /// let value: Value = 0.into();
    /// assert_eq!(value.to_sql().to_sql_string().unwrap(), "0");
    /// let value: Value = 0.1.into();
    /// assert_eq!(value.to_sql().to_sql_string().unwrap(), "0.1");
    /// let vec: Vec<u8> = vec![1, 2, 3];
    /// let value: Value = vec.into();
    /// assert_eq!(value.to_sql().to_sql_string().unwrap(), "[1,2,3]");
    /// let json: serde_json::Value = serde_json::from_str(r#"{"hello":"world"}"#).unwrap();
    /// let value: Value = json.into();
    /// assert_eq!(value.to_sql().to_sql_string().unwrap(), r#"{"hello":"world"}"#);
    /// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
    /// let value: Value = utc_time.into();
    /// assert_eq!(value.to_sql().to_sql_string().unwrap(), r#"2023-12-31 00:00:00 +00:00"#);
    /// ```
    pub fn to_sql(&self) -> crate::Sql {
        let raw_sql_string = match self {
            Value::Bool(val) => match val {
                Some(v) => {
                    if *v {
                        crate::Sql::new("TRUE")
                    } else {
                        crate::Sql::new("FALSE")
                    }
                }
                None => crate::Sql::new("NULL"),
            },
            Value::TinyInt(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("NULL"),
            },
            Value::SmallInt(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("NULL"),
            },
            Value::Int(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("null"),
            },
            Value::BigInt(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("NULL"),
            },
            Value::TinyUnsigned(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("NULL"),
            },
            Value::SmallUnsigned(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("null"),
            },
            Value::Unsigned(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("NULL"),
            },
            Value::BigUnsigned(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("null"),
            },
            Value::Float(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("NULL"),
            },
            Value::Double(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new("NULL"),
            },
            Value::Char(val) => match val {
                Some(v) => crate::Sql::new(format!("'{}'", v)),
                None => crate::Sql::new("NULL"),
            },
            Value::String(val) => match val {
                Some(v) => crate::Sql::new(format!(r#""{}""#, v)),
                None => crate::Sql::new("NULL"),
            },
            Value::Bytes(val) => match val {
                Some(v) => crate::Sql::new(serde_json::to_string(v).unwrap()),
                None => crate::Sql::new("null"),
            },
            Value::Json(val) => match val {
                Some(v) => crate::Sql::new(serde_json::to_string(v).unwrap()),
                None => crate::Sql::new("NULL"),
            },
            Value::ChronoDateTime(val) => match val {
                Some(v) => crate::Sql::new(format!("{}", v)),
                None => crate::Sql::new(String::from("NULL")),
            },
        };
        raw_sql_string
    }
}
