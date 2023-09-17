mod query_builder;

use query_builder::QueryBuilder;
use std::ops::{Bound, RangeBounds};

#[derive(Debug)]
pub struct Sql {
    pub raw_value: String,
    pub bind_indexs: Vec<usize>,
    pub bind_values: Vec<crate::Value>,
}

impl Default for Sql {
    fn default() -> Self {
        Self {
            raw_value: String::new(),
            bind_indexs: vec![],
            bind_values: vec![],
        }
    }
}

impl Sql {
    pub fn new<T: ToString>(value: T) -> Self {
        Self {
            raw_value: value.to_string(),
            bind_indexs: vec![],
            bind_values: vec![],
        }
    }
    pub fn push_str<T: AsRef<str>>(&mut self, raw_str: T) -> &mut Self {
        self.raw_value.push_str(raw_str.as_ref());
        self
    }
    pub fn push_bind<V: Into<crate::Value>>(&mut self, bind_value: V) -> &mut Self {
        self.bind_indexs.push(self.raw_value.len());
        self.push_str("?");
        self.bind_values.push(bind_value.into());
        self
    }
    pub fn push_str_with_bind<T: AsRef<str>, V: Into<crate::Value>>(&mut self, raw_str: T, bind_value: V) -> &mut Self {
        self.push_str(raw_str);
        self.push_bind(bind_value);
        self
    }
    pub fn push_sql(&mut self, sql: Sql) -> &mut Self {
        let raw_value_len = self.raw_value.len();
        self.push_str(sql.raw_value);
        self.bind_indexs.extend(sql.bind_indexs.into_iter().map(|idx| raw_value_len + idx).collect::<Vec<usize>>());
        self.bind_values.extend(sql.bind_values);
        self
    }
    pub fn to_debug_sql_string(&self) -> crate::Result<String> {
        let query_builder: QueryBuilder = self.try_into()?;
        Ok(query_builder.sql().to_string())
    }
}

impl Sql {
    /// # Examples
    ///
    /// ```
    /// let sql = arel::Sql::range_sql("age", ..18).unwrap();
    /// assert_eq!(sql.to_debug_sql_string().unwrap(), r#"age < 18"#);
    ///
    /// let sql = arel::Sql::range_sql("age", ..=18).unwrap();
    /// assert_eq!(sql.to_debug_sql_string().unwrap(), r#"age <= 18"#);
    ///
    /// let sql = arel::Sql::range_sql("age", 18..20).unwrap();
    /// assert_eq!(sql.to_debug_sql_string().unwrap(), r#"age >= 18 AND age < 20"#);
    ///
    /// let sql = arel::Sql::range_sql("age", 18..=20).unwrap();
    /// assert_eq!(sql.to_debug_sql_string().unwrap(), r#"age BETWEEN 18 AND 20"#);
    ///
    /// let sql = arel::Sql::range_sql("age", (std::ops::Bound::Excluded(18), std::ops::Bound::Included(20))).unwrap();
    /// assert_eq!(sql.to_debug_sql_string().unwrap(), r#"age > 18 AND age <= 20"#);
    ///
    /// let sql = arel::Sql::range_sql("age", 18..).unwrap();
    /// assert_eq!(sql.to_debug_sql_string().unwrap(), r#"age >= 18"#);
    ///
    /// ```
    pub fn range_sql<K: AsRef<str>, V: ToString, R: RangeBounds<V>>(key: K, range: R) -> Option<Sql> {
        let raw_sql;
        match range.start_bound() {
            Bound::Unbounded => match range.end_bound() {
                Bound::Unbounded => return None,
                Bound::Included(end) => {
                    raw_sql = format!("{} <= {}", key.as_ref(), end.to_string());
                }
                Bound::Excluded(end) => {
                    raw_sql = format!("{} < {}", key.as_ref(), end.to_string());
                }
            },
            Bound::Included(start) => match range.end_bound() {
                Bound::Unbounded => raw_sql = format!("{} >= {}", key.as_ref(), start.to_string()),
                Bound::Included(end) => raw_sql = format!("{} BETWEEN {} AND {}", key.as_ref(), start.to_string(), end.to_string()),
                Bound::Excluded(end) => raw_sql = format!("{} >= {} AND {} < {}", key.as_ref(), start.to_string(), key.as_ref(), end.to_string()),
            },
            Bound::Excluded(start) => match range.end_bound() {
                Bound::Unbounded => raw_sql = format!("{} > {}", key.as_ref(), start.to_string()),
                Bound::Included(end) => raw_sql = format!("{} > {} AND {} <= {}", key.as_ref(), start.to_string(), key.as_ref(), end.to_string()),
                Bound::Excluded(end) => raw_sql = format!("{} > {} AND {} < {}", key.as_ref(), start.to_string(), key.as_ref(), end.to_string()),
            },
        }
        Some(Sql::new(raw_sql))
    }
}

#[cfg(test)]
mod tests {
    use crate::sub_value::{ValueInt, ValueString};

    use super::*;

    #[test]
    fn it_works() {
        let v1: ValueInt = 1.into();
        let v2: ValueString = "sanmu".into();
        let mut sql1 = Sql::default();
        let mut sql2 = Sql::default();
        let mut sql3 = Sql::default();

        sql1.push_str("select ").push_str_with_bind(r#"* from users where users.id = "#, v1);
        sql2.push_str_with_bind(" and name = ", v2);
        sql3.push_sql(sql1).push_sql(sql2);

        #[cfg(any(feature = "sqlite", feature = "mysql"))]
        assert_eq!(sql3.to_debug_sql_string().unwrap(), r#"select * from users where users.id = ? and name = ?"#.to_owned());
        #[cfg(any(feature = "postgres"))]
        assert_eq!(sql3.to_debug_sql_string().unwrap(), r#"select * from users where users.id = $1 and name = $2"#.to_owned());
    }
}
