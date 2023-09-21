mod eq;
mod from;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueChronoTimestamp(pub Option<chrono::DateTime<chrono::FixedOffset>>);

impl Deref for ValueChronoTimestamp {
    type Target = Option<chrono::DateTime<chrono::FixedOffset>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueChronoTimestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
