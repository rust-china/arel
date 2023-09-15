use super::ValueChronoDate;

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDate;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let naive_date = utc_dt.date_naive();
/// let v: ValueChronoDate = naive_date.clone().into();
/// assert_eq!(v, ValueChronoDate(Some(naive_date)));
///```
impl From<chrono::NaiveDate> for ValueChronoDate {
    fn from(val: chrono::NaiveDate) -> Self {
        ValueChronoDate(Some(val))
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDate;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let native_dt = utc_dt.naive_local();
/// let v: ValueChronoDate = native_dt.clone().into();
/// assert_eq!(v, ValueChronoDate(Some(native_dt.date())));
///```
impl From<chrono::NaiveDateTime> for ValueChronoDate {
    fn from(val: chrono::NaiveDateTime) -> Self {
        let value = val.date();
        ValueChronoDate(Some(value))
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDate;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let v: ValueChronoDate = utc_dt.clone().into();
/// assert_eq!(v, ValueChronoDate(Some(utc_dt.date_naive())));
///```
impl From<chrono::DateTime<chrono::Utc>> for ValueChronoDate {
    fn from(val: chrono::DateTime<chrono::Utc>) -> Self {
        let value = val.date_naive();
        ValueChronoDate(Some(value))
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDate;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let fixed_dt: chrono::DateTime<chrono::FixedOffset> = utc_dt.clone().into();
/// let v: ValueChronoDate = fixed_dt.clone().into();
/// assert_eq!(v, ValueChronoDate(Some(fixed_dt.date_naive())));
///```
impl From<chrono::DateTime<chrono::FixedOffset>> for ValueChronoDate {
    fn from(val: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let value = val.date_naive();
        ValueChronoDate(Some(value))
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDate;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let naive_date = utc_dt.date_naive();
/// let v: ValueChronoDate = (&naive_date.clone()).into();
/// assert_eq!(v, ValueChronoDate(Some(naive_date)));
///```
impl<T> From<&T> for ValueChronoDate
where
    T: Into<ValueChronoDate> + Clone,
{
    fn from(val: &T) -> Self {
        val.clone().into()
    }
}

/// # Examples
/// Value<chrono::NaiveDate>
/// ```
/// use arel::prelude::*;
/// use arel::value::sub_value::ValueChronoDate;
/// use chrono::{TimeZone};
/// let utc_dt = chrono::Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
/// let naive_date = utc_dt.date_naive();
/// let v: ValueChronoDate = Some(naive_date.clone()).into();
/// assert_eq!(v, ValueChronoDate(Some(naive_date)));
/// let v: ValueChronoDate = Some(&naive_date.clone()).into();
/// assert_eq!(v, ValueChronoDate(Some(naive_date)));
///```
impl<T> From<Option<T>> for ValueChronoDate
where
    T: Into<ValueChronoDate>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(value) => value.into(),
            None => ValueChronoDate(None),
        }
    }
}
