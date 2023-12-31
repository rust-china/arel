use crate::prelude::Arel;
use crate::statements::ArelStatement;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct Update<M: Arel> {
    fields: Vec<String>,
    values: Vec<crate::Value>,
    where_fields: Vec<String>,
    where_values: Vec<crate::Value>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Update<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        let table_name = M::table_name();
        let mut final_sql = crate::Sql::new(format!(r#"UPDATE "{}" SET "#, table_name));

        let len = self.fields.len();
        for (idx, field) in self.fields.iter().enumerate() {
            let value = &self.values[idx];
            final_sql.push_str_with_bind(format!(r#""{}" = "#, field), value.clone());
            if idx < len - 1 {
                final_sql.push_str(", ");
            }
        }

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

impl<M: Arel> Update<M> {
    pub fn new<F: Into<String>, V: Into<crate::Value>>(fields: Vec<F>, values: Vec<V>, where_fields: Vec<F>, where_values: Vec<V>) -> Self {
        Self {
            fields: fields.into_iter().map(|f| f.into()).collect(),
            values: values.into_iter().map(|v| v.into()).collect(),
            where_fields: where_fields.into_iter().map(|f| f.into()).collect(),
            where_values: where_values.into_iter().map(|v| v.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
}
