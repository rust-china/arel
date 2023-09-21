mod query_builder;

pub use query_builder::QueryBuilder;
use std::ops::{Bound, DerefMut, RangeBounds};

#[derive(Debug, Clone)]
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

impl TryFrom<Sql> for String {
    type Error = crate::Error;
    fn try_from(sql: Sql) -> Result<Self, Self::Error> {
        sql.to_sql_string()
    }
}

impl<T: ToString> From<T> for Sql {
    fn from(value: T) -> Self {
        Self::new(value)
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
    pub fn push_strs<T: AsRef<str>>(&mut self, raw_strs: Vec<T>, separated_str: &str) -> &mut Self {
        let len = raw_strs.len();
        for (idx, raw_str) in raw_strs.into_iter().enumerate() {
            self.push_str(raw_str);
            if idx < len - 1 {
                self.push_str(separated_str);
            }
        }
        self
    }
    pub fn push_bind<V: Into<crate::Value>>(&mut self, bind_value: V) -> &mut Self {
        self.bind_indexs.push(self.raw_value.len());
        self.push_str("?");
        self.bind_values.push(bind_value.into());
        self
    }
    pub fn push_binds<V: Into<crate::Value>>(&mut self, bind_values: Vec<V>, separated_str: &str) -> &mut Self {
        let len = bind_values.len();
        for (idx, bind_value) in bind_values.into_iter().enumerate() {
            self.push_bind(bind_value);
            if idx < len - 1 {
                self.push_str(separated_str);
            }
        }
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
    pub fn push_sqls(&mut self, sqls: Vec<Sql>, separated_str: &str) -> &mut Self {
        let len = sqls.len();
        for (idx, sql) in sqls.into_iter().enumerate() {
            self.push_sql(sql);
            if idx < len - 1 {
                self.push_str(separated_str);
            }
        }
        self
    }
    pub fn to_sql_string(&self) -> crate::Result<String> {
        let query_builder: QueryBuilder = self.try_into()?;
        Ok(query_builder.sql().to_string())
    }
}

impl Sql {
    /// # Examples
    ///
    /// ```
    /// let sql = arel::Sql::range_sql("age", ..18).unwrap();
    /// assert_eq!(sql.to_sql_string().unwrap(), r#"age < 18"#);
    ///
    /// let sql = arel::Sql::range_sql("age", ..=18).unwrap();
    /// assert_eq!(sql.to_sql_string().unwrap(), r#"age <= 18"#);
    ///
    /// let sql = arel::Sql::range_sql("age", 18..20).unwrap();
    /// assert_eq!(sql.to_sql_string().unwrap(), r#"age >= 18 AND age < 20"#);
    ///
    /// let sql = arel::Sql::range_sql("age", 18..=20).unwrap();
    /// assert_eq!(sql.to_sql_string().unwrap(), r#"age BETWEEN 18 AND 20"#);
    ///
    /// let sql = arel::Sql::range_sql("age", (std::ops::Bound::Excluded(18), std::ops::Bound::Included(20))).unwrap();
    /// assert_eq!(sql.to_sql_string().unwrap(), r#"age > 18 AND age <= 20"#);
    ///
    /// let sql = arel::Sql::range_sql("age", 18..).unwrap();
    /// assert_eq!(sql.to_sql_string().unwrap(), r#"age >= 18"#);
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

impl Sql {
    #[allow(dead_code)]
    pub async fn exec<'a, E>(&self, executor: E) -> crate::Result<crate::db::DatabaseQueryResult>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        let mut query_builder: QueryBuilder = self.try_into()?;
        let query = query_builder.deref_mut().build();
        match query.execute(executor).await {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(anyhow::anyhow!(err.to_string()).into()),
        }
    }
    pub async fn fetch_one_exec<'a, E>(&self, executor: E) -> crate::Result<crate::db::DatabaseRow>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        let mut query_builder: QueryBuilder = self.try_into()?;
        let query = query_builder.deref_mut().build();
        match query.fetch_one(executor).await {
            Ok(val) => Ok(val),
            Err(err) => Err(anyhow::anyhow!(err.to_string()).into()),
        }
    }
    pub async fn fetch_one_as_exec<'a, T, E>(&self, executor: E) -> crate::Result<T>
    where
        for<'b> T: Send + Unpin + sqlx::FromRow<'b, crate::db::DatabaseRow>,
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        let mut query_builder: QueryBuilder = self.try_into()?;
        let query_as = query_builder.build_query_as::<T>();
        match query_as.fetch_one(executor).await {
            Ok(val) => Ok(val),
            Err(err) => Err(anyhow::anyhow!(err.to_string()).into()),
        }
    }
    pub(crate) async fn fetch_all_exec<'a, E>(&self, executor: E) -> crate::Result<Vec<crate::db::DatabaseRow>>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        let mut query_builder: QueryBuilder = self.try_into()?;
        let query = query_builder.build();
        match query.fetch_all(executor).await {
            Ok(val) => Ok(val),
            Err(err) => Err(anyhow::anyhow!(err.to_string()).into()),
        }
    }
    pub(crate) async fn fetch_all_as_exec<'a, T, E>(&self, executor: E) -> crate::Result<Vec<T>>
    where
        for<'b> T: Send + Unpin + sqlx::FromRow<'b, crate::db::DatabaseRow>,
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        let mut query_builder: QueryBuilder = self.try_into()?;
        let query_as = query_builder.build_query_as::<T>();
        match query_as.fetch_all(executor).await {
            Ok(val) => Ok(val),
            Err(err) => Err(anyhow::anyhow!(err.to_string()).into()),
        }
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
        assert_eq!(sql3.to_sql_string().unwrap(), r#"select * from users where users.id = ? and name = ?"#.to_owned());
        #[cfg(any(feature = "postgres"))]
        assert_eq!(sql3.to_sql_string().unwrap(), r#"select * from users where users.id = $1 and name = $2"#.to_owned());
    }
}
