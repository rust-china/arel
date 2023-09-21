mod eq;
mod from;
mod ops;

pub mod active_vulue;
pub mod sub_value;

use serde::{Deserialize, Serialize};
use std::{
    cmp::PartialEq,
    ops::{Deref, DerefMut},
};

// https://docs.rs/sqlx/latest/sqlx/types/index.html
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Value {
    Bool(sub_value::ValueBool),
    TinyInt(sub_value::ValueTinyInt),
    SmallInt(sub_value::ValueSmallInt),
    Int(sub_value::ValueInt),
    BigInt(sub_value::ValueBigInt),

    #[cfg(any(feature = "sqlite", feature = "mysql"))]
    TinyUnsigned(sub_value::ValueTinyUnsigned),
    #[cfg(any(feature = "sqlite", feature = "mysql"))]
    SmallUnsigned(sub_value::ValueSmallUnsigned),
    #[cfg(any(feature = "sqlite", feature = "mysql"))]
    Unsigned(sub_value::ValueUnsigned),
    #[cfg(any(feature = "mysql"))]
    BigUnsigned(sub_value::ValueBigUnsigned),

    Float(sub_value::ValueFloat),
    Double(sub_value::ValueDouble),

    String(sub_value::ValueString),

    Bytes(sub_value::ValueBytes),

    Array(sub_value::ValueArray),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(sub_value::ValueJson),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTimestamp(sub_value::ValueChronoTimestamp),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(sub_value::ValueChronoDateTime),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate(sub_value::ValueChronoDate),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime(sub_value::ValueChronoTime),
}

impl Value {
    pub fn is_null(&self) -> bool {
        match self {
            crate::Value::Bool(val) => val.deref().is_none(),
            crate::Value::TinyInt(val) => val.deref().is_none(),
            crate::Value::SmallInt(val) => val.deref().is_none(),
            crate::Value::Int(val) => val.deref().is_none(),
            crate::Value::BigInt(val) => val.deref().is_none(),
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::TinyUnsigned(val) => val.deref().is_none(),
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::SmallUnsigned(val) => val.deref().is_none(),
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::Unsigned(val) => val.deref().is_none(),
            #[cfg(any(feature = "mysql"))]
            crate::Value::BigUnsigned(val) => val.deref().is_none(),
            crate::Value::Float(val) => val.deref().is_none(),
            crate::Value::Double(val) => val.deref().is_none(),
            crate::Value::String(val) => val.deref().is_none(),
            crate::Value::Bytes(val) => val.deref().is_none(),
            crate::Value::Array(val) => val.deref().is_none(),
            #[cfg(feature = "with-json")]
            crate::Value::Json(val) => val.deref().is_none(),
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoTimestamp(val) => val.deref().is_none(),
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoDateTime(val) => val.deref().is_none(),
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoDate(val) => val.deref().is_none(),
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoTime(val) => val.deref().is_none(),
        }
    }
    pub fn set_null(&mut self) {
        match self {
            crate::Value::Bool(val) => *val.deref_mut() = None,
            crate::Value::TinyInt(val) => *val.deref_mut() = None,
            crate::Value::SmallInt(val) => *val.deref_mut() = None,
            crate::Value::Int(val) => *val.deref_mut() = None,
            crate::Value::BigInt(val) => *val.deref_mut() = None,
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::TinyUnsigned(val) => *val.deref_mut() = None,
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::SmallUnsigned(val) => *val.deref_mut() = None,
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::Unsigned(val) => *val.deref_mut() = None,
            #[cfg(any(feature = "mysql"))]
            crate::Value::BigUnsigned(val) => *val.deref_mut() = None,
            crate::Value::Float(val) => *val.deref_mut() = None,
            crate::Value::Double(val) => *val.deref_mut() = None,
            crate::Value::String(val) => *val.deref_mut() = None,
            crate::Value::Bytes(val) => *val.deref_mut() = None,
            crate::Value::Array(val) => *val.deref_mut() = None,
            #[cfg(feature = "with-json")]
            crate::Value::Json(val) => *val.deref_mut() = None,
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoTimestamp(val) => *val.deref_mut() = None,
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoDateTime(val) => *val.deref_mut() = None,
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoDate(val) => *val.deref_mut() = None,
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoTime(val) => *val.deref_mut() = None,
        };
    }
}
