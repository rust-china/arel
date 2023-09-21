mod eq;
mod from;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "with-chrono")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueChronoTime(pub Option<chrono::NaiveTime>);

impl Deref for ValueChronoTime {
    type Target = Option<chrono::NaiveTime>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueChronoTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
