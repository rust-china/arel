mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueDouble(pub Option<f64>);

impl Deref for ValueDouble {
    type Target = Option<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueDouble {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
