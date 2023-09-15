mod from;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueJson(pub Option<serde_json::Value>);

impl Deref for ValueJson {
    type Target = Option<serde_json::Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueJson {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
