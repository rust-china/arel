use super::ValueDouble;
use std::ops::Deref;

impl PartialEq<f64> for ValueDouble {
    fn eq(&self, other: &f64) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<f64>> for ValueDouble {
    fn eq(&self, other: &Option<f64>) -> bool {
        self.deref() == other
    }
}
