mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueTinyInt(pub Option<i8>);

impl Deref for ValueTinyInt {
    type Target = Option<i8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueTinyInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
