mod eq;

use crate::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveValue<V>
where
    V: Into<Value> + Clone + PartialEq,
{
    Changed(V, Box<ActiveValue<V>>),
    Unchanged(V),
    NotSet,
}

impl<V> Serialize for ActiveValue<V>
where
    V: Into<Value> + Clone + PartialEq + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Changed(nv, _) => V::serialize(nv, serializer),
            Self::Unchanged(v) => V::serialize(v, serializer),
            Self::NotSet => serializer.serialize_none(),
        }
    }
}
impl<'de, V> Deserialize<'de> for ActiveValue<V>
where
    V: Into<Value> + Clone + PartialEq + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::Changed(V::deserialize(deserializer)?, Box::new(ActiveValue::NotSet)))
    }
}

impl<V> Default for ActiveValue<V>
where
    V: Into<Value> + Clone + PartialEq,
{
    fn default() -> Self {
        Self::NotSet
    }
}

impl<V> ActiveValue<V>
where
    V: Into<Value> + Clone + PartialEq,
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
    pub fn set<ToV>(&mut self, to_v: ToV) -> &mut Self
    where
        ToV: Into<V>,
    {
        let v: V = to_v.into();
        match self {
            Self::Changed(nv, ov) => {
                // *self = ActiveValue::Changed(v, ov.clone());
                match (*ov).as_ref() {
                    Self::Unchanged(ov) => {
                        if *ov == v {
                            *self = Self::Unchanged(v);
                        } else {
                            *nv = v;
                        }
                    }
                    _ => {
                        *nv = v;
                    }
                }
            }
            Self::Unchanged(ov) => {
                if *ov != v {
                    *self = ActiveValue::Changed(v, Box::new(self.clone()));
                }
            }
            _ => *self = ActiveValue::Changed(v, Box::new(self.clone())),
        }
        self
    }
    ///
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::{Value, ActiveValue};
    /// let mut not_set: ActiveValue<i32> = arel::ActiveValue::NotSet;
    /// assert_eq!(not_set.value(), None);
    ///
    /// let mut unchanged = arel::ActiveValue::Unchanged(false);
    /// assert_eq!(unchanged.value(), Some(&false));
    ///
    /// let mut changed = arel::ActiveValue::Changed(1, Box::new(ActiveValue::NotSet));
    /// assert_eq!(changed.value(), Some(&1));
    /// ```
    pub fn value(&self) -> Option<&V> {
        match self {
            Self::Changed(nv, _) => Some(nv),
            Self::Unchanged(v) => Some(v),
            _ => None,
        }
    }
}

#[allow(non_snake_case)]
pub fn Set<V, ToV>(to_v: ToV) -> ActiveValue<V>
where
    V: Into<Value> + Clone + PartialEq,
    ToV: Into<V>,
{
    let v: V = to_v.into();
    ActiveValue::Changed(v, Box::new(ActiveValue::NotSet))
}
