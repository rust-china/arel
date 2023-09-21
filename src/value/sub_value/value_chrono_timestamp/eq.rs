use super::ValueChronoTimestamp;
use std::ops::Deref;

impl PartialEq<chrono::DateTime<chrono::FixedOffset>> for ValueChronoTimestamp {
    fn eq(&self, other: &chrono::DateTime<chrono::FixedOffset>) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<chrono::DateTime<chrono::FixedOffset>>> for ValueChronoTimestamp {
    fn eq(&self, other: &Option<chrono::DateTime<chrono::FixedOffset>>) -> bool {
        self.deref() == other
    }
}
