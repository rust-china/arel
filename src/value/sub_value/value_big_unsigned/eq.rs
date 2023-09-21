use super::ValueBigUnsigned;
use std::ops::Deref;

impl PartialEq<u64> for ValueBigUnsigned {
    fn eq(&self, other: &u64) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<u64>> for ValueBigUnsigned {
    fn eq(&self, other: &Option<u64>) -> bool {
        self.deref() == other
    }
}
