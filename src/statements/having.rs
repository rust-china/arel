use crate::{
    prelude::Arel,
    statements::{filter::Filter, ArelStatement},
};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Having<M: Arel>(Filter<M>);

impl<M: Arel> ArelStatement for Having<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        if let Some(filter_sql) = self.0.to_sql()? {
            let mut final_sql = crate::Sql::new("HAVING ");
            final_sql.push_sql(filter_sql);
            Ok(Some(final_sql))
        } else {
            Ok(None)
        }
    }
}

impl<M: Arel> Deref for Having<M> {
    type Target = Filter<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M: Arel> DerefMut for Having<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// # Examples
///
/// ```
/// use arel::prelude::*;
/// use arel::statements::Having;
/// #[arel]
/// struct User {}
/// impl Arel for User {}
/// let mut having = Having::<User>::default();
/// having.and_filter("username", "sanmu");
/// #[cfg(any(feature = "sqlite", feature = "mysql"))]
/// assert_eq!(having.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"HAVING "user"."username" = ?"#);
/// ```
impl<M: Arel> Default for Having<M> {
    fn default() -> Self {
        Self(Filter::<M>::default())
    }
}
