use super::ValueArray;
use std::ops::Deref;

impl PartialEq<Vec<crate::Value>> for ValueArray {
    fn eq(&self, other: &Vec<crate::Value>) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<Vec<crate::Value>>> for ValueArray {
    fn eq(&self, other: &Option<Vec<crate::Value>>) -> bool {
        self.deref() == other
    }
}
