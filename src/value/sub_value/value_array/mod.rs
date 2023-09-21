mod eq;
mod from;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueArray(pub Option<Vec<crate::Value>>);

impl Deref for ValueArray {
    type Target = Option<Vec<crate::Value>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
