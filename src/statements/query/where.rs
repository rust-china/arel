use crate::statements::ArelStatement;
// use std::ops::{Bound, RangeBounds};

#[derive(Clone, Debug)]
pub struct Where {
    sqls: Vec<crate::Sql>,
}

impl Default for Where {
    fn default() -> Self {
        Self { sqls: vec![] }
    }
}

impl ArelStatement for Where {
    fn sqls(&self) -> &Vec<crate::Sql> {
        &self.sqls
    }
}
