mod eq;
mod from;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "with-chrono")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueChronoDate(pub Option<chrono::NaiveDate>);

impl Deref for ValueChronoDate {
    type Target = Option<chrono::NaiveDate>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValueChronoDate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
