use crate::prelude::Arel;
use crate::statements::ArelStatement;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct Delete<M: Arel> {
    where_fields: Vec<String>,
    where_values: Vec<crate::Value>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Delete<M> {
    fn to_sql(&self) -> Option<crate::Sql> {
        let table_name = M::table_name();
        let mut final_sql = crate::Sql::new(format!(r#"DELETE FROM "{}" WHERE "#, table_name));

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

impl<M: Arel> Delete<M> {
    pub fn new<F: Into<String>, V: Into<crate::Value>>(where_fields: Vec<F>, where_values: Vec<V>) -> Self {
        Self {
            where_fields: where_fields.into_iter().map(|f| f.into()).collect(),
            where_values: where_values.into_iter().map(|v| v.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
}
