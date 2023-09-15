mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueUnsigned(pub Option<u32>);

impl Deref for ValueUnsigned {
    type Target = Option<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueUnsigned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
