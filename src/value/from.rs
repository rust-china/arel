use super::{sub_value, Value};

impl From<sub_value::ValueBool> for Value {
    fn from(val: sub_value::ValueBool) -> Self {
        Value::Bool(val)
    }
}
impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Bool(val.into())
    }
}
// impl From<Option<bool>> for Value {
//     fn from(val: Option<bool>) -> Self {
//         Value::Bool(val.into())
//     }
// }

impl From<sub_value::ValueTinyInt> for Value {
    fn from(val: sub_value::ValueTinyInt) -> Self {
        Value::TinyInt(val)
    }
}
impl From<i8> for Value {
    fn from(val: i8) -> Self {
        Value::TinyInt(val.into())
    }
}
// impl From<Option<i8>> for Value {
//     fn from(val: Option<i8>) -> Self {
//         Value::TinyInt(val.into())
//     }
// }

impl From<sub_value::ValueSmallInt> for Value {
    fn from(val: sub_value::ValueSmallInt) -> Self {
        Value::SmallInt(val)
    }
}
impl From<i16> for Value {
    fn from(val: i16) -> Self {
        Value::SmallInt(val.into())
    }
}
// impl From<Option<i16>> for Value {
//     fn from(val: Option<i16>) -> Self {
//         Value::SmallInt(val.into())
//     }
// }

impl From<sub_value::ValueInt> for Value {
    fn from(val: sub_value::ValueInt) -> Self {
        Value::Int(val)
    }
}
impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Value::Int(val.into())
    }
}
// impl From<Option<i32>> for Value {
//     fn from(val: Option<i32>) -> Self {
//         Value::Int(val.into())
//     }
// }

impl From<sub_value::ValueBigInt> for Value {
    fn from(val: sub_value::ValueBigInt) -> Self {
        Value::BigInt(val)
    }
}
impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Value::BigInt(val.into())
    }
}
// impl From<Option<i64>> for Value {
//     fn from(val: Option<i64>) -> Self {
//         Value::BigInt(val.into())
//     }
// }

#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<sub_value::ValueTinyUnsigned> for Value {
    fn from(val: sub_value::ValueTinyUnsigned) -> Self {
        Value::TinyUnsigned(val)
    }
}
#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<u8> for Value {
    fn from(val: u8) -> Self {
        Value::TinyUnsigned(val.into())
    }
}
// #[cfg(any(feature = "sqlite", feature = "mysql"))]
// impl From<Option<u8>> for Value {
//     fn from(val: Option<u8>) -> Self {
//         Value::TinyUnsigned(val.into())
//     }
// }

#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<sub_value::ValueSmallUnsigned> for Value {
    fn from(val: sub_value::ValueSmallUnsigned) -> Self {
        Value::SmallUnsigned(val)
    }
}
#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<u16> for Value {
    fn from(val: u16) -> Self {
        Value::SmallUnsigned(val.into())
    }
}
// #[cfg(any(feature = "sqlite", feature = "mysql"))]
// impl From<Option<u16>> for Value {
//     fn from(val: Option<u16>) -> Self {
//         Value::SmallUnsigned(val.into())
//     }
// }

#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<sub_value::ValueUnsigned> for Value {
    fn from(val: sub_value::ValueUnsigned) -> Self {
        Value::Unsigned(val)
    }
}
#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<u32> for Value {
    fn from(val: u32) -> Self {
        Value::Unsigned(val.into())
    }
}
// #[cfg(any(feature = "sqlite", feature = "mysql"))]
// impl From<Option<u32>> for Value {
//     fn from(val: Option<u32>) -> Self {
//         Value::Unsigned(val.into())
//     }
// }

#[cfg(any(feature = "mysql"))]
impl From<sub_value::ValueBigUnsigned> for Value {
    fn from(val: sub_value::ValueBigUnsigned) -> Self {
        Value::BigUnsigned(val)
    }
}
#[cfg(any(feature = "mysql"))]
impl From<u64> for Value {
    fn from(val: u64) -> Self {
        Value::BigUnsigned(val.into())
    }
}
// #[cfg(any(feature = "mysql"))]
// impl From<Option<u64>> for Value {
//     fn from(val: Option<u64>) -> Self {
//         Value::BigUnsigned(val.into())
//     }
// }

impl From<sub_value::ValueFloat> for Value {
    fn from(val: sub_value::ValueFloat) -> Self {
        Value::Float(val)
    }
}
impl From<f32> for Value {
    fn from(val: f32) -> Self {
        Value::Float(val.into())
    }
}
// impl From<Option<f32>> for Value {
//     fn from(val: Option<f32>) -> Self {
//         Value::Float(val.into())
//     }
// }

impl From<sub_value::ValueDouble> for Value {
    fn from(val: sub_value::ValueDouble) -> Self {
        Value::Double(val)
    }
}
impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Double(val.into())
    }
}
// impl From<Option<f64>> for Value {
//     fn from(val: Option<f64>) -> Self {
//         Value::Double(val.into())
//     }
// }

impl From<sub_value::ValueString> for Value {
    fn from(val: sub_value::ValueString) -> Self {
        Value::String(val)
    }
}
impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::String(val.into())
    }
}
// impl From<Option<String>> for Value {
//     fn from(val: Option<String>) -> Self {
//         Value::String(val.into())
//     }
// }
impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Value::String(val.into())
    }
}

impl From<sub_value::ValueBytes> for Value {
    fn from(val: sub_value::ValueBytes) -> Self {
        Value::Bytes(val)
    }
}
impl From<bytes::Bytes> for Value {
    fn from(val: bytes::Bytes) -> Self {
        Value::Bytes(val.into())
    }
}
// impl From<Option<bytes::Bytes>> for Value {
//     fn from(val: Option<bytes::Bytes>) -> Self {
//         Value::Bytes(val.into())
//     }
// }

impl From<sub_value::ValueArray> for Value {
    fn from(vals: sub_value::ValueArray) -> Self {
        Value::Array(vals)
    }
}
impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(vals: Vec<T>) -> Self {
        Value::Array(vals.into())
    }
}
// impl<T> From<Option<Vec<T>>> for Value
// where
//     T: Into<Value>,
// {
//     fn from(vals: Option<Vec<T>>) -> Self {
//         Value::Array(vals.into())
//     }
// }

#[cfg(feature = "with-json")]
impl From<sub_value::ValueJson> for Value {
    fn from(val: sub_value::ValueJson) -> Self {
        Value::Json(val)
    }
}
impl From<serde_json::Value> for Value {
    fn from(val: serde_json::Value) -> Self {
        Value::Json(val.into())
    }
}
// impl From<Option<serde_json::Value>> for Value {
//     fn from(val: Option<serde_json::Value>) -> Self {
//         Value::Json(val.into())
//     }
// }

#[cfg(feature = "with-chrono")]
impl From<sub_value::ValueChronoTimestamp> for Value {
    fn from(val: sub_value::ValueChronoTimestamp) -> Self {
        Value::ChronoTimestamp(val)
    }
}
impl From<chrono::DateTime<chrono::FixedOffset>> for Value {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Value::ChronoTimestamp(val.into())
    }
}
// impl From<Option<chrono::DateTime<chrono::FixedOffset>>> for Value {
//     fn from(val: Option<chrono::DateTime<chrono::FixedOffset>>) -> Self {
//         Value::ChronoTimestamp(val.into())
//     }
// }
impl From<chrono::DateTime<chrono::Utc>> for Value {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        Value::ChronoTimestamp(val.into())
    }
}
// impl From<Option<chrono::DateTime<chrono::Utc>>> for Value {
//     fn from(val: Option<chrono::DateTime<chrono::Utc>>) -> Self {
//         Value::ChronoTimestamp(val.into())
//     }
// }

#[cfg(feature = "with-chrono")]
impl From<sub_value::ValueChronoDateTime> for Value {
    fn from(val: sub_value::ValueChronoDateTime) -> Self {
        Value::ChronoDateTime(val)
    }
}
impl From<chrono::NaiveDateTime> for Value {
    fn from(val: chrono::NaiveDateTime) -> Self {
        Value::ChronoDateTime(val.into())
    }
}
// impl From<Option<chrono::NaiveDateTime>> for Value {
//     fn from(val: Option<chrono::NaiveDateTime>) -> Self {
//         Value::ChronoDateTime(val.into())
//     }
// }

#[cfg(feature = "with-chrono")]
impl From<sub_value::ValueChronoDate> for Value {
    fn from(val: sub_value::ValueChronoDate) -> Self {
        Value::ChronoDate(val)
    }
}
impl From<chrono::NaiveDate> for Value {
    fn from(val: chrono::NaiveDate) -> Self {
        Value::ChronoDate(val.into())
    }
}
// impl From<Option<chrono::NaiveDate>> for Value {
//     fn from(val: Option<chrono::NaiveDate>) -> Self {
//         Value::ChronoDate(val.into())
//     }
// }

#[cfg(feature = "with-chrono")]
impl From<sub_value::ValueChronoTime> for Value {
    fn from(val: sub_value::ValueChronoTime) -> Self {
        Value::ChronoTime(val)
    }
}
impl From<chrono::NaiveTime> for Value {
    fn from(val: chrono::NaiveTime) -> Self {
        Value::ChronoTime(val.into())
    }
}
// impl From<Option<chrono::NaiveTime>> for Value {
//     fn from(val: Option<chrono::NaiveTime>) -> Self {
//         Value::ChronoTime(val.into())
//     }
// }

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
    T: Clone + Into<Value> + Default,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => T::default().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sub_value;

    #[test]
    fn it_works() {
        let value: Value = 1i32.into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(1))));
        let value: Value = (&1i32).into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(1))));

        let value: Value = Some(1i32).into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(1))));
        let value: Value = (&Some(1i32)).into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(1))));

        let value: Value = Option::<i32>::None.into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(0))));
        let value: Value = (&Option::<i32>::None).into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(0))));

        let v1: sub_value::ValueInt = 1.into();
        let value: Value = v1.clone().into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(1))));
        let v1: sub_value::ValueInt = 1.into();
        let value: Value = (&v1).into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(1))));

        let v2 = Some(v1.clone());
        let value: Value = v2.into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(1))));
        let v2 = Some(v1.clone());
        let value: Value = (&v2).into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(Some(1))));

        let v3: Option<sub_value::ValueInt> = None;
        let value: Value = v3.into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(None)));
        let v3: Option<sub_value::ValueInt> = None;
        let value: Value = (&v3).into();
        assert_eq!(value, Value::Int(sub_value::ValueInt(None)));
    }
}
