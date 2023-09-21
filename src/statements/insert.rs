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
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        let table_name = M::table_name();
        let mut final_sql = crate::Sql::new(format!(r#"INSERT INTO "{}" "#, table_name));

        let mut field_sql = crate::Sql::default();
        field_sql.push_str("(").push_strs(self.fields.iter().collect(), ", ").push_str(")");
        let mut value_sql = crate::Sql::default();
        value_sql.push_str("(").push_binds(self.values.iter().collect(), ", ").push_str(")");
        final_sql.push_sql(field_sql).push_str(" VALUES ").push_sql(value_sql);

        final_sql.push_str(" RETURNING *");
        Ok(Some(final_sql))
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
