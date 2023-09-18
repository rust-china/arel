use crate::{
    prelude::Arel,
    statements::{filter::Filter, ArelStatement},
};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Where<M: Arel>(Filter<M>);

impl<M: Arel> ArelStatement for Where<M> {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>> {
        if let Some(filter_sql) = self.0.to_sql()? {
            let mut final_sql = crate::Sql::new("WHERE ");
            final_sql.push_sql(filter_sql);
            Ok(Some(final_sql))
        } else {
            Ok(None)
        }
    }
}

impl<M: Arel> Deref for Where<M> {
    type Target = Filter<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<M: Arel> DerefMut for Where<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// # Examples
/// #[cfg(any(feature = "sqlite", feature = "mysql"))]
/// ```
/// use arel::prelude::*;
/// use arel::statements::Where;
/// #[arel]
/// struct User {}
/// impl Arel for User {}
/// let mut r#where = Where::<User>::default();
/// r#where.and_filter("username", "sanmu");
/// #[cfg(any(feature = "sqlite", feature = "mysql"))]
/// assert_eq!(r#where.to_sql().unwrap().unwrap().to_sql_string().unwrap(), r#"WHERE "user"."username" = ?"#);
/// ```
impl<M: Arel> Default for Where<M> {
    fn default() -> Self {
        Self(Filter::<M>::default())
    }
}
