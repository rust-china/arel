use super::ValueFloat;

/// # Examples
/// Value<f32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueFloat;
/// let v: ValueFloat = 1.0.into();
/// assert_eq!(v, ValueFloat(Some(1.0)));
///```
impl From<f32> for ValueFloat {
    fn from(val: f32) -> Self {
        ValueFloat(Some(val))
    }
}

/// # Examples
/// Value<f32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueFloat;
/// let v: ValueFloat = (&1.0).into();
/// assert_eq!(v, ValueFloat(Some(1.0)));
///```
impl<T> From<&T> for ValueFloat
where
    T: Into<ValueFloat> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<f32>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueFloat;
/// let v: ValueFloat = Some(1.0).into();
/// assert_eq!(v, ValueFloat(Some(1.0)));
/// let v: ValueFloat = Some(&1.0).into();
/// assert_eq!(v, ValueFloat(Some(1.0)));
///```
impl<T> From<Option<T>> for ValueFloat
where
    T: Into<ValueFloat>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueFloat(None),
        }
    }
}

// === revert ===
impl TryFrom<ValueFloat> for Option<f32> {
    type Error = crate::Error;
    fn try_from(value: ValueFloat) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

impl TryFrom<ValueFloat> for f32 {
    type Error = crate::Error;
    fn try_from(value: ValueFloat) -> Result<Self, Self::Error> {
        match value.0 {
            Some(v) => Ok(v),
            None => Err(crate::Error::Message("Value is None!".into())),
        }
    }
}
