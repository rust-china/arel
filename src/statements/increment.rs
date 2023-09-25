use crate::prelude::Arel;
use crate::statements::ArelStatement;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct Increment<M: Arel> {
    field: String,
    step: i32,
    where_fields: Vec<String>,
    where_values: Vec<crate::Value>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Increment<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        let table_name = M::table_name();
        let mut final_sql = crate::Sql::new(format!(r#"UPDATE "{}" SET "{}" = COALESCE({}, 0) + ({})"#, table_name, self.field, self.field, self.step));

        final_sql.push_str(" WHERE ");
        let len = self.where_fields.len();
        for (idx, field) in self.where_fields.iter().enumerate() {
            let value = &self.where_values[idx];
            final_sql.push_str_with_bind(format!(r#""{}" = "#, field), value.clone());
            if idx < len - 1 {
                final_sql.push_str(" AND ");
            }
        }
        final_sql.push_str(" RETURNING *");
        Ok(Some(final_sql))
    }
}

impl<M: Arel> Increment<M> {
    pub fn new<F: Into<String>, V: Into<crate::Value>>(field: String, step: i32, where_fields: Vec<F>, where_values: Vec<V>) -> Self {
        Self {
            field,
            step,
            where_fields: where_fields.into_iter().map(|f| f.into()).collect(),
            where_values: where_values.into_iter().map(|v| v.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
}
