mod eq;
mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueBool(pub Option<bool>);

impl Deref for ValueBool {
    type Target = Option<bool>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
