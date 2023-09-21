use super::ValueTinyUnsigned;
use std::ops::Deref;

impl PartialEq<u8> for ValueTinyUnsigned {
    fn eq(&self, other: &u8) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<u8>> for ValueTinyUnsigned {
    fn eq(&self, other: &Option<u8>) -> bool {
        self.deref() == other
    }
}
