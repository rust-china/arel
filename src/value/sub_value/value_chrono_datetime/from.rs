use super::ValueChronoDateTime;
use std::time::SystemTime;

/// # Examples
/// Value<chrono::NaiveDateTime>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDateTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let native_dt = utc_dt.naive_local();
/// let v: ValueChronoDateTime = native_dt.clone().into();
/// assert_eq!(v, ValueChronoDateTime(Some(native_dt)));
///```
impl From<chrono::NaiveDateTime> for ValueChronoDateTime {
    fn from(val: chrono::NaiveDateTime) -> Self {
        ValueChronoDateTime(Some(val))
    }
}

/// # Examples
/// Value<chrono::NaiveDateTime>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDateTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let v: ValueChronoDateTime = utc_dt.clone().into();
/// assert_eq!(v, ValueChronoDateTime(Some(utc_dt.naive_local())));
///```
impl From<chrono::DateTime<chrono::Utc>> for ValueChronoDateTime {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = val.naive_local();
        ValueChronoDateTime(Some(value))
    }
}

/// # Examples
/// Value<chrono::NaiveDateTime>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDateTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_dt: chrono::DateTime<chrono::FixedOffset> = utc_dt.clone().into();
/// let v: ValueChronoDateTime = fixed_dt.clone().into();
/// assert_eq!(v, ValueChronoDateTime(Some(fixed_dt.naive_local())));
///```
impl From<chrono::DateTime<chrono::FixedOffset>> for ValueChronoDateTime {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let value = val.naive_local();
        ValueChronoDateTime(Some(value))
    }
}

impl From<SystemTime> for ValueChronoDateTime {
    fn from(value: SystemTime) -> Self {
        let utc_value: chrono::DateTime<chrono::Utc> = value.into();
        utc_value.into()
    }
}

/// # Examples
/// Value<chrono::NaiveDateTime>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDateTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let native_dt = utc_dt.naive_local();
/// let v: ValueChronoDateTime = (&native_dt.clone()).into();
/// assert_eq!(v, ValueChronoDateTime(Some(native_dt)));
///```
impl<T> From<&T> for ValueChronoDateTime
where
    T: Into<ValueChronoDateTime> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<chrono::NaiveDateTime>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDateTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let native_dt = utc_dt.naive_local();
/// let v: ValueChronoDateTime = Some(native_dt.clone()).into();
/// assert_eq!(v, ValueChronoDateTime(Some(native_dt)));
/// let v: ValueChronoDateTime = Some(&native_dt.clone()).into();
/// assert_eq!(v, ValueChronoDateTime(Some(native_dt)));
///```
impl<T> From<Option<T>> for ValueChronoDateTime
where
    T: Into<ValueChronoDateTime>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueChronoDateTime(None),
        }
    }
}
