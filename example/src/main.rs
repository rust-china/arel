use arel::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
}
impl ArelAttributeFromRow for Gender {
    fn from_row<'r, I>(row: &'r arel::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<arel::DatabaseRow>,
    {
        let v: u8 = row.try_get(index)?;
        let ret = match v {
            1 => Gender::Male,
            2 => Gender::Female,
            _ => Gender::Unknown,
        };
        Ok(ret)
    }
}
impl From<Gender> for arel::Value {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => 1.into(),
            Gender::Female => 2.into(),
            _ => 0.into(),
        }
    }
}

#[arel(table_name = "user")]
#[allow(dead_code)]
struct User {
    #[arel(primary_key)]
    pub id: i32,
    name: String,
    #[arel(rename = "type")]
    r#type: String,
    gender: Option<Gender>,
    desc: Option<String>,
    done: Option<bool>,
    lock_version: Option<i32>,
    expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
impl Arel for User {}

async fn init_db() -> anyhow::Result<()> {
    let visitor = arel::visitor::get_or_init(|| Box::pin(async { arel::DatabasePoolOptions::new().max_connections(5).connect("sqlite::memory:").await })).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user
            (
                id             INTEGER PRIMARY KEY NOT NULL,
                name           VARCHAR(255),
                type           VARCHAR(255),
                gender         INT(1) NOT NULL DEFAULT 0,
                desc           TEXT,
                done           BOOLEAN NOT NULL DEFAULT 0,
                lock_version   INT(11) NOT NULL DEFAULT 0,
                expired_at     DATETIME
            );",
    )
    .execute(visitor.pool())
    .await?;

    User::with_transaction(|tx| {
        Box::pin(async move {
            for entry in 1i32..=100 {
                sqlx::query("INSERT INTO user (name, type) VALUES ($1, $2)")
                    .bind(format!("name-{}", entry))
                    .bind("Admin")
                    .execute(tx.as_mut())
                    .await?;
            }
            Ok(None)
        })
    })
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    init_db().await?;

    let count = User::query().select_sql("COUNT(*)").fetch_count().await?;
    println!("total: {}", count);

    let user: User = User::query().r#where("id", 5).fetch_one_as().await?;
    println!("user: {:?}", user);
    let active_user: ArelActiveUser = user.into();
    println!("active_user: {:?}", active_user);

    let arel_user: ArelUser = User::query().fetch_one_as().await?;
    println!("arel_user: {:?}", arel_user);
    let active_user: ArelActiveUser = arel_user.into();
    println!("active_user: {:?}", active_user);

    let users: Vec<User> = User::query().fetch_all_as().await?;
    println!("user: {:?}", users[0]);

    Ok(())
}
