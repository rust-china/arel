use crate::{
    prelude::ArelBase,
    statements::{filter::Filter, ArelStatement},
};
use std::ops::{Deref, DerefMut};

pub struct Having<M: ArelBase>(Filter<M>);

impl<M: ArelBase> ArelStatement for Having<M> {
    fn to_sql(&self) -> Option<crate::Sql> {
        if let Some(filter_sql) = self.0.to_sql() {
            let mut final_sql = crate::Sql::new("HAVING ");
            final_sql.push_sql(filter_sql);
            Some(final_sql)
        } else {
            None
        }
    }
}

impl<M: ArelBase> Deref for Having<M> {
    type Target = Filter<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M: ArelBase> DerefMut for Having<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<M: ArelBase> Having<M> {
    pub fn new() -> Self {
        Self(Filter::<M>::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::ArelBase;

    use super::*;
    #[test]
    fn to_sql() {
        struct User {}
        impl ArelBase for User {}

        let mut having = Having::<User>::new();
        having.filter_and("username", "sanmu");
        assert_eq!(having.to_sql().unwrap().to_sql_string().unwrap(), r#"HAVING ("user"."username" = "sanmu")"#);
    }
}
