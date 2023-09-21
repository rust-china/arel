mod eq;
mod from;
mod ops;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueFloat(pub Option<f32>);

impl Deref for ValueFloat {
    type Target = Option<f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
