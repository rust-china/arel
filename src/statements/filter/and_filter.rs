use super::ArelSubFilterStatement;

#[derive(Debug, Default)]
pub struct AndFilter {
    pub(crate) sqls: Vec<crate::Sql>,
}

impl ArelSubFilterStatement for AndFilter {
    fn sqls(&self) -> &Vec<crate::Sql> {
        &self.sqls
    }
    fn sqls_mut(&mut self) -> &mut Vec<crate::Sql> {
        &mut self.sqls
    }
    fn join_str(&self) -> &'static str {
        " AND "
    }
}
