use crate::{statements::ArelStatement, Arel};
use std::marker::PhantomData;

pub enum JoinConst {
    LeftJoin,
    InnerJoin,
    RightJoin,
    FullOuterJoin,
    CrossJoin,
}
impl std::fmt::Display for JoinConst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JoinConst::LeftJoin => write!(f, "LEFT JOIN"),
            JoinConst::InnerJoin => write!(f, "INNER JOIN"),
            JoinConst::RightJoin => write!(f, "RIGHT JOIN"),
            JoinConst::FullOuterJoin => write!(f, "FULL OUTER JOIN"),
            JoinConst::CrossJoin => write!(f, "CROSS JOIN"),
        }
    }
}

#[derive(Debug)]
pub struct Join<M: Arel> {
    sqls: Vec<crate::Sql>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Join<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        if self.sqls.len() > 0 {
            let mut final_sql = crate::Sql::new("");
            final_sql.push_sqls(self.sqls.clone(), " ");
            Ok(Some(final_sql))
        } else {
            Ok(None)
        }
    }
}

impl<M: Arel> Default for Join<M> {
    fn default() -> Self {
        Self {
            sqls: vec![],
            _marker: PhantomData::<M>,
        }
    }
}

impl<M: Arel> Join<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::join::Join;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// #[arel]
    /// struct Wallet {}
    /// impl Arel for Wallet {}
    /// let mut join = Join::<User>::default();
    /// join.join::<Wallet>(arel::JoinConst::InnerJoin);
    /// assert_eq!(join.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"INNER JOIN "wallet" ON "user"."id" = "wallet"."user_id""#);
    ///
    /// #[arel]
    /// struct Admin {
    /// 	#[arel(primary_key)]
    /// 	id: i32,
    /// 	#[arel(primary_key)]
    /// 	uuid: String,
    /// }
    /// impl Arel for Admin {}
    /// let mut join = Join::<Admin>::default();
    /// join.join::<Wallet>(arel::JoinConst::InnerJoin);
    /// assert_eq!(join.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"INNER JOIN "wallet" ON "admin"."id" = "wallet"."admin_id" AND "admin"."uuid" = "wallet"."admin_uuid""#);
    ///
    /// ```
    pub fn join<U: Arel>(&mut self, join_type: JoinConst) -> &mut Self {
        let m_table_name = M::table_name();
        let u_table_name = U::table_name();
        let m_primary_keys = M::primary_keys();
        let mut sql = crate::Sql::new(format!(r#"{} "{}" ON "#, join_type.to_string(), u_table_name));
        for (idx, m_primary_key) in m_primary_keys.iter().enumerate() {
            if idx >= 1 {
                sql.push_str(" AND ");
            }
            let u_m_foreign_key = format!("{}_{}", m_table_name, m_primary_key);
            sql.push_str(format!(r#""{}"."{}" = "{}"."{}""#, m_table_name, m_primary_key, u_table_name, u_m_foreign_key));
        }
        self.sqls.push(sql);
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::join::Join;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut join = Join::<User>::default();
    /// join.join_sql("LEFT JOIN wallet ON user.id = wallet.user_id");
    /// join.join_sql("INNER JOIN order ON user.id = order.user_id");
    /// assert_eq!(join.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"LEFT JOIN wallet ON user.id = wallet.user_id INNER JOIN order ON user.id = order.user_id"#);
    ///
    /// ```
    pub fn join_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        self.sqls.push(sql.into());
        self
    }
}
