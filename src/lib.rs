#[cfg(not(any(feature = "sqlite", feature = "mysql", feature = "postgres")))]
compile_error!("`sqlite`, `mysql` or `postgres` should be enable one.");
#[cfg(all(feature = "sqlite", feature = "mysql"))]
compile_error!("feature `sqlite` and `mysql` shouldn't be enabled both.");
#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("feature `sqlite` and `postgres` shouldn't be enabled both.");
#[cfg(all(feature = "mysql", feature = "postgres"))]
compile_error!("feature `mysql` and `postgres` shouldn't be enabled both.");

pub mod db;
pub mod error;
pub mod prelude;
pub mod sql;
pub mod traits;
pub mod value;

pub use arel_macros::arel;
pub use bytes;
pub use chrono;
pub use serde_json;
pub use sqlx;

pub use bytes::Bytes;
pub use error::Error;
pub use value::{sub_value, Value};

pub use traits::{arel_attribute_from_row::ArelAttributeFromRow, Arel, SuperArel};
pub type Result<T> = std::result::Result<T, crate::Error>;
