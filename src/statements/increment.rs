use crate::prelude::Arel;
use crate::statements::ArelStatement;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct Increment<M: Arel> {
    field: String,
    by: i32,
    where_fields: Vec<String>,
    where_values: Vec<crate::Value>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Increment<M> {
    fn to_sql(&self) -> Option<crate::Sql> {
        let table_name = M::table_name();
        let full_column_name = format!(r#""{}"."{}""#, table_name, self.field);
        let mut final_sql = crate::Sql::new(format!(r#"UPDATE "{}" SET "{}" = COALESCE({}, 0)"#, table_name, self.field, full_column_name));
        if self.by >= 0 {
            final_sql.push_str(&format!(" + {}", self.by));
        } else {
            final_sql.push_str(&format!(" - {}", self.by.abs()));
        }

        final_sql.push_str(" WHERE ");
        let len = self.where_fields.len();
        for (idx, field) in self.where_fields.iter().enumerate() {
            let value = &self.where_values[idx];
            let mut sql = crate::Sql::default();
            sql.push_str_with_prepare_value(format!(r#""{}" = {}"#, field, sql.prepare_symbol()), value.clone());
            final_sql.push_sql(sql);
            if idx < len - 1 {
                final_sql.push_str(" AND ");
            }
        }

        Some(final_sql)
    }
}

impl<M: Arel> Increment<M> {
    pub fn new<F: Into<String>, V: Into<crate::Value>>(field: String, by: i32, where_fields: Vec<F>, where_values: Vec<V>) -> Self {
        Self {
            field,
            by,
            where_fields: where_fields.into_iter().map(|f| f.into()).collect(),
            where_values: where_values.into_iter().map(|v| v.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
}
