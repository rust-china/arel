use super::ValueSmallUnsigned;
use std::ops::Deref;

impl PartialEq<u16> for ValueSmallUnsigned {
    fn eq(&self, other: &u16) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<u16>> for ValueSmallUnsigned {
    fn eq(&self, other: &Option<u16>) -> bool {
        self.deref() == other
    }
}
