use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct QueryBuilder<'a>(sqlx::QueryBuilder<'a, crate::db::Database>);

impl<'a> Deref for QueryBuilder<'a> {
    type Target = sqlx::QueryBuilder<'a, crate::db::Database>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for QueryBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryFrom<&super::Sql> for QueryBuilder<'a> {
    type Error = crate::Error;
    fn try_from(sql: &super::Sql) -> Result<Self, Self::Error> {
        let mut query_builder = QueryBuilder::default();
        let mut handle_start_index = 0;
        for (idx, replace_index) in sql.bind_indexs.iter().enumerate() {
            let replace_index = *replace_index;
            let idx_value = &sql.bind_values[idx];
            query_builder.push(&sql.raw_value[handle_start_index..replace_index]);
            query_builder.push_bind_arel_value(idx_value)?;
            handle_start_index = replace_index + 1;
        }
        if handle_start_index < sql.raw_value.len() {
            query_builder.push(&sql.raw_value[handle_start_index..]);
        }
        Ok(query_builder)
    }
}

impl<'a> QueryBuilder<'a> {
    pub fn push_bind_arel_value(&mut self, value: &crate::Value) -> crate::Result<&mut Self> {
        match value {
            crate::Value::Bool(val) => {
                self.push_bind(val.0);
            }
            crate::Value::TinyInt(val) => {
                self.push_bind(val.0);
            }
            crate::Value::SmallInt(val) => {
                self.push_bind(val.0);
            }
            crate::Value::Int(val) => {
                self.push_bind(val.0);
            }
            crate::Value::BigInt(val) => {
                self.push_bind(val.0);
            }
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::TinyUnsigned(val) => {
                self.push_bind(val.0);
            }
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::SmallUnsigned(val) => {
                self.push_bind(val.0);
            }
            #[cfg(any(feature = "sqlite", feature = "mysql"))]
            crate::Value::Unsigned(val) => {
                self.push_bind(val.0);
            }
            #[cfg(any(feature = "mysql"))]
            crate::Value::BigUnsigned(val) => {
                self.push_bind(val.0);
            }
            crate::Value::Float(val) => {
                self.push_bind(val.0);
            }
            crate::Value::Double(val) => {
                self.push_bind(val.0);
            }
            crate::Value::String(val) => {
                self.push_bind(val.0.clone());
            }
            crate::Value::Bytes(val) => {
                let bytes: Option<Vec<u8>> = val.as_ref().map(|v| v.clone().into());
                self.push_bind(bytes);
            }
            crate::Value::Array(_) => {
                return Err(anyhow::anyhow!("Value::Array type not allow to bind value.").into());
            }
            #[cfg(feature = "with-json")]
            crate::Value::Json(val) => {
                self.push_bind(val.0.clone());
            }
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoTimestamp(val) => match &val.0 {
                Some(value) => {
                    let value: chrono::DateTime<chrono::Utc> = value.clone().into();
                    self.push_bind(Some(value));
                }
                None => {
                    self.push_bind(Option::<Option<chrono::DateTime<chrono::Utc>>>::None);
                }
            },
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoDateTime(val) => {
                self.push_bind(val.0);
            }
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoDate(val) => {
                self.push_bind(val.0);
            }
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoTime(val) => {
                self.push_bind(val.0);
            }
        };
        Ok(self)
    }
}
