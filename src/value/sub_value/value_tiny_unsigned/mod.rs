mod eq;
mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueTinyUnsigned(pub Option<u8>);

impl Deref for ValueTinyUnsigned {
    type Target = Option<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueTinyUnsigned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
