use super::ValueBytes;
use std::ops::Deref;

impl PartialEq<bytes::Bytes> for ValueBytes {
    fn eq(&self, other: &bytes::Bytes) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<bytes::Bytes>> for ValueBytes {
    fn eq(&self, other: &Option<bytes::Bytes>) -> bool {
        self.deref() == other
    }
}
