use crate::{prelude::Arel, statements::ArelStatement};
use std::marker::PhantomData;

pub enum SortConst {
    Asc,
    Desc,
}
impl std::fmt::Display for SortConst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SortConst::Asc => write!(f, "ASC"),
            SortConst::Desc => write!(f, "DESC"),
        }
    }
}

#[derive(Debug)]
pub struct Order<M: Arel> {
    sqls: Vec<crate::Sql>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Order<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        if self.sqls.len() > 0 {
            let mut final_sql = crate::Sql::new("ORDER BY ");
            final_sql.push_sqls(self.sqls.clone(), ", ");
            Ok(Some(final_sql))
        } else {
            Ok(None)
        }
    }
}

impl<M: Arel> Default for Order<M> {
    fn default() -> Self {
        Self {
            sqls: vec![],
            _marker: PhantomData::<M>,
        }
    }
}

impl<M: Arel> Order<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::order::Order;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let order = Order::<User>::new("name", arel::SortConst::Desc);
    /// assert_eq!(order.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"ORDER BY "user"."name" DESC"#);
    ///
    /// ```
    pub fn new<T: AsRef<str>>(column: T, sort_type: SortConst) -> Self {
        Self::new_columns(vec![(column, sort_type)])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::order::Order;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let order = Order::<User>::new_columns(vec![("name", arel::SortConst::Desc), ("age", arel::SortConst::Asc)]);
    /// assert_eq!(order.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"ORDER BY "user"."name" DESC, "user"."age" ASC"#);
    ///
    /// ```
    pub fn new_columns<T: AsRef<str>>(columns: Vec<(T, SortConst)>) -> Self {
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
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let order = Order::<User>::new_sqls(vec!["name DESC", "age ASC"]);
    /// assert_eq!(order.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"ORDER BY name DESC, age ASC"#);
    ///
    /// ```
    pub fn new_sqls<S: Into<crate::Sql>>(sqls: Vec<S>) -> Self {
        Self {
            sqls: sqls.into_iter().map(|sql| sql.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
    pub fn append<T: AsRef<str>>(&mut self, column: T, sort_type: SortConst) -> &mut Self {
        let table_name = M::table_name();
        self.sqls.push(crate::Sql::new(format!(r#""{}"."{}" {}"#, table_name, column.as_ref(), sort_type.to_string())));
        self
    }
    pub fn append_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        self.sqls.push(sql.into());
        self
    }
}
