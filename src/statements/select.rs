use crate::{prelude::ArelBase, statements::ArelStatement};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Select<M: ArelBase> {
    distinct: bool,
    sqls: Vec<crate::Sql>,
    _marker: PhantomData<M>,
}

impl<M: ArelBase> ArelStatement for Select<M> {
    fn sqls(&self) -> Option<&Vec<crate::Sql>> {
        if self.sqls.len() > 0 {
            Some(&self.sqls)
        } else {
            None
        }
    }
    fn to_sql(&self) -> Option<crate::Sql> {
        let mut final_sql = crate::Sql::new("SELECT ");
        if self.distinct {
            final_sql.push_str("DISTINCT ");
        }

        if let Some(sqls) = self.sqls() {
            for (idx, sql) in sqls.iter().enumerate() {
                if idx >= 1 {
                    final_sql.push_str(", ");
                }
                final_sql.push_sql(sql.clone());
            }
        } else {
            final_sql.push_str(format!(r#""{}".*"#, M::table_name()));
        }
        final_sql.push_str(format!(r#" FROM "{}""#, M::table_name()));
        Some(final_sql)
    }
}

impl<M: ArelBase> Default for Select<M> {
    fn default() -> Self {
        Self {
            distinct: false,
            sqls: vec![],
            _marker: PhantomData::<M>,
        }
    }
}

impl<M: ArelBase> Select<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::select::Select;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let select = Select::<User>::new(vec!["name", "age"]);
    /// assert_eq!(select.to_sql().unwrap().to_sql_string().unwrap(), r#"SELECT "user"."name", "user"."age" FROM "user""#);
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
    /// struct User {}
    /// impl ArelBase for User {}
    /// let select = Select::<User>::new_sqls(vec!["name", "age"]);
    /// assert_eq!(select.to_sql().unwrap().to_sql_string().unwrap(), r#"SELECT name, age FROM "user""#);
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
