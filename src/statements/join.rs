use crate::{prelude::ArelModel, statements::ArelStatement};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Join<M: ArelModel> {
    sqls: Vec<crate::Sql>,
    _marker: PhantomData<M>,
}

impl<M: ArelModel> ArelStatement for Join<M> {
    fn sqls(&self) -> Option<&Vec<crate::Sql>> {
        if self.sqls.len() > 0 {
            Some(&self.sqls)
        } else {
            None
        }
    }
    fn to_sql(&self) -> Option<crate::Sql> {
        if let Some(sqls) = self.sqls() {
            let mut final_sql = crate::Sql::new("");
            for (idx, sql) in sqls.iter().enumerate() {
                if idx >= 1 {
                    final_sql.push_str(" ");
                }
                final_sql.push_sql(sql.clone());
            }
            Some(final_sql)
        } else {
            None
        }
    }
}

impl<M: ArelModel> Join<M> {
    pub fn new() -> Self {
        Self {
            sqls: vec![],
            _marker: PhantomData::<M>,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::join::Join;
    /// struct User {}
    /// impl ArelBase for User {}
    /// impl ArelRecord for User {}
    /// impl ArelModel for User {}
    /// struct Wallet {}
    /// impl ArelBase for Wallet {}
    /// impl ArelRecord for Wallet {}
    /// impl ArelModel for Wallet {}
    /// let mut join = Join::<User>::new();
    /// join.join::<Wallet>(arel::JoinType::InnerJoin);
    /// assert_eq!(join.to_sql().unwrap().to_sql_string().unwrap(), r#"INNER JOIN "wallet" ON "user"."id" = "wallet"."user_id""#);
    ///
    /// use std::borrow::Cow;
    /// struct Admin {}
    /// impl ArelBase for Admin {}
    /// impl ArelRecord for Admin {}
    /// impl ArelModel for Admin {
    ///     fn primary_keys() -> Option<Vec<Cow<'static, str>>> where Self: Sized {
    ///         Some(vec!["id".into(), "uuid".into()])
    ///     }
    /// }
    /// let mut join = Join::<Admin>::new();
    /// join.join::<Wallet>(arel::JoinType::InnerJoin);
    /// assert_eq!(join.to_sql().unwrap().to_sql_string().unwrap(), r#"INNER JOIN "wallet" ON "admin"."id" = "wallet"."admin_id" AND "admin"."uuid" = "wallet"."admin_uuid""#);
    ///
    /// ```
    pub fn join<U: ArelModel>(&mut self, join_type: crate::JoinType) -> &mut Self {
        let m_table_name = M::table_name();
        let u_table_name = U::table_name();
        if let Some(m_primary_keys) = M::primary_keys() {
            let mut sql = crate::Sql::new(format!(r#"{} "{}" ON "#, join_type.to_string(), u_table_name));
            for (idx, m_primary_key) in m_primary_keys.iter().enumerate() {
                if idx >= 1 {
                    sql.push_str(" AND ");
                }
                let u_m_foreign_key = format!("{}_{}", m_table_name, m_primary_key);
                sql.push_str(format!(r#""{}"."{}" = "{}"."{}""#, m_table_name, m_primary_key, u_table_name, u_m_foreign_key));
            }
            self.sqls.push(sql);
        } else if let Some(m_primary_key) = M::primary_key() {
            let u_m_foreign_key = format!("{}_{}", m_table_name, m_primary_key);
            let sql = crate::Sql::new(format!(
                r#"INNER JOIN "{}" ON "{}"."{}" = "{}"."{}""#,
                u_table_name, m_table_name, m_primary_key, u_table_name, u_m_foreign_key
            ));
            self.sqls.push(sql);
        }
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::join::Join;
    /// struct User {}
    /// impl ArelBase for User {}
    /// impl ArelRecord for User {}
    /// impl ArelModel for User {}
    /// let mut join = Join::<User>::new();
    /// join.join_sql("LEFT JOIN wallet ON user.id = wallet.user_id");
    /// join.join_sql("INNER JOIN order ON user.id = order.user_id");
    /// assert_eq!(join.to_sql().unwrap().to_sql_string().unwrap(), r#"LEFT JOIN wallet ON user.id = wallet.user_id INNER JOIN order ON user.id = order.user_id"#);
    ///
    /// ```
    pub fn join_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        self.sqls.push(sql.into());
        self
    }
}
