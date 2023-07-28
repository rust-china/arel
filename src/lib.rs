pub mod prelude;
pub mod sql;
pub mod statements;
pub mod traits;
pub mod value;

pub use bytes::Bytes;
pub use sql::Sql;
pub use value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveValue<V>
where
    V: Into<Value> + Clone,
{
    Changed(V, Option<Box<ActiveValue<V>>>),
    Unchanged(V),
    NotSet,
}

impl<V> ActiveValue<V>
where
    V: Into<Value> + Clone,
{
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// let active_value = ActiveValue::set(1);
    /// assert_eq!(active_value, ActiveValue::Changed(1, None));
    ///
    /// ```
    pub fn set(v: V) -> Self {
        Self::Changed(v, None)
    }
    ///
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// let mut not_set = ActiveValue::NotSet;
    /// assert_eq!(not_set.change(1), &ActiveValue::Changed(1, Some(Box::new(ActiveValue::NotSet))));
    ///
    /// let mut unchanged = ActiveValue::Unchanged(false);
    /// let old_value = unchanged.clone();
    /// assert_eq!(unchanged.change(true), &ActiveValue::Changed(true, Some(Box::new(old_value))));
    ///
    /// let mut changed = ActiveValue::Changed(1, Some(Box::new(ActiveValue::NotSet)));
    /// assert_eq!(changed.change(1), &ActiveValue::Changed(1, Some(Box::new(ActiveValue::NotSet))));
    /// assert_eq!(changed.change(2), &ActiveValue::Changed(2, Some(Box::new(ActiveValue::NotSet))));
    /// ```
    pub fn change(&mut self, v: V) -> &mut Self {
        match self {
            Self::Changed(_, ov) => {
                *self = ActiveValue::Changed(v, ov.clone());
            }
            Self::Unchanged(ov) => *self = ActiveValue::Changed(v, Some(Box::new(Self::Unchanged(ov.clone())))),
            Self::NotSet => *self = ActiveValue::Changed(v, Some(Box::new(Self::NotSet))),
        }
        self
    }
}

/// Defines a set operation on an [ActiveValue]
///
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let active_value = Change(1);
/// assert_eq!(active_value, ActiveValue::Changed(1, None));
///
/// ```
#[allow(non_snake_case)]
pub fn Change<V>(v: V) -> ActiveValue<V>
where
    V: Into<Value> + Clone,
{
    ActiveValue::set(v)
}
