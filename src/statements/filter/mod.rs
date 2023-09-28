mod and_filter;
mod or_filter;

pub(crate) use and_filter::AndFilter;
pub(crate) use or_filter::OrFilter;

use crate::{statements::ArelStatement, Arel};
use std::{fmt::Debug, marker::PhantomData, ops::Deref};

trait ArelSubFilterStatement: Debug {
    fn sqls(&self) -> &Vec<crate::Sql>;
    fn sqls_mut(&mut self) -> &mut Vec<crate::Sql>;
    fn join_str(&self) -> &'static str;
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        let mut final_sql = crate::Sql::default();
        let len = self.sqls().len();
        if len > 0 {
            for (idx, sql) in self.sqls().iter().enumerate() {
                final_sql.push_sql(sql.clone());
                if idx < len - 1 {
                    final_sql.push_str(self.join_str());
                }
            }
        }
        Ok(Some(final_sql))
    }
}

#[derive(Debug)]
pub struct Filter<M: crate::Arel> {
    sub_filters: Vec<Box<dyn ArelSubFilterStatement + Sync + Send + 'static>>,
    _marker: PhantomData<M>,
}
impl<M: crate::Arel> Default for Filter<M> {
    fn default() -> Self {
        Self {
            sub_filters: vec![],
            _marker: PhantomData,
        }
    }
}

impl<M: crate::Arel> ArelStatement for Filter<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        let sub_filters: Vec<&Box<dyn ArelSubFilterStatement + Sync + Send + 'static>> = self.sub_filters.iter().collect();
        if self.sub_filters.len() > 0 {
            let mut final_sql = crate::Sql::new("");
            for (idx, sub_filter) in sub_filters.into_iter().enumerate() {
                let sql = sub_filter.to_sql()?;
                if let Some(sql) = sql {
                    if idx >= 1 {
                        final_sql.push_str(sub_filter.join_str());
                    }
                    if sub_filter.join_str() == " OR " && sub_filter.sqls().len() > 1 {
                        final_sql.push_str("(").push_sql(sql).push_str(")");
                    } else {
                        final_sql.push_sql(sql);
                    }
                }
            }
            Ok(Some(final_sql))
        } else {
            Ok(None)
        }
    }
}

impl<M: Arel> Filter<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::default();
    /// filter.and_filter("username", "sanmu");
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" = ?"#);
    ///
    /// filter.and_filter("age", vec![18, 20]);
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" = ? AND "user"."age" IN (?, ?)"#);
    ///
    /// ```
    pub fn and_filter<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.and_filter_pairs(vec![(key, value)])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::default();
    /// filter.and_filter_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" = ? AND "user"."age" IN (?, ?)"#);
    ///
    /// filter.and_filter("gender", "male");
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" = ? AND "user"."age" IN (?, ?) AND "user"."gender" = ?"#);
    ///
    /// ```
    pub fn and_filter_pairs<K: AsRef<str>, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut and_filter = AndFilter::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
            match &value {
                crate::Value::Array(arrary) => match arrary.deref() {
                    Some(arr) => {
                        sql.push_str(" IN (").push_binds(arr.clone(), ", ").push_str(")");
                    }
                    None => {
                        sql.push_str(" IS NULL");
                    }
                },
                _ => {
                    if !value.is_null() {
                        sql.push_str(" = ").push_bind(value);
                    } else {
                        sql.push_str(" IS NULL");
                    }
                }
            }
            and_filter.sqls.push(sql);
        }
        self.sub_filters.push(Box::new(and_filter));
        self
    }
    pub fn and_filter_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        let mut and_filter = AndFilter::default();
        and_filter.sqls.push(sql.into());
        self.sub_filters.push(Box::new(and_filter));
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
    /// let mut filter = Filter::<User>::default();
    /// filter.and_not_filter("username", "sanmu");
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" != ?"#);
    ///
    /// filter.and_not_filter("aga", vec![18, 20]);
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" != ? AND "user"."aga" NOT IN (?, ?)"#);
    ///
    /// ```
    pub fn and_not_filter<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.and_not_filter_pairs(vec![(key, value)])
    }
    pub fn and_not_filter_pairs<K: AsRef<str>, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut and_filter = AndFilter::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
            match &value {
                crate::Value::Array(arrary) => match arrary.deref() {
                    Some(arr) => {
                        sql.push_str(" NOT IN (").push_binds(arr.clone(), ", ").push_str(")");
                    }
                    None => {
                        sql.push_str(" IS NOT NULL");
                    }
                },
                _ => {
                    if !value.is_null() {
                        sql.push_str(" != ").push_bind(value);
                    } else {
                        sql.push_str(" IS NOT NULL");
                    }
                }
            }
            and_filter.sqls.push(sql);
        }
        self.sub_filters.push(Box::new(and_filter));
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
    /// let mut filter = Filter::<User>::default();
    /// filter.or_filter("username", "sanmu");
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" = ?"#);
    ///
    /// filter.or_filter("age", vec![18, 20]);
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" = ? OR "user"."age" IN (?, ?)"#);
    ///
    /// filter.and_filter("gender", "male");
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" = ? OR "user"."age" IN (?, ?) AND "user"."gender" = ?"#);
    ///
    /// ```
    pub fn or_filter<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.or_filter_pairs(vec![(key, value)])
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::default();
    /// filter.or_filter_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ? OR "user"."age" IN (?, ?))"#);
    ///
    /// filter.or_filter("gender", "male");
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"("user"."username" = ? OR "user"."age" IN (?, ?)) OR "user"."gender" = ?"#);
    ///
    /// ```
    pub fn or_filter_pairs<K: AsRef<str>, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut or_filter = OrFilter::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
            match &value {
                crate::Value::Array(arrary) => match arrary.deref() {
                    Some(arr) => {
                        sql.push_str(" IN (").push_binds(arr.clone(), ", ").push_str(")");
                    }
                    None => {
                        sql.push_str(" IS NULL");
                    }
                },
                _ => {
                    if !value.is_null() {
                        sql.push_str(" = ").push_bind(value);
                    } else {
                        sql.push_str(" IS NULL");
                    }
                }
            }
            or_filter.sqls.push(sql);
        }
        self.sub_filters.push(Box::new(or_filter));
        self
    }
    pub fn or_filter_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        let mut or_filter = OrFilter::default();
        or_filter.sqls.push(sql.into());
        self.sub_filters.push(Box::new(or_filter));
        self
    }
    /// # Examples
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// ```
    /// use arel::prelude::*;
    /// use arel::statements::filter::Filter;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut filter = Filter::<User>::default();
    /// filter.or_not_filter("username", "sanmu");
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" != ?"#);
    ///
    /// filter.or_not_filter("aga", vec![18, 20]);
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" != ? OR "user"."aga" NOT IN (?, ?)"#);
    ///
    /// ```
    pub fn or_not_filter<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.or_not_filter_pairs(vec![(key, value)])
    }
    pub fn or_not_filter_pairs<K: AsRef<str>, V: Into<crate::Value>>(&mut self, pairs: Vec<(K, V)>) -> &mut Self {
        let table_name = M::table_name();
        let mut or_filter = OrFilter::default();
        for (key, value) in pairs.into_iter() {
            let mut sql = crate::Sql::default();
            let value: crate::Value = value.into();
            sql.push_str(format!(r#""{}"."{}""#, table_name, key.as_ref()));
            match &value {
                crate::Value::Array(arrary) => match arrary.deref() {
                    Some(arr) => {
                        sql.push_str(" NOT IN (").push_binds(arr.clone(), ", ").push_str(")");
                    }
                    None => {
                        sql.push_str(" IS NOT NULL");
                    }
                },
                _ => {
                    if !value.is_null() {
                        sql.push_str(" != ").push_bind(value);
                    } else {
                        sql.push_str(" IS NOT NULL");
                    }
                }
            }
            or_filter.sqls.push(sql);
        }
        self.sub_filters.push(Box::new(or_filter));
        self
    }
    pub fn unfilter_starts_with<K: AsRef<str>>(&mut self, start: K) -> &mut Self {
        for sub_filter in self.sub_filters.iter_mut() {
            sub_filter.sqls_mut().retain(|sql| !sql.raw_value.starts_with(start.as_ref()));
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
    /// let mut filter = Filter::<User>::default();
    /// filter.and_filter_pairs(vec![("username", Into::<arel::Value>::into("sanmu")), ("age", Into::<arel::Value>::into(vec![18, 20]))]);
    /// filter.unfilter("age");
    /// #[cfg(any(feature = "sqlite", feature = "mysql"))]
    /// assert_eq!(filter.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#""user"."username" = ?"#);
    ///
    /// ```
    pub fn unfilter<K: ToString>(&mut self, key: K) -> &mut Self {
        let table_name = M::table_name();
        let start_string = format!(r#""{}"."{}""#, table_name, key.to_string());
        self.unfilter_starts_with(start_string)
    }
}
