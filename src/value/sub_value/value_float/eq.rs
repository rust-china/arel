use super::ValueFloat;
use std::ops::Deref;

impl PartialEq<f32> for ValueFloat {
    fn eq(&self, other: &f32) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<f32>> for ValueFloat {
    fn eq(&self, other: &Option<f32>) -> bool {
        self.deref() == other
    }
}
