mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueInt(pub Option<i32>);

impl Deref for ValueInt {
    type Target = Option<i32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
