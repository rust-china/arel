use crate::{prelude::ArelBase, statements::ArelStatement};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Order<M: ArelBase> {
    sqls: Vec<crate::Sql>,
    _marker: PhantomData<M>,
}

impl<M: ArelBase> ArelStatement for Order<M> {
    fn sqls(&self) -> Option<&Vec<crate::Sql>> {
        if self.sqls.len() > 0 {
            Some(&self.sqls)
        } else {
            None
        }
    }
    fn to_sql(&self) -> Option<crate::Sql> {
        if let Some(sqls) = self.sqls() {
            let mut final_sql = crate::Sql::new("ORDER BY ");
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

impl<M: ArelBase> Order<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::order::Order;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let order = Order::<User>::new("name", arel::SortType::Desc);
    /// assert_eq!(order.to_sql().unwrap().to_sql_string().unwrap(), r#"ORDER BY "user"."name" DESC"#);
    ///
    /// ```
    pub fn new<T: AsRef<str>>(column: T, sort_type: crate::SortType) -> Self {
        Self::new_columns(vec![(column, sort_type)])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::order::Order;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let order = Order::<User>::new_columns(vec![("name", arel::SortType::Desc), ("age", arel::SortType::Asc)]);
    /// assert_eq!(order.to_sql().unwrap().to_sql_string().unwrap(), r#"ORDER BY "user"."name" DESC, "user"."age" ASC"#);
    ///
    /// ```
    pub fn new_columns<T: AsRef<str>>(columns: Vec<(T, crate::SortType)>) -> Self {
        let table_name = M::table_name();
        Self {
            sqls: columns
                .iter()
                .map(|column| crate::Sql::new(format!(r#""{}"."{}" {}"#, table_name, column.0.as_ref(), column.1.to_string())))
                .collect(),
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
    /// use arel::statements::order::Order;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let order = Order::<User>::new_sqls(vec!["name DESC", "age ASC"]);
    /// assert_eq!(order.to_sql().unwrap().to_sql_string().unwrap(), r#"ORDER BY name DESC, age ASC"#);
    ///
    /// ```
    pub fn new_sqls<S: Into<crate::Sql>>(sqls: Vec<S>) -> Self {
        Self {
            sqls: sqls.into_iter().map(|sql| sql.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
    pub fn append<T: AsRef<str>>(&mut self, column: T, sort_type: crate::SortType) -> &mut Self {
        let table_name = M::table_name();
        self.sqls.push(crate::Sql::new(format!(r#""{}"."{}" {}"#, table_name, column.as_ref(), sort_type.to_string())));
        self
    }
    pub fn append_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        self.sqls.push(sql.into());
        self
    }
}
