mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueBigInt(pub Option<i64>);

impl Deref for ValueBigInt {
    type Target = Option<i64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueBigInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
