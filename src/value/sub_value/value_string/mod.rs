mod from;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueString(pub Option<String>);

impl Deref for ValueString {
    type Target = Option<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
