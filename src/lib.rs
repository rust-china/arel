pub mod manager;
pub mod prelude;
pub mod sql;
pub mod statements;
pub mod traits;
pub mod value;

pub use bytes::Bytes;
pub use sql::Sql;
pub use value::{ActiveValue, Value};

pub enum SortType {
    Asc,
    Desc,
}
impl std::fmt::Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SortType::Asc => write!(f, "ASC"),
            SortType::Desc => write!(f, "DESC"),
        }
    }
}

pub enum JoinType {
    LeftJoin,
    InnerJoin,
    RightJoin,
    FullOuterJoin,
    CrossJoin,
}
impl std::fmt::Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JoinType::LeftJoin => write!(f, "LEFT JOIN"),
            JoinType::InnerJoin => write!(f, "INNER JOIN"),
            JoinType::RightJoin => write!(f, "RIGHT JOIN"),
            JoinType::FullOuterJoin => write!(f, "FULL OUTER JOIN"),
            JoinType::CrossJoin => write!(f, "CROSS JOIN"),
        }
    }
}

/// Defines a set operation on an [ActiveValue]
///
/// # Examples
///
/// ```
/// use arel::prelude::*;
/// let active_value = Change(1);
/// assert_eq!(active_value, ActiveValue::Changed(1, None));
///
/// ```
#[allow(non_snake_case)]
pub fn Change<V>(v: V) -> ActiveValue<V>
where
    V: Into<Value> + Clone,
{
    ActiveValue::set(v)
}
