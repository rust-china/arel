use crate::statements::ArelStatement;

#[derive(Debug)]
pub struct Offset {
    num: usize,
}

impl ArelStatement for Offset {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        let mut final_sql = crate::Sql::new("OFFSET ");
        final_sql.push_str(self.num.to_string());
        Ok(Some(final_sql))
    }
}

impl Offset {
    pub fn new(num: usize) -> Self {
        Self { num }
    }
}
