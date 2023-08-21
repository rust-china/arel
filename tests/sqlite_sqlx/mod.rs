use arel::prelude::*;

#[allow(dead_code)]
#[arel]
struct User {
    id: i32,
    name: String,
    desc: Option<String>,
    done: Option<bool>,
    lock_version: Option<i32>,
    expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
impl ArelModel for User {}

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
            // let inner_tx = tx.begin().await?;
            for entry in 1i32..=100 {
                sqlx::query("INSERT INTO user (name) VALUES ($1)").bind(format!("name-{}", entry)).execute(tx.as_mut()).await?;
            }
            Ok(None)
        })
    })
    .await?;

    // rollback
    let _ = User::with_transaction(|tx| {
        Box::pin(async move {
            // let inner_tx = tx.begin().await?;
            for entry in 101i32..=200 {
                sqlx::query("INSERT INTO user (name) VALUES ($1)").bind(format!("name-{}", entry)).execute(tx.as_mut()).await?;
            }
            Err(anyhow::anyhow!("rollback"))
        })
    })
    .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::Row;

    use super::*;

    #[tokio::test]
    async fn test_visitor() -> anyhow::Result<()> {
        init_db().await?;

        test_query().await?;

        Ok(())
    }
    async fn test_query() -> anyhow::Result<()> {
        let row: (i64,) = sqlx::query_as("SELECT $1").bind(150_i64).fetch_one(User::pool()?).await?;
        assert_eq!(row.0, 150);

        let row = User::query().select_sql("COUNT(*) as count").fetch_one().await?;
        assert_eq!(row.try_get::<i64, _>("count")?, 100);

        let row: (i64,) = User::query().select_sql("COUNT(*)").fetch_one_as().await?;
        assert_eq!(row.0, 100);

        let user: User = User::query().fetch_one_as().await?;
        assert_eq!(user.id, 1);
        let user: User = User::query().r#where("id", 10).fetch_one_as().await?;
        assert_eq!(user.id, 10);

        let users: Vec<User> = User::query().fetch_all_as().await?;
        assert_eq!(users.len(), 100);
        let users: Vec<User> = User::query().r#where("id", vec![1, 2, 3]).fetch_all_as().await?;
        assert_eq!(users.len(), 3);
        let users: Vec<User> = User::query().where_range("id", ..=10).fetch_all_as().await?;
        assert_eq!(users.len(), 10);
        let users: Vec<User> = User::query().paginate(2, 10).fetch_all_as().await?;
        assert_eq!(users.len(), 10);
        assert_eq!(users[0].id, 11);

        let user: User = User::query().r#where("name", "name-5").fetch_one_as().await?;
        assert_eq!(user.id, 5);

        let users: Vec<User> = User::query().r#where("name", vec!["name-5", "name-6"]).fetch_all_as().await?;
        assert_eq!(users.len(), 2);

        Ok(())
    }
}
