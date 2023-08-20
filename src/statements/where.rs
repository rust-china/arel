use crate::{
    prelude::ArelModel,
    statements::{filter::Filter, ArelStatement},
};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Where<M: ArelModel>(Filter<M>);

impl<M: ArelModel> ArelStatement for Where<M> {
    fn to_sql(&self) -> Option<crate::Sql> {
        if let Some(filter_sql) = self.0.to_sql() {
            let mut final_sql = crate::Sql::new("WHERE ");
            final_sql.push_sql(filter_sql);
            Some(final_sql)
        } else {
            None
        }
    }
}

impl<M: ArelModel> Deref for Where<M> {
    type Target = Filter<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M: ArelModel> DerefMut for Where<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<M: ArelModel> Where<M> {
    pub fn new() -> Self {
        Self(Filter::<M>::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;
    #[test]
    fn to_sql() {
        struct User {}
        impl ArelBase for User {}
        impl ArelRecord for User {}
        impl ArelModel for User {}

        let mut r#where = Where::<User>::new();
        r#where.filter_and("username", "sanmu");
        assert_eq!(r#where.to_sql().unwrap().to_sql_string().unwrap(), r#"WHERE ("user"."username" = "sanmu")"#);
    }
}
