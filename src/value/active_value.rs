use super::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveValue<V>
where
    V: Into<Value> + Clone,
{
    Changed(V, Box<ActiveValue<V>>),
    Unchanged(V),
    NotSet,
}

impl<V> ActiveValue<V>
where
    V: Into<Value> + Clone,
{
    ///
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::{Value, ActiveValue};
    /// let mut not_set = arel::ActiveValue::NotSet;
    /// assert_eq!(not_set.set(1), &ActiveValue::Changed(1, Box::new(ActiveValue::NotSet)));
    ///
    /// let mut unchanged = arel::ActiveValue::Unchanged(false);
    /// let old_value = unchanged.clone();
    /// assert_eq!(unchanged.set(true), &ActiveValue::Changed(true, Box::new(old_value)));
    ///
    /// let mut changed = arel::ActiveValue::Changed(1, Box::new(ActiveValue::NotSet));
    /// assert_eq!(changed.set(1), &ActiveValue::Changed(1, Box::new(ActiveValue::NotSet)));
    /// assert_eq!(changed.set(2), &ActiveValue::Changed(2, Box::new(ActiveValue::NotSet)));
    /// ```
    pub fn set(&mut self, v: V) -> &mut Self {
        match self {
            Self::Changed(nv, _) => {
                // *self = ActiveValue::Changed(v, ov.clone());
                *nv = v;
            }
            _ => *self = ActiveValue::Changed(v, Box::new(self.clone())),
        }
        self
    }
}
