use crate::statements::r#where::ArelSubWhereStatement;
// use std::ops::{Bound, RangeBounds};

#[derive(Clone, Debug, Default)]
pub struct WhereAnd {
    pub(crate) sqls: Vec<crate::Sql>,
}

impl ArelSubWhereStatement for WhereAnd {
    fn sqls(&self) -> Option<&Vec<crate::Sql>> {
        Some(&self.sqls)
    }
    fn sqls_mut(&mut self) -> Option<&mut Vec<crate::Sql>> {
        Some(&mut self.sqls)
    }
}
