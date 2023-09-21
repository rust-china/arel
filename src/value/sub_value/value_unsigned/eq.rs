use super::ValueUnsigned;
use std::ops::Deref;

impl PartialEq<u32> for ValueUnsigned {
    fn eq(&self, other: &u32) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<u32>> for ValueUnsigned {
    fn eq(&self, other: &Option<u32>) -> bool {
        self.deref() == other
    }
}
