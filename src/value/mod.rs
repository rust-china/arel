mod eq;
mod from;
mod ops;

use std::cmp::PartialEq;

// https://docs.rs/sqlx/latest/sqlx/types/index.html
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Value<T: Sized> {
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
    String(String),

    #[allow(clippy::box_collection)]
    Bytes(bytes::Bytes),

    Array(T),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(serde_json::Value),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTimestamp(chrono::DateTime<chrono::FixedOffset>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(chrono::NaiveDateTime),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate(chrono::NaiveDate),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime(chrono::NaiveTime),

    Null,
    _Unreachable(std::marker::PhantomData<T>),
}
