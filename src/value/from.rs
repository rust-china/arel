use super::{sub_value, Value};

impl From<sub_value::ValueBool> for Value {
    fn from(val: sub_value::ValueBool) -> Self {
        Value::Bool(val)
    }
}

impl From<sub_value::ValueTinyInt> for Value {
    fn from(val: sub_value::ValueTinyInt) -> Self {
        Value::TinyInt(val)
    }
}

impl From<sub_value::ValueSmallInt> for Value {
    fn from(val: sub_value::ValueSmallInt) -> Self {
        Value::SmallInt(val)
    }
}

impl From<sub_value::ValueInt> for Value {
    fn from(val: sub_value::ValueInt) -> Self {
        Value::Int(val)
    }
}

impl From<sub_value::ValueBigInt> for Value {
    fn from(val: sub_value::ValueBigInt) -> Self {
        Value::BigInt(val)
    }
}

#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<sub_value::ValueTinyUnsigned> for Value {
    fn from(val: sub_value::ValueTinyUnsigned) -> Self {
        Value::TinyUnsigned(val)
    }
}
#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<sub_value::ValueSmallUnsigned> for Value {
    fn from(val: sub_value::ValueSmallUnsigned) -> Self {
        Value::SmallUnsigned(val)
    }
}
#[cfg(any(feature = "sqlite", feature = "mysql"))]
impl From<sub_value::ValueUnsigned> for Value {
    fn from(val: sub_value::ValueUnsigned) -> Self {
        Value::Unsigned(val.into())
    }
}
#[cfg(any(feature = "mysql"))]
impl From<sub_value::ValueBigUnsigned> for Value {
    fn from(val: sub_value::ValueBigUnsigned) -> Self {
        Value::BigUnsigned(val.into())
    }
}

impl From<sub_value::ValueFloat> for Value {
    fn from(val: sub_value::ValueFloat) -> Self {
        Value::Float(val)
    }
}

impl From<sub_value::ValueDouble> for Value {
    fn from(val: sub_value::ValueDouble) -> Self {
        Value::Double(val)
    }
}

impl From<sub_value::ValueString> for Value {
    fn from(val: sub_value::ValueString) -> Self {
        Value::String(val)
    }
}

impl From<sub_value::ValueBytes> for Value {
    fn from(val: sub_value::ValueBytes) -> Self {
        Value::Bytes(val)
    }
}

impl From<sub_value::ValueArray> for Value {
    fn from(vals: sub_value::ValueArray) -> Self {
        Value::Array(vals)
    }
}

#[cfg(feature = "with-json")]
impl From<sub_value::ValueJson> for Value {
    fn from(val: sub_value::ValueJson) -> Self {
        Value::Json(val)
    }
}

#[cfg(feature = "with-chrono")]
impl From<sub_value::ValueChronoTimestamp> for Value {
    fn from(val: sub_value::ValueChronoTimestamp) -> Self {
        Value::ChronoTimestamp(val)
    }
}

#[cfg(feature = "with-chrono")]
impl From<sub_value::ValueChronoDateTime> for Value {
    fn from(val: sub_value::ValueChronoDateTime) -> Self {
        Value::ChronoDateTime(val)
    }
}

#[cfg(feature = "with-chrono")]
impl From<sub_value::ValueChronoDate> for Value {
    fn from(val: sub_value::ValueChronoDate) -> Self {
        Value::ChronoDate(val)
    }
}

#[cfg(feature = "with-chrono")]
impl From<sub_value::ValueChronoTime> for Value {
    fn from(val: sub_value::ValueChronoTime) -> Self {
        Value::ChronoTime(val)
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
