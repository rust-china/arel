use crate::{prelude::ArelBase, statements::ArelStatement};
use std::marker::PhantomData;

pub struct Group<M: ArelBase> {
    sqls: Vec<crate::Sql>,
    _mark: PhantomData<M>,
}

impl<M: ArelBase> ArelStatement for Group<M> {
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

impl<M: ArelBase> Group<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::group::Group;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let group = Group::<User>::new_column("name");
    /// assert_eq!(group.to_sql().unwrap().to_sql_string().unwrap(), r#"GROUP BY "user"."name""#);
    ///
    /// ```
    pub fn new_column<T: AsRef<str>>(column: T) -> Self {
        Self::new_columns(vec![column])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::group::Group;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let group = Group::<User>::new_columns(vec!["name", "age"]);
    /// assert_eq!(group.to_sql().unwrap().to_sql_string().unwrap(), r#"GROUP BY "user"."name", "user"."age""#);
    ///
    /// ```
    pub fn new_columns<T: AsRef<str>>(columns: Vec<T>) -> Self {
        let table_name = M::table_name();
        Self {
            sqls: columns.iter().map(|column| crate::Sql::new(format!(r#""{}"."{}""#, table_name, column.as_ref()))).collect(),
            _mark: PhantomData::<M>,
        }
    }
    pub fn new_sql<S: Into<crate::Sql>>(sql: S) -> Self {
        Self {
            sqls: vec![sql.into()],
            _mark: PhantomData::<M>,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::group::Group;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let group = Group::<User>::new_sqls(vec!["name", "age"]);
    /// assert_eq!(group.to_sql().unwrap().to_sql_string().unwrap(), r#"GROUP BY name, age"#);
    ///
    /// ```
    pub fn new_sqls<S: Into<crate::Sql>>(sqls: Vec<S>) -> Self {
        Self {
            sqls: sqls.into_iter().map(|sql| sql.into()).collect(),
            _mark: PhantomData::<M>,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::prelude::ArelBase;

//     #[test]
//     fn to_sql() {
//         struct User {}
//         impl ArelBase for User {}

//         // let group = Group::<User>::new("name");
//         // assert_eq!(group.to_sql().unwrap().to_sql_string().unwrap(), r#"GROUP "user"."name""#);
//     }
// }
