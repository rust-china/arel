use crate::{prelude::Arel, statements::ArelStatement};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Group<M: Arel> {
    sqls: Vec<crate::Sql>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Group<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        if self.sqls.len() > 0 {
            let mut final_sql = crate::Sql::new("GROUP BY ");
            final_sql.push_sqls(self.sqls.clone(), ", ");
            Ok(Some(final_sql))
        } else {
            Ok(None)
        }
    }
}

impl<M: Arel> Default for Group<M> {
    fn default() -> Self {
        Self {
            sqls: vec![],
            _marker: PhantomData::<M>,
        }
    }
}

impl<M: Arel> Group<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::group::Group;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let group = Group::<User>::new(vec!["name", "age"]);
    /// assert_eq!(group.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"GROUP BY "user"."name", "user"."age""#);
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
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let group = Group::<User>::new_sqls(vec!["name", "age"]);
    /// assert_eq!(group.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"GROUP BY name, age"#);
    ///
    /// ```
    pub fn new_sqls<S: Into<crate::Sql>>(sqls: Vec<S>) -> Self {
        Self {
            sqls: sqls.into_iter().map(|sql| sql.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
}
