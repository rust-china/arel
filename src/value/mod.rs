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
    Json(Option<Box<Json>>),

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
    fn from(x: &str) -> Value {
        let string: String = x.into();
        Value::String(Some(Box::new(string)))
    }
}
impl From<&String> for Value {
    fn from(x: &String) -> Value {
        let string: String = x.into();
        Value::String(Some(Box::new(string)))
    }
}
impl From<String> for Value {
    fn from(string: String) -> Value {
        Value::String(Some(Box::new(string)))
    }
}
impl From<Cow<'_, str>> for Value {
    fn from(x: Cow<'_, str>) -> Value {
        x.into_owned().into()
    }
}
