use super::ValueTinyInt;
use std::ops::Deref;

impl PartialEq<i8> for ValueTinyInt {
    fn eq(&self, other: &i8) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<i8>> for ValueTinyInt {
    fn eq(&self, other: &Option<i8>) -> bool {
        self.deref() == other
    }
}
