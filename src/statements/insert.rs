use crate::prelude::Arel;
use crate::statements::ArelStatement;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
pub struct Insert<M: Arel> {
    fields: Vec<String>,
    values: Vec<crate::Value>,
    _marker: PhantomData<M>,
}

impl<M: Arel> ArelStatement for Insert<M> {
    fn to_sql(&self) -> Option<crate::Sql> {
        let table_name = M::table_name();
        let mut final_sql = crate::Sql::new(format!(r#"INSERT INTO "{}""#, table_name));

        let mut field_sql = crate::Sql::new("(");
        let mut value_sql = crate::Sql::new("(");
        let len = self.fields.len();
        for (idx, field) in self.fields.iter().enumerate() {
            let value = &self.values[idx];
            field_sql.push_str(format!(r#""{}""#, field));
            value_sql.push_str_with_prepare_value(value_sql.prepare_symbol(), value.clone());
            if idx < len - 1 {
                field_sql.push_str(", ");
                value_sql.push_str(", ");
            }
        }
        field_sql.push(')');
        value_sql.push(')');
        final_sql.push_sql(field_sql).push_str(" VALUES ").push_sql(value_sql);

        Some(final_sql)
    }
}

impl<M: Arel> Insert<M> {
    pub fn new<F: Into<String>, V: Into<crate::Value>>(fields: Vec<F>, values: Vec<V>) -> Self {
        Self {
            fields: fields.into_iter().map(|f| f.into()).collect(),
            values: values.into_iter().map(|v| v.into()).collect(),
            _marker: PhantomData::<M>,
        }
    }
}
