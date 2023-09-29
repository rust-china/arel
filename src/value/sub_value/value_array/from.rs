use super::ValueArray;

impl<T> From<Vec<T>> for ValueArray
where
    T: Into<crate::Value>,
{
    fn from(vals: Vec<T>) -> Self {
        let values = vals.into_iter().map(|v| v.into()).collect();
        ValueArray(Some(values))
    }
}

impl<T> From<&Vec<T>> for ValueArray
where
    T: Into<crate::Value> + Clone,
{
    fn from(vals: &Vec<T>) -> Self {
        vals.clone().into()
    }
}

impl<T> From<Option<Vec<T>>> for ValueArray
where
    T: Into<crate::Value>,
{
    fn from(vals: Option<Vec<T>>) -> Self {
        match vals {
            Some(values) => values.into(),
            None => ValueArray(None),
        }
    }
}

// === revert ===
impl TryFrom<ValueArray> for Option<Vec<crate::Value>> {
    type Error = crate::Error;
    fn try_from(value: ValueArray) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<ValueArray> for Vec<crate::Value> {
    type Error = crate::Error;
    fn try_from(value: ValueArray) -> Result<Self, Self::Error> {
        match value.0 {
            Some(v) => Ok(v),
            None => Err(crate::Error::Message("Value is None!".into())),
        }
    }
}
