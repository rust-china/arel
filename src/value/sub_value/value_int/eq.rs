use super::ValueInt;
use std::ops::Deref;

impl PartialEq<i32> for ValueInt {
    fn eq(&self, other: &i32) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<i32>> for ValueInt {
    fn eq(&self, other: &Option<i32>) -> bool {
        self.deref() == other
    }
}
