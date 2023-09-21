mod eq;
mod from;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

// #[allow(clippy::box_collection)]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueBytes(pub Option<bytes::Bytes>);

impl Deref for ValueBytes {
    type Target = Option<bytes::Bytes>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ValueBytes {
    pub fn debug_string(&self) -> String {
        match self.deref() {
            Some(v) => format!("Some({})", v.escape_ascii().to_string()),
            None => "None".to_string(),
        }
    }
}
