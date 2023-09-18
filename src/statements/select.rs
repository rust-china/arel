use crate::{statements::ArelStatement, Arel};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Select<M: Arel> {
    distinct: bool,
    sqls: Vec<crate::Sql>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Select<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        let mut final_sql = crate::Sql::new("SELECT ");
        if self.distinct {
            final_sql.push_str("DISTINCT ");
        }

        if self.sqls.len() > 0 {
            final_sql.push_sqls(self.sqls.clone(), ", ");
        } else {
            final_sql.push_str(format!(r#""{}".*"#, M::table_name()));
        }
        final_sql.push_str(format!(r#" FROM "{}""#, M::table_name()));
        Ok(Some(final_sql))
    }
}

impl<M: Arel> Default for Select<M> {
    fn default() -> Self {
        Self {
            distinct: false,
            sqls: vec![],
            _marker: PhantomData::<M>,
        }
    }
}

impl<M: Arel> Select<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::Select;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let select = Select::<User>::new(vec!["name", "age"]);
    /// assert_eq!(select.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"SELECT "user"."name", "user"."age" FROM "user""#);
    ///
    /// ```
    pub fn new<T: AsRef<str>>(columns: Vec<T>) -> Self {
        let table_name = M::table_name();
        Self {
            distinct: false,
            sqls: columns.iter().map(|column| crate::Sql::new(format!(r#""{}"."{}""#, table_name, column.as_ref()))).collect(),
            _marker: PhantomData::<M>,
        }
    }
    pub fn new_sql<S: Into<crate::Sql>>(sql: S) -> Self {
        Self {
            distinct: false,
            sqls: vec![sql.into()],
            _marker: PhantomData::<M>,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::select::Select;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let select = Select::<User>::new_sqls(vec!["name", "age"]);
    /// assert_eq!(select.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"SELECT name, age FROM "user""#);
    ///
    /// ```
    pub fn new_sqls<S: Into<crate::Sql>>(sqls: Vec<S>) -> Self {
        Self {
            distinct: false,
            sqls: sqls.into_iter().map(|sql| sql.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = true;
        self
    }
}
