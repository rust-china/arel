mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueBigUnsigned(pub Option<u64>);

impl Deref for ValueBigUnsigned {
    type Target = Option<u64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueBigUnsigned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
