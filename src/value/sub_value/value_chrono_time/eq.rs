use super::ValueChronoTime;
use std::ops::Deref;

impl PartialEq<chrono::NaiveTime> for ValueChronoTime {
    fn eq(&self, other: &chrono::NaiveTime) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<chrono::NaiveTime>> for ValueChronoTime {
    fn eq(&self, other: &Option<chrono::NaiveTime>) -> bool {
        self.deref() == other
    }
}
