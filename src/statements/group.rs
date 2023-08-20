use crate::{prelude::ArelModel, statements::ArelStatement};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Group<M: ArelModel> {
    sqls: Vec<crate::Sql>,
    _marker: PhantomData<M>,
}

impl<M: ArelModel> ArelStatement for Group<M> {
    fn sqls(&self) -> Option<&Vec<crate::Sql>> {
        if self.sqls.len() > 0 {
            Some(&self.sqls)
        } else {
            None
        }
    }
    fn to_sql(&self) -> Option<crate::Sql> {
        if let Some(sqls) = self.sqls() {
            let mut final_sql = crate::Sql::new("GROUP BY ");
            for (idx, sql) in sqls.iter().enumerate() {
                if idx >= 1 {
                    final_sql.push_str(", ");
                }
                final_sql.push_sql(sql.clone());
            }
            Some(final_sql)
        } else {
            None
        }
    }
}

impl<M: ArelModel> Group<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::group::Group;
    /// struct User {}
    /// impl ArelBase for User {}
    /// impl ArelRecord for User {}
    /// impl ArelModel for User {}
    /// let group = Group::<User>::new(vec!["name", "age"]);
    /// assert_eq!(group.to_sql().unwrap().to_sql_string().unwrap(), r#"GROUP BY "user"."name", "user"."age""#);
    ///
    /// ```
    pub fn new<T: AsRef<str>>(columns: Vec<T>) -> Self {
        let table_name = M::table_name();
        Self {
            sqls: columns.iter().map(|column| crate::Sql::new(format!(r#""{}"."{}""#, table_name, column.as_ref()))).collect(),
            _marker: PhantomData::<M>,
        }
    }
    pub fn new_sql<S: Into<crate::Sql>>(sql: S) -> Self {
        Self {
            sqls: vec![sql.into()],
            _marker: PhantomData::<M>,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::group::Group;
    /// struct User {}
    /// impl ArelBase for User {}
    /// impl ArelRecord for User {}
    /// impl ArelModel for User {}
    /// let group = Group::<User>::new_sqls(vec!["name", "age"]);
    /// assert_eq!(group.to_sql().unwrap().to_sql_string().unwrap(), r#"GROUP BY name, age"#);
    ///
    /// ```
    pub fn new_sqls<S: Into<crate::Sql>>(sqls: Vec<S>) -> Self {
        Self {
            sqls: sqls.into_iter().map(|sql| sql.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
}
