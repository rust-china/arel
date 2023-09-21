mod eq;
mod from;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "with-chrono")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueChronoDateTime(pub Option<chrono::NaiveDateTime>);

impl Deref for ValueChronoDateTime {
    type Target = Option<chrono::NaiveDateTime>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueChronoDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
