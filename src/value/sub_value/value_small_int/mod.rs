mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueSmallInt(pub Option<i16>);

impl Deref for ValueSmallInt {
    type Target = Option<i16>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueSmallInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
