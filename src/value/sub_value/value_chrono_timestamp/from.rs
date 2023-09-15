use super::ValueChronoTimestamp;

/// # Examples
/// Value<chrono::DateTime<chrono::FixedOffset>>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTimestamp;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_dt: chrono::DateTime<chrono::FixedOffset> = utc_dt.clone().into();
/// let v: ValueChronoTimestamp = fixed_dt.clone().into();
/// assert_eq!(v, ValueChronoTimestamp(Some(fixed_dt)));
///```
impl From<chrono::DateTime<chrono::FixedOffset>> for ValueChronoTimestamp {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        ValueChronoTimestamp(Some(val))
    }
}

/// # Examples
/// Value<chrono::DateTime<chrono::FixedOffset>>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTimestamp;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let v: ValueChronoTimestamp = utc_dt.clone().into();
/// let fixed_time: chrono::DateTime<chrono::FixedOffset> = utc_dt.clone().into();
/// assert_eq!(v, ValueChronoTimestamp(Some(fixed_time)));
///```
impl From<chrono::DateTime<chrono::Utc>> for ValueChronoTimestamp {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = val.into();
        ValueChronoTimestamp(Some(value))
    }
}

/// # Examples
/// Value<chrono::DateTime<chrono::FixedOffset>>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTimestamp;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_dt: chrono::DateTime<chrono::FixedOffset> = utc_dt.clone().into();
/// let v: ValueChronoTimestamp = (&fixed_dt.clone()).into();
/// assert_eq!(v, ValueChronoTimestamp(Some(fixed_dt)));
///```
impl<T> From<&T> for ValueChronoTimestamp
where
    T: Into<ValueChronoTimestamp> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<chrono::DateTime<chrono::FixedOffset>>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTimestamp;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_dt: chrono::DateTime<chrono::FixedOffset> = utc_dt.clone().into();
/// let v: ValueChronoTimestamp = Some(fixed_dt.clone()).into();
/// assert_eq!(v, ValueChronoTimestamp(Some(fixed_dt)));
/// let v: ValueChronoTimestamp = Some(&fixed_dt.clone()).into();
/// assert_eq!(v, ValueChronoTimestamp(Some(fixed_dt)));
///```
impl<T> From<Option<T>> for ValueChronoTimestamp
where
    T: Into<ValueChronoTimestamp>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueChronoTimestamp(None),
        }
    }
}
