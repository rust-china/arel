use super::ArelSubFilterStatement;

#[derive(Debug, Default)]
pub struct OrFilter {
    pub(crate) sqls: Vec<crate::Sql>,
}

impl ArelSubFilterStatement for OrFilter {
    fn sqls(&self) -> &Vec<crate::Sql> {
        &self.sqls
    }
    fn sqls_mut(&mut self) -> &mut Vec<crate::Sql> {
        &mut self.sqls
    }
    fn join_str(&self) -> &'static str {
        " OR "
    }
}
