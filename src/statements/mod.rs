pub mod group;
pub mod limit;
pub mod offset;
pub mod r#where;

pub trait ArelStatement {
    fn sqls(&self) -> Option<&Vec<crate::Sql>> {
        None
    }
    fn to_sql(&self) -> Option<crate::Sql> {
        match self.sqls() {
            Some(sqls) => {
                let mut final_sql = crate::Sql::default();
                for sql in sqls.iter() {
                    final_sql.push_sql(sql.clone());
                }
                Some(final_sql)
            }
            None => None,
        }
    }
}
