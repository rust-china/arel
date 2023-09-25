#[cfg(not(any(feature = "sqlite", feature = "mysql", feature = "postgres")))]
compile_error!("`sqlite`, `mysql` or `postgres` should be enable one.");
#[cfg(all(feature = "sqlite", feature = "mysql"))]
compile_error!("feature `sqlite` and `mysql` shouldn't be enabled both.");
#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("feature `sqlite` and `postgres` shouldn't be enabled both.");
#[cfg(all(feature = "mysql", feature = "postgres"))]
compile_error!("feature `mysql` and `postgres` shouldn't be enabled both.");

pub mod statements;

pub mod db;
pub mod error;
pub mod manager;
pub mod prelude;
pub mod sql;
pub mod traits;
pub mod value;
pub use async_trait;

pub use arel_macros::{self, arel, arel_enum};
pub use bytes;
pub use chrono;
pub use serde_json;
pub use sqlx;

pub use bytes::Bytes;
pub use error::Error;
pub use manager::SelectManager;
pub use sql::Sql;
pub use statements::{join::JoinConst, order::SortConst};
pub use value::{
    active_vulue::{ActiveValue, Set},
    sub_value, Value,
};

pub use traits::{arel_attribute_from_row::ArelAttributeFromRow, arel_model::ArelModel, arel_persisted::ArelPersisted, Arel, SuperArel};
pub type Result<T> = std::result::Result<T, crate::Error>;
