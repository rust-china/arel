use super::ValueBigInt;
use std::ops::Deref;

impl PartialEq<i64> for ValueBigInt {
    fn eq(&self, other: &i64) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<i64>> for ValueBigInt {
    fn eq(&self, other: &Option<i64>) -> bool {
        self.deref() == other
    }
}
