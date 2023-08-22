mod query_builder;

pub use query_builder::QueryBuilder;
use std::ops::{Bound, DerefMut, RangeBounds};

#[derive(Clone, Debug, PartialEq)]
pub struct Sql {
    pub value: String,
    pub prepare_values: Option<Vec<crate::Value>>,
}
impl Default for Sql {
    fn default() -> Self {
        Self {
            value: String::new(),
            prepare_values: None,
        }
    }
}

impl Sql {
    pub fn new<T: ToString>(value: T) -> Self {
        Self {
            value: value.to_string(),
            prepare_values: None,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    ///
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
    pub fn range_sql<K: AsRef<str>, V: ToString, R: RangeBounds<V>>(key: K, range: R) -> Option<crate::Sql> {
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
        Some(crate::Sql::new(raw_sql))
    }
}

impl Sql {
    pub fn new_with_prepare<T: ToString, V: Into<crate::Value>>(value: T, prepare_value: V) -> Self {
        Self {
            value: value.to_string(),
            prepare_values: Some(vec![prepare_value.into()]),
        }
    }
    // pub fn new_with_prepare_one<T: ToString, V: Into<crate::Value>>(value: T, prepare_value: V) -> Self {
    //     Self::new_with_prepare(value, prepare_value)
    // }
    pub fn new_with_prepares<T: ToString, V: Into<crate::Value>>(value: T, prepare_values: Vec<V>) -> Self {
        Self {
            value: value.to_string(),
            prepare_values: Some(prepare_values.into_iter().map(|v| v.into()).collect()),
        }
    }
    // pub fn new_with_prepare_multiple<T: ToString, V: Into<crate::Value>>(value: T, prepare_values: Vec<V>) -> Self {
    //     Self::new_with_prepares(value, prepare_values)
    // }
    pub fn push(&mut self, r#char: char) -> &mut Self {
        self.value.push(r#char);
        self
    }
    pub fn push_str<T: AsRef<str>>(&mut self, sub_str: T) -> &mut Self {
        self.value.push_str(sub_str.as_ref());
        self
    }
    pub fn push_prepare_value<V: Into<crate::Value>>(&mut self, sub_prepare_value: V) -> &mut Self {
        let sub_prepare_value: crate::Value = sub_prepare_value.into();
        if let Some(prepare_values) = &mut self.prepare_values {
            // prepare_value.extend_from_slice(&sub_prepare_value);
            prepare_values.push(sub_prepare_value)
        } else {
            self.prepare_values = Some(vec![sub_prepare_value]);
        }
        self
    }
    pub fn push_prepare_values<V: Into<crate::Value>>(&mut self, sub_prepare_values: Vec<V>) -> &mut Self {
        let sub_prepare_values: Vec<crate::Value> = sub_prepare_values.into_iter().map(|v| v.into()).collect();
        if let Some(prepare_value) = &mut self.prepare_values {
            prepare_value.extend_from_slice(&sub_prepare_values);
        } else {
            self.prepare_values = Some(sub_prepare_values);
        }
        self
    }
    pub fn push_str_with_prepare_value<T: AsRef<str>, V: Into<crate::Value>>(&mut self, sub_str: T, sub_prepare_value: V) -> &mut Self {
        self.push_str(sub_str);
        self.push_prepare_value(sub_prepare_value);
        self
    }
    pub fn push_str_with_prepare_values<V: Into<crate::Value>>(&mut self, sub_str: &str, sub_prepare_value: Vec<V>) -> &mut Self {
        self.value.push_str(sub_str);
        self.push_prepare_values(sub_prepare_value);
        self
    }
    pub fn push_sql(&mut self, sql: Sql) -> &mut Self {
        if let Some(prepare_values) = sql.prepare_values {
            self.push_str_with_prepare_values(&sql.value, prepare_values);
        } else {
            self.push_str(&sql.value);
        }
        self
    }
    pub fn push_sqls(&mut self, sqls: Vec<Sql>, join_str: &str) -> &mut Self {
        let len = sqls.len();
        for (idx, sql) in sqls.into_iter().enumerate() {
            self.push_sql(sql);
            if idx != len - 1 {
                self.push_str(join_str);
            }
        }
        self
    }
    pub fn prepare_symbol(&self) -> String {
        if cfg!(feature = "postgres") {
            return format!("${}", if let Some(prepare_values) = &self.prepare_values { prepare_values.len() + 1 } else { 1 });
        }
        "?".to_string()
    }
    pub fn to_sql_string(&self) -> anyhow::Result<String> {
        if let Some(prepare_values) = &self.prepare_values {
            let mut prepare_idx = 0;

            let mut raw_sql = String::new();
            let chars = self.value.chars().collect::<Vec<char>>();
            let chars_len = chars.len();
            let mut idx = 0;
            while idx < chars_len {
                let ch = chars[idx];
                let mut next_idx = idx + 1;
                let mut should_replace = false;
                match ch {
                    '?' => {
                        should_replace = true;
                    }
                    '$' => {
                        let mut prepare_end_idx = idx;
                        while prepare_end_idx < chars_len - 1 {
                            match chars[prepare_end_idx + 1] {
                                '0'..='9' => {
                                    prepare_end_idx += 1;
                                }
                                _ => break,
                            }
                        }
                        if prepare_end_idx > idx {
                            should_replace = true;
                            next_idx = prepare_end_idx + 1;
                        }
                    }
                    _ => (),
                }

                if should_replace {
                    let prepare_value = prepare_values.get(prepare_idx).ok_or_else(|| anyhow::anyhow!("参数不足"))?;
                    prepare_idx += 1;
                    match prepare_value {
                        crate::Value::Bytes(Some(bytes)) => {
                            raw_sql.push_str(&format!(r#"?b"{}""#, bytes.escape_ascii().to_string()));
                        }
                        _ => {
                            raw_sql.push_str(&format!("?{}", serde_json::to_string(prepare_value).unwrap()));
                        }
                    }
                } else {
                    raw_sql.push(ch);
                }

                idx = next_idx;
            }
            Ok(raw_sql)
        } else {
            Ok(self.value.clone())
        }
    }
}

impl Sql {
    pub fn to_query_builder<'a>(&self) -> anyhow::Result<QueryBuilder<'a>> {
        let mut query_builder = QueryBuilder::default();
        if let Some(prepare_values) = &self.prepare_values {
            let mut prepare_idx = 0;

            let chars = self.value.chars().collect::<Vec<char>>();
            let chars_len = chars.len();
            let mut idx = 0;
            while idx < chars_len {
                let ch = chars[idx];
                let mut next_idx = idx + 1;
                let mut should_replace = false;
                match ch {
                    '?' => {
                        should_replace = true;
                    }
                    '$' => {
                        let mut prepare_end_idx = idx;
                        while prepare_end_idx < chars_len - 1 {
                            match chars[prepare_end_idx + 1] {
                                '0'..='9' => {
                                    prepare_end_idx += 1;
                                }
                                _ => break,
                            }
                        }
                        if prepare_end_idx > idx {
                            should_replace = true;
                            next_idx = prepare_end_idx + 1;
                        } else {
                            query_builder.push('$');
                        }
                    }
                    other @ _ => {
                        query_builder.push(other);
                    }
                }

                if should_replace {
                    let prepare_value = prepare_values.get(prepare_idx).ok_or_else(|| anyhow::anyhow!("参数不足"))?;
                    prepare_idx += 1;
                    query_builder.push_bind_prepare_value(prepare_value)?;
                }

                idx = next_idx;
            }
        } else {
            query_builder.push(&self.value);
        }
        Ok(query_builder)
    }
    pub(crate) async fn fetch_one_exec<'a, E>(&self, executor: E) -> anyhow::Result<crate::DatabaseRow>
    where
        E: sqlx::Executor<'a, Database = crate::Database>,
    {
        let mut query_builder = self.to_query_builder()?;
        let query = query_builder.deref_mut().build();
        match query.fetch_one(executor).await {
            Ok(val) => Ok(val),
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }
    }
    pub(crate) async fn fetch_one_exec_as<'a, T, E>(&self, executor: E) -> anyhow::Result<T>
    where
        for<'b> T: Send + Unpin + sqlx::FromRow<'b, crate::DatabaseRow>,
        E: sqlx::Executor<'a, Database = crate::Database>,
    {
        let mut query_builder = self.to_query_builder()?;
        let query_as = query_builder.build_query_as::<T>();
        match query_as.fetch_one(executor).await {
            Ok(val) => Ok(val),
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }
    }
    pub(crate) async fn fetch_all_exec<'a, E>(&self, executor: E) -> anyhow::Result<Vec<crate::DatabaseRow>>
    where
        E: sqlx::Executor<'a, Database = crate::Database>,
    {
        let mut query_builder = self.to_query_builder()?;
        let query = query_builder.build();
        match query.fetch_all(executor).await {
            Ok(val) => Ok(val),
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }
    }
    pub(crate) async fn fetch_all_exec_as<'a, T, E>(&self, executor: E) -> anyhow::Result<Vec<T>>
    where
        for<'b> T: Send + Unpin + sqlx::FromRow<'b, crate::DatabaseRow>,
        E: sqlx::Executor<'a, Database = crate::Database>,
    {
        let mut query_builder = self.to_query_builder()?;
        let query_as = query_builder.build_query_as::<T>();
        match query_as.fetch_all(executor).await {
            Ok(val) => Ok(val),
            Err(err) => Err(anyhow::anyhow!(err.to_string())),
        }
    }
}

impl TryFrom<Sql> for String {
    type Error = anyhow::Error;
    fn try_from(sql: Sql) -> Result<Self, Self::Error> {
        sql.to_sql_string()
    }
}

impl<T: ToString> From<T> for Sql {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sql = Sql::default();
        sql.push_str("select").push(' ').push_str_with_prepare_values(
            r#"* from users where users.id = ? and name = ?"#,
            vec![Into::<crate::Value>::into(1), Into::<crate::Value>::into("sanmu")],
        );
        assert_eq!(&sql.to_sql_string().unwrap(), r#"select * from users where users.id = ?{"Int":1} and name = ?{"String":"sanmu"}"#);
    }
}
