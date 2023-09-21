mod eq;
mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueSmallUnsigned(pub Option<u16>);

impl Deref for ValueSmallUnsigned {
    type Target = Option<u16>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueSmallUnsigned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
