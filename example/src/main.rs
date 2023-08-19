use arel::prelude::*;

#[arel]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    desc: Option<String>,
    done: Option<bool>,
    lock_version: Option<i32>,
    expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
// impl ArelModel for User {}

async fn init_db() -> anyhow::Result<()> {
    let visitor = arel::visitor::get_or_init(|| Box::pin(async { arel::DatabasePoolOptions::new().max_connections(5).connect("sqlite::memory:").await })).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user
            (
                id             INTEGER PRIMARY KEY NOT NULL,
                name           VARCHAR(255),
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
            for entry in 0i32..100 {
                sqlx::query("INSERT INTO user (name) VALUES ($1)").bind(format!("name-{}", entry)).execute(tx.as_mut()).await?;
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

    let row: (i64,) = User::query().select_sql("COUNT(*)").fetch_one_as().await?;
    println!("total: {}", row.0);

    let user: User = User::query().fetch_one_as().await?;
    println!("user: {:?}", user);

    let users: Vec<User> = User::query().fetch_all_as().await?;
    println!("user: {:?}", users[0]);

    Ok(())
}
