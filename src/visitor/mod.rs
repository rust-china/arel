use once_cell::sync::OnceCell;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

#[derive(Debug)]
pub struct Visitor(crate::DatabasePool);
impl Deref for Visitor {
    type Target = crate::DatabasePool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Visitor {
    pub fn new(pool: crate::DatabasePool) -> Self {
        Self(pool)
    }
    pub fn pool(&self) -> &crate::DatabasePool {
        &self.0
    }
}

pub static VISITOR: OnceCell<Visitor> = OnceCell::new();
pub async fn init() -> anyhow::Result<&'static Visitor> {
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
pub async fn get_or_init<F>(callback: F) -> anyhow::Result<&'static Visitor>
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
pub fn get() -> anyhow::Result<&'static Visitor> {
    match VISITOR.get() {
        Some(visitor) => Ok(visitor),
        None => Err(anyhow::anyhow!("Visitor Is Emplty")),
    }
}

#[cfg(test)]
#[cfg(feature = "sqlite")]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_visitor() {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        let visitor = init().await;
        assert!(visitor.is_ok());

        let visitor = get();
        assert!(visitor.is_ok());
    }
}
