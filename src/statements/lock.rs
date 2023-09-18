use crate::statements::ArelStatement;

#[derive(Debug)]
pub struct Lock {
    value: String,
}

impl ArelStatement for Lock {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        Ok(Some(crate::Sql::new(&self.value)))
    }
}

impl Lock {
    pub fn new() -> Self {
        Self { value: "FOR UPDATE".to_string() }
    }
}
