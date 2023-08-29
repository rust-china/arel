use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct QueryBuilder<'a>(sqlx::QueryBuilder<'a, crate::Database>);

impl<'a> Deref for QueryBuilder<'a> {
    type Target = sqlx::QueryBuilder<'a, crate::Database>;
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
    pub fn push_bind_prepare_value(&mut self, value: &crate::Value) -> anyhow::Result<&mut Self> {
        match value {
            crate::Value::Bool(val) => {
                if *val {
                    self.push_bind(Some(1));
                } else {
                    self.push_bind(Some(0));
                }
            }
            crate::Value::TinyInt(val) => {
                self.push_bind(Some(*val));
            }
            crate::Value::SmallInt(val) => {
                self.push_bind(Some(*val));
            }
            crate::Value::Int(val) => {
                self.push_bind(Some(*val));
            }
            crate::Value::BigInt(val) => {
                self.push_bind(Some(*val));
            }
            crate::Value::TinyUnsigned(val) => {
                self.push_bind(Some(*val as i32));
            }
            crate::Value::SmallUnsigned(val) => {
                self.push_bind(Some(*val as i32));
            }
            crate::Value::Unsigned(val) => {
                self.push_bind(Some(*val as i32));
            }
            crate::Value::BigUnsigned(val) => {
                self.push_bind(Some(*val as i32));
            }
            crate::Value::Float(val) => {
                self.push_bind(Some(*val));
            }
            crate::Value::Double(val) => {
                self.push_bind(Some(*val));
            }
            crate::Value::Char(val) => {
                self.push_bind(Some(val.to_string()));
            }
            crate::Value::String(val) => {
                self.push_bind(Some(val.to_string()));
            }
            crate::Value::Bytes(_) => {
                return Err(anyhow::anyhow!("Value::Bytes type not allow to bind value."));
            }
            crate::Value::Array(_) => {
                return Err(anyhow::anyhow!("Value::Array type not allow to bind value."));
            }
            #[cfg(feature = "with-json")]
            crate::Value::Json(val) => {
                self.push_bind(Some(*val.clone()));
            }
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoDateTime(val) => {
                self.push_bind(Some(val.to_string()));
            }
            crate::Value::Null => {
                // FIXME: all null use bool type bind
                self.push_bind(Option::<bool>::None);
            }
        };
        Ok(self)
    }
}
