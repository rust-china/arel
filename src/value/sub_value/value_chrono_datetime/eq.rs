use super::ValueChronoDateTime;
use std::ops::Deref;

impl PartialEq<chrono::NaiveDateTime> for ValueChronoDateTime {
    fn eq(&self, other: &chrono::NaiveDateTime) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<chrono::NaiveDateTime>> for ValueChronoDateTime {
    fn eq(&self, other: &Option<chrono::NaiveDateTime>) -> bool {
        self.deref() == other
    }
}
