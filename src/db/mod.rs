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

pub mod visitor;

pub fn get_pool() -> crate::Result<&'static sqlx::Pool<Database>> {
	Ok(visitor::get()?.pool())
}