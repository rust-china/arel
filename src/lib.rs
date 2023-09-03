#[cfg(not(any(feature = "sqlite", feature = "mysql", feature = "postgres")))]
compile_error!("`sqlite`, `mysql` or `postgres` should be enable one.");

#[cfg(all(feature = "sqlite", feature = "mysql"))]
compile_error!("feature `sqlite` and `mysql` shouldn't be enabled both.");
#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("feature `sqlite` and `postgres` shouldn't be enabled both.");
#[cfg(all(feature = "mysql", feature = "postgres"))]
compile_error!("feature `mysql` and `postgres` shouldn't be enabled both.");

pub use anyhow;
pub use async_trait;
pub use chrono;
pub use sqlx;

pub use arel_macros::{arel, arel_attribute};

pub mod manager;
pub mod prelude;
pub mod sql;
pub mod statements;
pub mod traits;
pub mod value;
pub mod visitor;

pub use crate::traits::{Arel, ArelActiveModel, ArelAttributeFromRow, ArelPersisted, SuperArel};
pub use bytes::Bytes;
pub use sql::Sql;
pub use value::{ActiveValue, Value};

#[cfg(feature = "sqlite")]
pub type Database = sqlx::sqlite::Sqlite;
#[cfg(feature = "sqlite")]
pub type DatabaseConnection = sqlx::sqlite::SqliteConnection;
#[cfg(feature = "sqlite")]
pub type DatabasePool = sqlx::sqlite::SqlitePool;
#[cfg(feature = "sqlite")]
pub type DatabaseRow = sqlx::sqlite::SqliteRow;
#[cfg(feature = "sqlite")]
pub type DatabaseQueryResult = sqlx::sqlite::SqliteQueryResult;
#[cfg(feature = "sqlite")]
pub type DatabasePoolOptions = sqlx::sqlite::SqlitePoolOptions;

#[cfg(feature = "mysql")]
pub type Database = sqlx::mysql::MySql;
#[cfg(feature = "mysql")]
pub type DatabaseConnection = sqlx::mysql::MySqlConnection;
#[cfg(feature = "mysql")]
pub type DatabasePool = sqlx::mysql::MySqlPool;
#[cfg(feature = "mysql")]
pub type DatabaseRow = sqlx::mysql::MySqlRow;
#[cfg(feature = "mysql")]
pub type DatabaseQueryResult = sqlx::mysql::MySqlQueryResult;
#[cfg(feature = "mysql")]
pub type DatabasePoolOptions = sqlx::mysql::MySqlPoolOptions;

#[cfg(feature = "postgres")]
pub type Database = sqlx::Postgres;
#[cfg(feature = "postgres")]
pub type DatabaseConnection = sqlx::postgres::PgConnection;
#[cfg(feature = "postgres")]
pub type DatabasePool = sqlx::PgPool;
#[cfg(feature = "postgres")]
pub type DatabaseRow = sqlx::postgres::PgRow;
#[cfg(feature = "postgres")]
pub type DatabaseQueryResult = sqlx::postgres::PgQueryResult;
#[cfg(feature = "postgres")]
pub type DatabasePoolOptions = sqlx::postgres::PgPoolOptions;

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

#[allow(non_snake_case)]
pub fn Set<V, UV: Into<V>>(uv: UV) -> ActiveValue<V>
where
    V: Into<Value> + Clone + PartialEq,
{
    ActiveValue::Changed(uv.into(), Box::new(ActiveValue::NotSet))
}
