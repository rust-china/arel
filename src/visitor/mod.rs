use once_cell::sync::OnceCell;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

#[derive(Debug)]
pub struct Visitor<DB: sqlx::Database>(sqlx::Pool<DB>);
impl<DB: sqlx::Database> Deref for Visitor<DB> {
    type Target = sqlx::Pool<DB>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<DB: sqlx::Database> Visitor<DB> {
    pub fn new(pool: sqlx::Pool<DB>) -> Self {
        Self(pool)
    }
    pub fn pool(&self) -> &sqlx::Pool<DB> {
        &self.0
    }
    // pub async fn with_transaction<'a, T: crate::ArelModel, F: Send>(&'a self, callback: F) -> anyhow::Result<Option<T>>
    // where
    //     for<'c> F: FnOnce(&'c mut sqlx::Transaction<'a, crate::Database>) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<T>>> + Send + 'c>>,
    // {
    //     let mut tx = self.pool().begin().await?;
    //     // let inner_tx = sqlx::Transaction::<'_, crate::Database>::begin(tx).await?;
    //     match callback(&mut tx).await {
    //         Ok(model) => match tx.commit().await {
    //             Ok(_) => Ok(model),
    //             Err(e) => Err(anyhow::anyhow!(e.to_string())),
    //         },
    //         Err(e) => {
    //             tx.rollback().await?;
    //             Err(e)
    //         }
    //     }
    // }
}

pub static VISITOR: OnceCell<Visitor<crate::Database>> = OnceCell::new();
pub async fn init() -> anyhow::Result<&'static Visitor<crate::Database>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let database_max_connection = {
        let default_max_connection = 10;
        match std::env::var("DATABASE_MAX_CONNECTION") {
            Ok(max_connection) => match max_connection.trim().parse::<u32>() {
                Ok(max_connection) => max_connection,
                _ => default_max_connection,
            },
            _ => default_max_connection,
        }
    };
    get_or_init(|| {
        Box::pin(async move {
            // sqlx::sqlite::install_default_drivers();
            crate::DatabasePoolOptions::new().max_connections(database_max_connection).connect(database_url.trim()).await
        })
    })
    .await
}
pub async fn get_or_init<F>(callback: F) -> anyhow::Result<&'static Visitor<crate::Database>>
where
    F: FnOnce() -> Pin<Box<dyn Future<Output = Result<crate::DatabasePool, sqlx::Error>>>>,
{
    if VISITOR.get().is_none() {
        let pool = callback().await?;
        if let Err(_) = VISITOR.set(Visitor(pool)) {
            return Err(anyhow::anyhow!("Set Visitor Failed"));
        }
    }
    match VISITOR.get() {
        Some(visitor) => Ok(visitor),
        None => Err(anyhow::anyhow!("Get Visitor Failed")),
    }
}
pub fn get() -> anyhow::Result<&'static Visitor<crate::Database>> {
    match VISITOR.get() {
        Some(visitor) => Ok(visitor),
        None => Err(anyhow::anyhow!("Visitor Is Emplty")),
    }
}

#[cfg(test)]
#[cfg(feature = "sqlite")]
mod tests {
    use super::*;
    use sqlx::Executor;

    #[tokio::test]
    async fn test_visitor() {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        let visitor = init().await;
        assert!(visitor.is_ok());

        let visitor = get();
        assert!(visitor.is_ok());

        let create_sql = "CREATE TABLE IF NOT EXISTS user (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(255))";
        assert!(visitor.unwrap().pool().execute(create_sql).await.is_ok());
    }
}
