use super::ValueChronoTime;
use std::time::SystemTime;

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let naive_time = utc_dt.time();
/// let v: ValueChronoTime = naive_time.clone().into();
/// assert_eq!(v, ValueChronoTime(Some(naive_time)));
///```
impl From<chrono::NaiveTime> for ValueChronoTime {
    fn from(val: chrono::NaiveTime) -> Self {
        ValueChronoTime(Some(val))
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTime;
/// use chrono::{TimeZone};
/// let utc_time = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let native_dt = utc_time.naive_local();
/// let v: ValueChronoTime = native_dt.clone().into();
/// assert_eq!(v, ValueChronoTime(Some(native_dt.time())));
///```
impl From<chrono::NaiveDateTime> for ValueChronoTime {
    fn from(val: chrono::NaiveDateTime) -> Self {
        let value = val.time();
        ValueChronoTime(Some(value))
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let v: ValueChronoTime = utc_dt.clone().into();
/// assert_eq!(v, ValueChronoTime(Some(utc_dt.time())));
///```
impl From<chrono::DateTime<chrono::Utc>> for ValueChronoTime {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = val.time();
        ValueChronoTime(Some(value))
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_dt: chrono::DateTime<chrono::FixedOffset> = utc_dt.clone().into();
/// let v: ValueChronoTime = fixed_dt.clone().into();
/// assert_eq!(v, ValueChronoTime(Some(fixed_dt.time())));
///```
impl From<chrono::DateTime<chrono::FixedOffset>> for ValueChronoTime {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let value = val.time();
        ValueChronoTime(Some(value))
    }
}

impl From<SystemTime> for ValueChronoTime {
    fn from(value: SystemTime) -> Self {
        let utc_value: chrono::DateTime<chrono::Utc> = value.into();
        utc_value.into()
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let naive_time = utc_dt.time();
/// let v: ValueChronoTime = (&naive_time.clone()).into();
/// assert_eq!(v, ValueChronoTime(Some(naive_time)));
///```
impl<T> From<&T> for ValueChronoTime
where
    T: Into<ValueChronoTime> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoTime;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let naive_time = utc_dt.time();
/// let v: ValueChronoTime = Some(naive_time.clone()).into();
/// assert_eq!(v, ValueChronoTime(Some(naive_time)));
/// let v: ValueChronoTime = Some(&naive_time.clone()).into();
/// assert_eq!(v, ValueChronoTime(Some(naive_time)));
///```
impl<T> From<Option<T>> for ValueChronoTime
where
    T: Into<ValueChronoTime>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueChronoTime(None),
        }
    }
}
