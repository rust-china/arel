use super::ValueSmallInt;
use std::ops::Deref;

impl PartialEq<i16> for ValueSmallInt {
    fn eq(&self, other: &i16) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<i16>> for ValueSmallInt {
    fn eq(&self, other: &Option<i16>) -> bool {
        self.deref() == other
    }
}
