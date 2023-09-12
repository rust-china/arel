use crate::{ActiveValue, Value};
use std::ops::{Add, AddAssign};

impl<V> Add<ActiveValue<V>> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Add<V, Output = V> + Default,
{
    type Output = Self;
    fn add(mut self, rhs: ActiveValue<V>) -> Self::Output {
        match &mut self {
            ActiveValue::Changed(nv, ov) => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(*nv + rnv, ov.clone()),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(*nv + rv, ov.clone()),
                ActiveValue::NotSet => (),
            },
            ActiveValue::Unchanged(v) => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(*v + rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(*v + rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
            ActiveValue::NotSet => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(V::default() + rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(V::default() + rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
        };
        self
    }
}

impl<V> AddAssign<ActiveValue<V>> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Add<V, Output = V> + Default,
{
    fn add_assign(&mut self, rhs: Self) {
        match self {
            ActiveValue::Changed(nv, ov) => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(*nv + rnv, ov.clone()),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(*nv + rv, ov.clone()),
                ActiveValue::NotSet => (),
            },
            ActiveValue::Unchanged(v) => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(*v + rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(*v + rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
            ActiveValue::NotSet => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(V::default() + rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(V::default() + rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
        };
    }
}
impl<V> Add<&ActiveValue<V>> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Add<V, Output = V> + Default,
{
    type Output = Self;
    fn add(mut self, rhs: &ActiveValue<V>) -> Self::Output {
        match &mut self {
            ActiveValue::Changed(nv, ov) => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(*nv + *rnv, ov.clone()),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(*nv + *rv, ov.clone()),
                ActiveValue::NotSet => (),
            },
            ActiveValue::Unchanged(v) => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(*v + *rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(*v + *rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
            ActiveValue::NotSet => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(V::default() + *rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(V::default() + *rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
        };
        self
    }
}
impl<V> AddAssign<&ActiveValue<V>> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Add<V, Output = V> + Default,
{
    fn add_assign(&mut self, rhs: &Self) {
        match self {
            ActiveValue::Changed(nv, ov) => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(*nv + *rnv, ov.clone()),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(*nv + *rv, ov.clone()),
                ActiveValue::NotSet => (),
            },
            ActiveValue::Unchanged(v) => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(*v + *rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(*v + *rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
            ActiveValue::NotSet => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(V::default() + *rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(V::default() + *rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
        };
    }
}
///
/// # Examples
///
/// ```
///  use arel::prelude::*;
/// use arel::{ActiveValue};
/// let a: ActiveValue<i32> = arel::Set(0);
/// let b = a + 1;
/// assert_eq!(b, ActiveValue::Changed(1, Box::new(ActiveValue::NotSet)))
/// ```
impl<V> Add<V> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Add<V, Output = V> + Default,
{
    type Output = Self;
    fn add(mut self, rhs: V) -> Self::Output {
        match &mut self {
            ActiveValue::Changed(nv, ov) => self = ActiveValue::Changed(*nv + rhs, ov.clone()),
            ActiveValue::Unchanged(v) => self = ActiveValue::Changed(*v + rhs, Box::new(self.clone())),
            ActiveValue::NotSet => self = ActiveValue::Changed(V::default() + rhs, Box::new(ActiveValue::NotSet)),
        };
        self
    }
}

impl<V> AddAssign<V> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Add<V, Output = V> + Default,
{
    fn add_assign(&mut self, rhs: V) {
        match self {
            ActiveValue::Changed(nv, ov) => *self = ActiveValue::Changed(*nv + rhs, ov.clone()),
            ActiveValue::Unchanged(v) => *self = ActiveValue::Changed(*v + rhs, Box::new(self.clone())),
            ActiveValue::NotSet => *self = ActiveValue::Changed(V::default() + rhs, Box::new(ActiveValue::NotSet)),
        };
    }
}
