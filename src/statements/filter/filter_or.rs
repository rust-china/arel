use crate::statements::filter::ArelSubFilterStatement;
// use std::ops::{Bound, RangeBounds};

#[derive(Clone, Debug, Default)]
pub struct FilterOr {
    pub(crate) sqls: Vec<crate::Sql>,
}

impl ArelSubFilterStatement for FilterOr {
    fn sqls(&self) -> Option<&Vec<crate::Sql>> {
        Some(&self.sqls)
    }
    fn sqls_mut(&mut self) -> Option<&mut Vec<crate::Sql>> {
        Some(&mut self.sqls)
    }
    fn join_str(&self) -> &'static str {
        " OR "
    }
    // fn order(&self) -> i32 {
    //     1
    // }
}
