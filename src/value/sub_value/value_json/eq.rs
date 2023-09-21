use super::ValueJson;
use std::ops::Deref;

impl PartialEq<serde_json::Value> for ValueJson {
    fn eq(&self, other: &serde_json::Value) -> bool {
        match self.deref() {
            Some(v) => v == other,
            None => false,
        }
    }
}
impl PartialEq<Option<serde_json::Value>> for ValueJson {
    fn eq(&self, other: &Option<serde_json::Value>) -> bool {
        self.deref() == other
    }
}
