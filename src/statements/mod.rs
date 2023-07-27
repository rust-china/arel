pub mod query;

trait ArelStatement {
    fn sqls(&self) -> &Vec<crate::Sql>;
    fn to_sql(&self) -> crate::Sql {
        let mut final_sql = crate::Sql::default();
        for sql in self.sqls().iter() {
            final_sql.push_sql(sql.clone());
        }
        final_sql
    }
}
