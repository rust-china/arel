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
            crate::Value::Bool(val) => match val {
                Some(v) => {
                    if *v {
                        self.push_bind(Some(1));
                    } else {
                        self.push_bind(Some(0));
                    }
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::TinyInt(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v));
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::SmallInt(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v));
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::Int(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v));
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::BigInt(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v));
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::TinyUnsigned(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v as i32));
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::SmallUnsigned(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v as i32));
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::Unsigned(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v as i32));
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::BigUnsigned(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v as i32));
                }
                None => {
                    self.push_bind(Option::<i32>::None);
                }
            },
            crate::Value::Float(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v));
                }
                None => {
                    self.push_bind(Option::<f32>::None);
                }
            },
            crate::Value::Double(val) => match val {
                Some(v) => {
                    self.push_bind(Some(*v));
                }
                None => {
                    self.push_bind(Option::<f64>::None);
                }
            },
            crate::Value::Char(val) => match val {
                Some(v) => {
                    self.push_bind(Some(v.to_string()));
                }
                None => {
                    self.push_bind(Option::<String>::None);
                }
            },
            crate::Value::String(val) => match val {
                Some(v) => {
                    self.push_bind(v.to_string());
                }
                None => {
                    self.push_bind(Option::<String>::None);
                }
            },
            crate::Value::Bytes(_) => {
                return Err(anyhow::anyhow!("Value::Bytes type not allow to bind value."));
            }
            crate::Value::Array(_) => {
                return Err(anyhow::anyhow!("Value::Array type not allow to bind value."));
            }
            #[cfg(feature = "with-json")]
            crate::Value::Json(val) => match val {
                Some(json) => {
                    self.push_bind(*json.clone());
                }
                None => {
                    self.push_bind(Option::<serde_json::Value>::None);
                }
            },
            #[cfg(feature = "with-chrono")]
            crate::Value::ChronoDateTime(val) => match val {
                Some(v) => {
                    self.push_bind(v.to_string());
                }
                None => {
                    self.push_bind(Option::<chrono::DateTime<chrono::Utc>>::None);
                }
            },
        };
        Ok(self)
    }
}
