use super::ValueChronoDate;
use std::ops::Deref;

impl PartialEq<chrono::NaiveDate> for ValueChronoDate {
    fn eq(&self, other: &chrono::NaiveDate) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<chrono::NaiveDate>> for ValueChronoDate {
    fn eq(&self, other: &Option<chrono::NaiveDate>) -> bool {
        self.deref() == other
    }
}
