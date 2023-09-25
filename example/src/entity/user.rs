use arel::prelude::*;
#[arel_enum]
pub enum Gender {
    #[arel_enum(value = 0, default = true)]
    Unknown,
    #[arel_enum(value = 1)]
    Male,
    #[arel_enum(value = 2)]
    Female,
}

#[arel_enum]
pub enum Type {
    #[arel_enum(value = "USER", default = true)]
    User,
    #[arel_enum(value = "ADMIN")]
    Admin,
}

#[arel(table_name = "users")]
pub struct User {
    #[arel(primary_key)]
    pub id: i32,
    #[arel(primary_key)]
    pub name: String,
    pub age: Option<i32>,
    pub gender: Option<Gender>,
    pub r#type: Type,
    pub address: Option<String>,
    pub expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl Arel for User {}

pub async fn init_db() -> arel::Result<()> {
    let visitor = arel::db::visitor::get_or_init(|| Box::pin(async { arel::db::DatabasePoolOptions::new().max_connections(5).connect("sqlite::memory:").await })).await?;
    arel::sqlx::query(
        "CREATE TABLE IF NOT EXISTS users
					(
							id             INTEGER PRIMARY KEY NOT NULL,
							name           VARCHAR(255) NOT NULL,
							age   		   INT(11),
							gender         INT(1) NOT NULL DEFAULT 0,
							type           VARCHAR(255) NOT NULL default 'ADMIN',
							address        VARCHAR(255),
							expired_at     DATETIME,
                            created_at     DATETIME DEFAULT CURRENT_TIMESTAMP
					);",
    )
    .execute(visitor.pool())
    .await?;

    User::with_transaction(|tx| {
        Box::pin(async move {
            for entry in 1i32..=100 {
                sqlx::query("INSERT INTO users (name) VALUES ($1)").bind(format!("name-{}", entry)).execute(tx.as_mut()).await?;
            }
            Ok(None)
        })
    })
    .await?;

    Ok(())
}
