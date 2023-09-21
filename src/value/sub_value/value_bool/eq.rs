use super::ValueBool;
use std::ops::Deref;

impl PartialEq<bool> for ValueBool {
    fn eq(&self, other: &bool) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<bool>> for ValueBool {
    fn eq(&self, other: &Option<bool>) -> bool {
        self.deref() == other
    }
}
