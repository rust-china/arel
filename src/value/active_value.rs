use super::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveValue<V>
where
    V: Into<Value> + Clone + PartialEq,
{
    Changed(V, Box<ActiveValue<V>>),
    Unchanged(V),
    NotSet,
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
    pub fn set<UV>(&mut self, uv: UV) -> &mut Self
    where
        UV: Into<V>,
    {
        let to_v: V = uv.into();
        match self {
            Self::Changed(nv, ov) => {
                // *self = ActiveValue::Changed(v, ov.clone());
                match (*ov).as_ref() {
                    Self::Unchanged(ov) => {
                        if *ov == to_v {
                            *self = Self::Unchanged(to_v);
                        } else {
                            *nv = to_v;
                        }
                    }
                    _ => {
                        *nv = to_v;
                    }
                }
            }
            Self::Unchanged(ov) => {
                if *ov != to_v {
                    *self = ActiveValue::Changed(to_v, Box::new(self.clone()));
                }
            }
            _ => *self = ActiveValue::Changed(to_v, Box::new(self.clone())),
        }
        self
    }
    pub fn set_unchanged<UV>(&mut self, uv: UV) -> &mut Self
    where
        UV: Into<V>,
    {
        let to_v: V = uv.into();
        *self = Self::Unchanged(to_v);
        self
    }
    pub fn try_get_i32(&self) -> anyhow::Result<i32> {
        let value: crate::Value = match self {
            Self::Changed(nv, _) => nv.into(),
            Self::Unchanged(v) => v.into(),
            Self::NotSet => return Err(anyhow::anyhow!("No Value Set")),
        };
        Ok(value.try_get_i32()?)
    }
}
