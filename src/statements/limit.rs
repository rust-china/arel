use crate::statements::ArelStatement;

#[derive(Debug)]
pub struct Limit {
    num: usize,
}

impl ArelStatement for Limit {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        let mut final_sql = crate::Sql::new("LIMIT ");
        final_sql.push_str(self.num.to_string());
        Ok(Some(final_sql))
    }
}

impl Limit {
    pub fn new(num: usize) -> Self {
        Self { num }
    }
}
