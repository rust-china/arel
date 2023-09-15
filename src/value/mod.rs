mod eq;
mod from;
mod ops;

pub mod sub_value;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

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
