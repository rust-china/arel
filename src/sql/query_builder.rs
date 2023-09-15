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

impl<'a> QueryBuilder<'a> {
    pub fn push_bind_arel_value<T>(&mut self, value: &crate::Value) -> crate::Result<&mut Self>
    where
        T: for<'r> sqlx::Decode<'r, crate::db::Database> + sqlx::Encode<'a, crate::db::Database> + sqlx::Type<crate::db::Database> + Send + 'a,
    {
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
            crate::Value::Bytes(_) => {
                return Err(anyhow::anyhow!("Value::Bytes type not allow to bind value.").into());
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
