pub mod where_and;
pub mod where_or;

use std::marker::PhantomData;
pub use where_and::WhereAnd;
pub use where_or::WhereOr;

use crate::{prelude::ArelBase, statements::ArelStatement};

pub(crate) trait ArelSubWhereStatement {
    fn sqls(&self) -> Option<&Vec<crate::Sql>>;
    fn sqls_mut(&mut self) -> Option<&mut Vec<crate::Sql>>;
    fn join_str(&self) -> &'static str {
        "AND"
    }
    // 按值从小到大排序
    fn order(&self) -> i32 {
        0
    }
    fn to_sql(&self) -> Option<crate::Sql> {
        match self.sqls() {
            Some(sqls) => {
                let mut final_sql = crate::Sql::default();
                let len = sqls.len();
                if len > 0 {
                    for (idx, sql) in sqls.iter().enumerate() {
                        final_sql.push_sql(sql.clone());
                        if idx < len - 1 {
                            final_sql.push_str(" AND ");
                        }
                    }
                    Some(final_sql)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

pub struct Where<M: ArelBase> {
    sub_wheres: Vec<Box<dyn ArelSubWhereStatement>>,
    _mark: PhantomData<M>,
}

impl<T: ArelBase> ArelStatement for Where<T> {
    fn to_sql(&self) -> Option<crate::Sql> {
        let mut sub_wheres: Vec<&Box<dyn ArelSubWhereStatement>> = self.sub_wheres.iter().collect();
        if self.sub_wheres.len() > 0 {
            let mut final_sql = crate::Sql::new("WHERE ");
            sub_wheres.sort_by(|a, b| a.order().partial_cmp(&b.order()).unwrap());
            for (idx, sub_where) in sub_wheres.into_iter().enumerate() {
                let sql = sub_where.to_sql();
                if let Some(sql) = sql {
                    if idx >= 1 {
                        final_sql.push(' ').push_str(sub_where.join_str()).push(' ');
                    }
                    final_sql.push('(').push_sql(sql).push(')');
                }
            }
            Some(final_sql)
        } else {
            None
        }
    }
}

impl<M: ArelBase> Where<M> {
    pub fn new() -> Self {
        Self {
            sub_wheres: vec![],
            _mark: PhantomData,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::r#where::Where;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let mut r#where = Where::<User>::new();
    /// r#where.where_and("username", "sanmu");
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu")"#);
    ///
    /// r#where.where_and("age", vec![18, 20]);
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu") AND ("user"."age" IN (18,20))"#);
    ///
    /// ```
    pub fn where_and<K: ToString, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.where_and_pairs(vec![(key, value)])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::r#where::Where;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let mut r#where = Where::<User>::new();
    /// r#where.where_and_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu" AND "user"."age" IN (18,20))"#);
    ///
    /// r#where.where_and("gender", "male");
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu" AND "user"."age" IN (18,20)) AND ("user"."gender" = "male")"#);
    ///
    /// ```
    pub fn where_and_pairs<K: ToString, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut where_and = WhereAnd::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            match &value {
                crate::Value::Array(_) => {
                    sql.push_str(format!(r#""{}"."{}" IN "#, table_name, key.to_string()));
                    sql.push_sql(value.to_sql());
                }
                _ => {
                    sql.push_str(format!(r#""{}"."{}" = "#, table_name, key.to_string()));
                    sql.push_sql(value.to_sql());
                }
            }
            where_and.sqls.push(sql);
        }
        self.sub_wheres.push(Box::new(where_and));
        self
    }
    pub fn where_and_sql(&mut self, sql: crate::Sql) -> &mut Self {
        let mut where_and = WhereAnd::default();
        where_and.sqls.push(sql);
        self.sub_wheres.push(Box::new(where_and));
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::r#where::Where;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let mut r#where = Where::<User>::new();
    /// r#where.where_and_not("username", "sanmu");
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" != "sanmu")"#);
    ///
    /// r#where.where_and_not("aga", vec![18, 20]);
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" != "sanmu") AND ("user"."aga" NOT IN (18,20))"#);
    ///
    /// ```
    pub fn where_and_not<K: ToString, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.where_and_not_pairs(vec![(key, value)])
    }
    pub fn where_and_not_pairs<K: ToString, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut where_and = WhereAnd::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            match &value {
                crate::Value::Array(_) => {
                    sql.push_str(format!(r#""{}"."{}" NOT IN "#, table_name, key.to_string()));
                    sql.push_sql(value.to_sql());
                }
                _ => {
                    sql.push_str(format!(r#""{}"."{}" != "#, table_name, key.to_string()));
                    sql.push_sql(value.to_sql());
                }
            }
            where_and.sqls.push(sql);
        }
        self.sub_wheres.push(Box::new(where_and));
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::r#where::Where;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let mut r#where = Where::<User>::new();
    /// r#where.where_or("username", "sanmu");
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu")"#);
    ///
    /// r#where.where_or("age", vec![18, 20]);
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu") OR ("user"."age" IN (18,20))"#);
    ///
    /// r#where.where_and("gender", "male");
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."gender" = "male") OR ("user"."username" = "sanmu") OR ("user"."age" IN (18,20))"#);
    ///
    /// ```
    pub fn where_or<K: ToString, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.where_or_pairs(vec![(key, value)])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::r#where::Where;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let mut r#where = Where::<User>::new();
    /// r#where.where_or_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu" AND "user"."age" IN (18,20))"#);
    ///
    /// r#where.where_and("gender", "male");
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."gender" = "male") OR ("user"."username" = "sanmu" AND "user"."age" IN (18,20))"#);
    ///
    /// ```
    pub fn where_or_pairs<K: ToString, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut where_or = WhereOr::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            match &value {
                crate::Value::Array(_) => {
                    sql.push_str(format!(r#""{}"."{}" IN "#, table_name, key.to_string()));
                    sql.push_sql(value.to_sql());
                }
                _ => {
                    sql.push_str(format!(r#""{}"."{}" = "#, table_name, key.to_string()));
                    sql.push_sql(value.to_sql());
                }
            }
            where_or.sqls.push(sql);
        }
        self.sub_wheres.push(Box::new(where_or));
        self
    }
    pub fn where_or_sql(&mut self, sql: crate::Sql) -> &mut Self {
        let mut where_or = WhereOr::default();
        where_or.sqls.push(sql);
        self.sub_wheres.push(Box::new(where_or));
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::r#where::Where;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let mut r#where = Where::<User>::new();
    /// r#where.where_or_not("username", "sanmu");
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" != "sanmu")"#);
    ///
    /// r#where.where_or_not("aga", vec![18, 20]);
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" != "sanmu") OR ("user"."aga" NOT IN (18,20))"#);
    ///
    /// ```
    pub fn where_or_not<K: ToString, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.where_or_not_pairs(vec![(key, value)])
    }
    pub fn where_or_not_pairs<K: ToString, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut where_or = WhereOr::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            match &value {
                crate::Value::Array(_) => {
                    sql.push_str(format!(r#""{}"."{}" NOT IN "#, table_name, key.to_string()));
                    sql.push_sql(value.to_sql());
                }
                _ => {
                    sql.push_str(format!(r#""{}"."{}" != "#, table_name, key.to_string()));
                    sql.push_sql(value.to_sql());
                }
            }
            where_or.sqls.push(sql);
        }
        self.sub_wheres.push(Box::new(where_or));
        self
    }
    pub fn unwhere_starts_with<K: AsRef<str>>(&mut self, start: K) -> &mut Self {
        for sub_where in self.sub_wheres.iter_mut() {
            if let Some(sqls) = sub_where.sqls_mut() {
                sqls.retain(|sql| !sql.value.starts_with(start.as_ref()));
            }
        }
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::r#where::Where;
    /// struct User {}
    /// impl ArelBase for User {}
    /// let mut r#where = Where::<User>::new();
    /// r#where.where_and_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// r#where.unwhere("age");
    /// assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu")"#);
    ///
    /// ```
    pub fn unwhere<K: ToString>(&mut self, key: K) -> &mut Self {
        let table_name = M::table_name();
        let start_string = format!(r#""{}"."{}""#, table_name, key.to_string());
        self.unwhere_starts_with(start_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct User {}
    impl ArelBase for User {}

    #[test]
    fn it_works() {
        let mut r#where = Where::<User>::new();
        assert_eq!(r#where.to_sql(), None);

        let mut where_and = WhereAnd::default();
        where_and.sqls.push(crate::Sql::new_with_prepares("name = ?", vec!["sanmu"]));
        assert_eq!(where_and.to_sql().unwrap().to_sql_string().unwrap(), r#"name = "sanmu""#);
        where_and.sqls.push(crate::Sql::new_with_prepares("age = ?", vec![18]));
        assert_eq!(where_and.to_sql().unwrap().to_sql_string().unwrap(), r#"name = "sanmu" AND age = 18"#);
        r#where.sub_wheres.push(Box::new(where_and));
        assert!(r#where.to_sql().is_some());
        assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE (name = "sanmu" AND age = 18)"#);

        let mut where_or = WhereOr::default();
        where_or.sqls.push(crate::Sql::new_with_prepares("name = ?", vec!["sanmu"]));
        assert_eq!(where_or.to_sql().unwrap().to_sql_string().unwrap(), r#"name = "sanmu""#);
        where_or.sqls.push(crate::Sql::new_with_prepares("age = ?", vec![18]));
        assert_eq!(where_or.to_sql().unwrap().to_sql_string().unwrap(), r#"name = "sanmu" AND age = 18"#);
        r#where.sub_wheres.push(Box::new(where_or));
        assert!(r#where.to_sql().is_some());
        assert_eq!(
            r#where.to_sql().unwrap().to_sql_string().unwrap(),
            r#"WHERE (name = "sanmu" AND age = 18) OR (name = "sanmu" AND age = 18)"#
        );

        // bytes
        let mut where_and = WhereAnd::default();
        let bytes_value: crate::Value = bytes::Bytes::from_static(b"hello").into();
        where_and.sqls.push(crate::Sql::new_with_prepares("bytes = ?", vec![bytes_value]));
        assert_eq!(where_and.to_sql().unwrap().to_sql_string().unwrap(), r#"bytes = b"hello""#);

        // array
        let mut where_and = WhereAnd::default();
        let bytes: Vec<crate::Value> = vec![18.into(), 19.into(), 20.into()];
        let bytes_value: crate::Value = bytes.into();
        where_and.sqls.push(crate::Sql::new_with_prepares("age IN ?", vec![bytes_value]));
        assert_eq!(where_and.to_sql().unwrap().to_sql_string().unwrap(), r#"age IN (18,19,20)"#);
    }
}
