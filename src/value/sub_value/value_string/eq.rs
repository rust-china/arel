use super::ValueString;
use std::ops::Deref;

impl PartialEq<str> for ValueString {
    fn eq(&self, other: &str) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}

impl PartialEq<String> for ValueString {
    fn eq(&self, other: &String) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<String>> for ValueString {
    fn eq(&self, other: &Option<String>) -> bool {
        self.deref() == other
    }
}

impl<T> PartialEq<&T> for ValueString
where
    T: PartialEq<ValueString>,
{
    fn eq(&self, other: &&T) -> bool {
        *other == self
    }
}
