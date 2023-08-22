pub mod filter_and;
pub mod filter_or;

pub use filter_and::FilterAnd;
pub use filter_or::FilterOr;
use std::{fmt::Debug, marker::PhantomData};

use super::ArelStatement;
use crate::prelude::Arel;

pub(crate) trait ArelSubFilterStatement: Debug {
    fn sqls(&self) -> Option<&Vec<crate::Sql>>;
    fn sqls_mut(&mut self) -> Option<&mut Vec<crate::Sql>>;
    fn join_str(&self) -> &'static str;
    // 按值从小到大排序
    // fn order(&self) -> i32 {
    //     0
    // }
    fn to_sql(&self) -> Option<crate::Sql> {
        match self.sqls() {
            Some(sqls) => {
                let mut final_sql = crate::Sql::default();
                let len = sqls.len();
                if len > 0 {
                    for (idx, sql) in sqls.iter().enumerate() {
                        final_sql.push_sql(sql.clone());
                        if idx < len - 1 {
                            final_sql.push_str(self.join_str());
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

#[derive(Debug)]
pub struct Filter<M: Arel> {
    sub_filters: Vec<Box<dyn ArelSubFilterStatement>>,
    _marker: PhantomData<M>,
}

impl<T: Arel> ArelStatement for Filter<T> {
    fn to_sql(&self) -> Option<crate::Sql> {
        let sub_filters: Vec<&Box<dyn ArelSubFilterStatement>> = self.sub_filters.iter().collect();
        if self.sub_filters.len() > 0 {
            let mut final_sql = crate::Sql::new("");
            // sub_filters.sort_by(|a, b| a.order().partial_cmp(&b.order()).unwrap());
            for (idx, sub_filters) in sub_filters.into_iter().enumerate() {
                let sql = sub_filters.to_sql();
                if let Some(sql) = sql {
                    if idx >= 1 {
                        final_sql.push_str(sub_filters.join_str());
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

impl<M: Arel> Filter<M> {
    pub fn new() -> Self {
        Self {
            sub_filters: vec![],
            _marker: PhantomData::<M>,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::new();
    /// filter.filter_and("username", "sanmu");
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"})"#);
    ///
    /// filter.filter_and("age", vec![18, 20]);
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"}) AND ("user"."age" IN (18,20))"#);
    ///
    /// ```
    pub fn filter_and<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.filter_and_pairs(vec![(key, value)])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::new();
    /// filter.filter_and_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"} AND "user"."age" IN (18,20))"#);
    ///
    /// filter.filter_and("gender", "male");
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"} AND "user"."age" IN (18,20)) AND ("user"."gender" = ?{"String":"male"})"#);
    ///
    /// ```
    pub fn filter_and_pairs<K: AsRef<str>, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut filter_and = FilterAnd::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            match &value {
                crate::Value::Array(_) => {
                    sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
                    if let Some(v_sql) = value.to_sql() {
                        sql.push_str(" IN ").push_sql(v_sql);
                    } else {
                        sql.push_str(" IS NULL");
                    }
                }
                _ => {
                    sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
                    if let Some(v_sql) = value.to_sql() {
                        sql.push_str(" = ").push_sql(v_sql);
                    } else {
                        sql.push_str(" IS NULL");
                    }
                }
            }
            filter_and.sqls.push(sql);
        }
        self.sub_filters.push(Box::new(filter_and));
        self
    }
    pub fn filter_and_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        let mut filter_and = FilterAnd::default();
        filter_and.sqls.push(sql.into());
        self.sub_filters.push(Box::new(filter_and));
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::new();
    /// filter.filter_and_not("username", "sanmu");
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" != ?{"String":"sanmu"})"#);
    ///
    /// filter.filter_and_not("aga", vec![18, 20]);
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" != ?{"String":"sanmu"}) AND ("user"."aga" NOT IN (18,20))"#);
    ///
    /// ```
    pub fn filter_and_not<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.filter_and_not_pairs(vec![(key, value)])
    }
    pub fn filter_and_not_pairs<K: AsRef<str>, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut filter_and = FilterAnd::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            match &value {
                crate::Value::Array(_) => {
                    sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
                    if let Some(v_sql) = value.to_sql() {
                        sql.push_str(" NOT IN ").push_sql(v_sql);
                    } else {
                        sql.push_str(" IS NOT NULL");
                    }
                }
                _ => {
                    sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
                    if let Some(v_sql) = value.to_sql() {
                        sql.push_str(" != ").push_sql(v_sql);
                    } else {
                        sql.push_str(" IS NOT NULL");
                    }
                }
            }
            filter_and.sqls.push(sql);
        }
        self.sub_filters.push(Box::new(filter_and));
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::new();
    /// filter.filter_or("username", "sanmu");
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"})"#);
    ///
    /// filter.filter_or("age", vec![18, 20]);
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"}) OR ("user"."age" IN (18,20))"#);
    ///
    /// filter.filter_and("gender", "male");
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"}) OR ("user"."age" IN (18,20)) AND ("user"."gender" = ?{"String":"male"})"#);
    ///
    /// ```
    pub fn filter_or<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.filter_or_pairs(vec![(key, value)])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::new();
    /// filter.filter_or_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"} OR "user"."age" IN (18,20))"#);
    ///
    /// filter.filter_and("gender", "male");
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"} OR "user"."age" IN (18,20)) AND ("user"."gender" = ?{"String":"male"})"#);
    ///
    /// ```
    pub fn filter_or_pairs<K: AsRef<str>, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut filter_or = FilterOr::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            match &value {
                crate::Value::Array(_) => {
                    sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
                    if let Some(v_sql) = value.to_sql() {
                        sql.push_str(" IN ").push_sql(v_sql);
                    } else {
                        sql.push_str(" IS NULL");
                    }
                }
                _ => {
                    sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
                    if let Some(v_sql) = value.to_sql() {
                        sql.push_str(" = ").push_sql(v_sql);
                    } else {
                        sql.push_str(" IS NULL");
                    }
                }
            }
            filter_or.sqls.push(sql);
        }
        self.sub_filters.push(Box::new(filter_or));
        self
    }
    pub fn filter_or_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        let mut filter_or = FilterOr::default();
        filter_or.sqls.push(sql.into());
        self.sub_filters.push(Box::new(filter_or));
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::new();
    /// filter.filter_or_not("username", "sanmu");
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" != ?{"String":"sanmu"})"#);
    ///
    /// filter.filter_or_not("aga", vec![18, 20]);
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" != ?{"String":"sanmu"}) OR ("user"."aga" NOT IN (18,20))"#);
    ///
    /// ```
    pub fn filter_or_not<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.filter_or_not_pairs(vec![(key, value)])
    }
    pub fn filter_or_not_pairs<K: AsRef<str>, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut filter_or = FilterOr::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            match &value {
                crate::Value::Array(_) => {
                    sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
                    if let Some(v_sql) = value.to_sql() {
                        sql.push_str(" NOT IN ").push_sql(v_sql);
                    } else {
                        sql.push_str(" IS NOT NULL");
                    }
                }
                _ => {
                    sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
                    if let Some(v_sql) = value.to_sql() {
                        sql.push_str(" != ").push_sql(v_sql);
                    } else {
                        sql.push_str(" IS NOT NULL");
                    }
                }
            }
            filter_or.sqls.push(sql);
        }
        self.sub_filters.push(Box::new(filter_or));
        self
    }
    pub fn unfilter_starts_with<K: AsRef<str>>(&mut self, start: K) -> &mut Self {
        for sub_filter in self.sub_filters.iter_mut() {
            if let Some(sqls) = sub_filter.sqls_mut() {
                sqls.retain(|sql| !sql.value.starts_with(start.as_ref()));
            }
        }
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::new();
    /// filter.filter_and_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// filter.unfilter("age");
    /// assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ?{"String":"sanmu"})"#);
    ///
    /// ```
    pub fn unfilter<K: ToString>(&mut self, key: K) -> &mut Self {
        let table_name = M::table_name();
        let start_string = format!(r#""{}"."{}""#, table_name, key.to_string());
        self.unfilter_starts_with(start_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    struct User {}
    impl SuperArel for User {}
    impl Arel for User {}

    #[test]
    fn it_works() {
        let mut filter = Filter::<User>::new();
        assert_eq!(filter.to_sql(), None);

        let mut filter_and = FilterAnd::default();
        filter_and.sqls.push(crate::Sql::new_with_prepares("name = ?", vec!["sanmu"]));
        assert_eq!(filter_and.to_sql().unwrap().to_sql_string().unwrap(), r#"name = ?{"String":"sanmu"}"#);
        filter_and.sqls.push(crate::Sql::new_with_prepares("age = ?", vec![18]));
        assert_eq!(filter_and.to_sql().unwrap().to_sql_string().unwrap(), r#"name = ?{"String":"sanmu"} AND age = ?{"Int":18}"#);
        filter.sub_filters.push(Box::new(filter_and));
        assert!(filter.to_sql().is_some());
        assert_eq!(filter.to_sql().unwrap().to_sql_string().unwrap(), r#"(name = ?{"String":"sanmu"} AND age = ?{"Int":18})"#);

        let mut filter_or = FilterOr::default();
        filter_or.sqls.push(crate::Sql::new_with_prepares("name = ?", vec!["sanmu"]));
        assert_eq!(filter_or.to_sql().unwrap().to_sql_string().unwrap(), r#"name = ?{"String":"sanmu"}"#);
        filter_or.sqls.push(crate::Sql::new_with_prepares("age = ?", vec![18]));
        assert_eq!(filter_or.to_sql().unwrap().to_sql_string().unwrap(), r#"name = ?{"String":"sanmu"} OR age = ?{"Int":18}"#);
        filter.sub_filters.push(Box::new(filter_or));
        assert!(filter.to_sql().is_some());
        assert_eq!(
            filter.to_sql().unwrap().to_sql_string().unwrap(),
            r#"(name = ?{"String":"sanmu"} AND age = ?{"Int":18}) OR (name = ?{"String":"sanmu"} OR age = ?{"Int":18})"#
        );

        // bytes
        let mut filter_and = FilterAnd::default();
        let bytes_value: crate::Value = bytes::Bytes::from_static(b"hello").into();
        filter_and.sqls.push(crate::Sql::new_with_prepares("bytes = ?", vec![bytes_value]));
        assert_eq!(filter_and.to_sql().unwrap().to_sql_string().unwrap(), r#"bytes = ?b"hello""#);

        // array
        let mut filter_and = FilterAnd::default();
        let bytes: Vec<crate::Value> = vec![18.into(), 19.into(), 20.into()];
        let bytes_value: crate::Value = bytes.into();
        filter_and.sqls.push(crate::Sql::new_with_prepares("age IN ?", vec![bytes_value]));
        assert_eq!(filter_and.to_sql().unwrap().to_sql_string().unwrap(), r#"age IN ?{"Array":[{"Int":18},{"Int":19},{"Int":20}]}"#);
    }
}
