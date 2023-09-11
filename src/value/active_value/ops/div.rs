use crate::{ActiveValue, Value};
use std::ops::{Div, DivAssign};

impl<V> Div<ActiveValue<V>> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Div<V, Output = V> + Default,
{
    type Output = Self;
    fn div(mut self, rhs: ActiveValue<V>) -> Self::Output {
        match &mut self {
            ActiveValue::Changed(nv, ov) => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(*nv / rnv, ov.clone()),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(*nv / rv, ov.clone()),
                ActiveValue::NotSet => (),
            },
            ActiveValue::Unchanged(v) => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(*v / rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(*v / rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
            ActiveValue::NotSet => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(V::default() / rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(V::default() / rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
        };
        self
    }
}
impl<V> DivAssign<ActiveValue<V>> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Div<V, Output = V> + Default,
{
    fn div_assign(&mut self, rhs: Self) {
        match self {
            ActiveValue::Changed(nv, ov) => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(*nv / rnv, ov.clone()),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(*nv / rv, ov.clone()),
                ActiveValue::NotSet => (),
            },
            ActiveValue::Unchanged(v) => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(*v / rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(*v / rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
            ActiveValue::NotSet => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(V::default() / rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(V::default() / rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
        };
    }
}
impl<V> Div<&ActiveValue<V>> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Div<V, Output = V> + Default,
{
    type Output = Self;
    fn div(mut self, rhs: &ActiveValue<V>) -> Self::Output {
        match &mut self {
            ActiveValue::Changed(nv, ov) => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(*nv / *rnv, ov.clone()),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(*nv / *rv, ov.clone()),
                ActiveValue::NotSet => (),
            },
            ActiveValue::Unchanged(v) => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(*v / *rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(*v / *rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
            ActiveValue::NotSet => match rhs {
                ActiveValue::Changed(rnv, _) => self = ActiveValue::Changed(V::default() / *rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => self = ActiveValue::Changed(V::default() / *rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
        };
        self
    }
}
impl<V> DivAssign<&ActiveValue<V>> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Div<V, Output = V> + Default,
{
    fn div_assign(&mut self, rhs: &Self) {
        match self {
            ActiveValue::Changed(nv, ov) => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(*nv / *rnv, ov.clone()),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(*nv / *rv, ov.clone()),
                ActiveValue::NotSet => (),
            },
            ActiveValue::Unchanged(v) => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(*v / *rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(*v / *rv, Box::new(self.clone())),
                ActiveValue::NotSet => (),
            },
            ActiveValue::NotSet => match rhs {
                ActiveValue::Changed(rnv, _) => *self = ActiveValue::Changed(V::default() / *rnv, Box::new(self.clone())),
                ActiveValue::Unchanged(rv) => *self = ActiveValue::Changed(V::default() / *rv, Box::new(self.clone())),
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
/// let a: ActiveValue<i32> = arel::Set(4);
/// let b = a / 2;
/// assert_eq!(b, ActiveValue::Changed(2, Box::new(ActiveValue::NotSet)))
/// ```
impl<V> Div<V> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Div<V, Output = V> + Default,
{
    type Output = Self;
    fn div(mut self, rhs: V) -> Self::Output {
        match &mut self {
            ActiveValue::Changed(nv, ov) => self = ActiveValue::Changed(*nv / rhs, ov.clone()),
            ActiveValue::Unchanged(v) => self = ActiveValue::Changed(*v / rhs, Box::new(self.clone())),
            ActiveValue::NotSet => self = ActiveValue::Changed(V::default() / rhs, Box::new(ActiveValue::NotSet)),
        };
        self
    }
}

impl<V> DivAssign<V> for ActiveValue<V>
where
    V: Into<Value> + Copy + PartialEq + Div<V, Output = V> + Default,
{
    fn div_assign(&mut self, rhs: V) {
        match self {
            ActiveValue::Changed(nv, ov) => *self = ActiveValue::Changed(*nv / rhs, ov.clone()),
            ActiveValue::Unchanged(v) => *self = ActiveValue::Changed(*v / rhs, Box::new(self.clone())),
            ActiveValue::NotSet => *self = ActiveValue::Changed(V::default() / rhs, Box::new(ActiveValue::NotSet)),
        };
    }
}
