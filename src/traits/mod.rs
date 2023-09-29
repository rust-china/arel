pub mod arel_attribute_from_row;
pub mod arel_persisted;

use arel_persisted::ArelPersisted;
use std::future::Future;
use std::pin::Pin;

#[async_trait::async_trait]
pub trait SuperArel {
    fn _table_name() -> String {
        let struct_full_name = std::any::type_name::<Self>();
        regex::Regex::new(r#"((\w+)$)|(\w+<.+)"#)
            .unwrap()
            .find(&struct_full_name)
            .expect(&format!("match {} fail", struct_full_name))
            .as_str()
            .to_lowercase()
    }
    fn primary_keys() -> Vec<&'static str> {
        vec!["id"]
    }
    fn _pool() -> crate::Result<&'static sqlx::Pool<crate::db::Database>> {
        Ok(crate::db::get_pool()?)
    }
    fn primary_values(&self) -> Vec<crate::Value>;
    fn assign(&mut self, other: &Self) -> &mut Self;
    fn is_dirty(&self) -> bool;
    async fn insert_with_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn update_with_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn increment_with_exec<'a, K: Send + ToString, E>(&mut self, key: K, step: i32, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn destroy_with_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
}

#[async_trait::async_trait]
pub trait Arel: SuperArel + ArelPersisted + Send {
    fn table_name() -> String {
        Self::_table_name()
    }
    fn pool() -> crate::Result<&'static sqlx::Pool<crate::db::Database>> {
        Self::_pool()
    }
    fn query() -> crate::manager::SelectManager<Self>
    where
        Self: Sized,
    {
        crate::manager::SelectManager::<Self>::default()
    }
    async fn with_transaction<'a, F: Send>(callback: F) -> crate::Result<Option<Self>>
    where
        Self: Sized,
        for<'c> F: FnOnce(&'c mut sqlx::Transaction<'a, crate::db::Database>) -> Pin<Box<dyn Future<Output = crate::Result<Option<Self>>> + Send + 'c>>,
    {
        let pool = Self::pool()?;
        let mut tx = pool.begin().await?;
        match callback(&mut tx).await {
            Ok(model) => match tx.commit().await {
                Ok(_) => Ok(model),
                Err(e) => Err(e.into()),
            },
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
    fn validates(&mut self) -> crate::Result<()> {
        Ok(())
    }
    async fn before_save_with_tx(&mut self, _tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        Ok(())
    }
    async fn before_insert_with_tx(&mut self, _tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        Ok(())
    }
    async fn after_insert_with_tx(&mut self, _tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        Ok(())
    }
    async fn before_update_with_tx(&mut self, _tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        Ok(())
    }
    async fn after_update_with_tx(&mut self, _tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        Ok(())
    }
    async fn after_save_with_tx(&mut self, _tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        Ok(())
    }
    async fn after_save_commit(&mut self) -> crate::Result<()> {
        Ok(())
    }
    async fn save_with_tx(&mut self, tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        self.validates()?;
        self.before_save_with_tx(tx).await?;
        if self.persited() {
            self.before_update_with_tx(tx).await?;
            // update when dirty
            if self.is_dirty() {
                self.update_with_exec(tx.as_mut()).await?;
            }
            self.after_update_with_tx(tx).await?;
        } else {
            self.before_insert_with_tx(tx).await?;
            self.insert_with_exec(tx.as_mut()).await?;
            self.after_insert_with_tx(tx).await?;
        }
        self.after_save_with_tx(tx).await?;
        Ok(())
    }
    async fn save(&mut self) -> crate::Result<()>
    where
        Self: Sized,
    {
        Self::with_transaction(|tx| {
            Box::pin(async {
                self.save_with_tx(tx).await?;
                Ok(None)
            })
        })
        .await?;
        self.after_save_commit().await?;
        Ok(())
    }
    async fn decrement_with_exec<'a, K: Send + ToString, E>(&mut self, key: K, step: i32, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        self.increment_with_exec(key, step * -1, executor).await
    }
    async fn increment<K: Send + ToString>(&mut self, key: K, step: i32) -> crate::Result<()> {
        self.increment_with_exec(key, step, Self::pool()?).await
    }
    async fn decrement<K: Send + ToString>(&mut self, key: K, step: i32) -> crate::Result<()> {
        self.increment(key, step * -1).await
    }
    async fn before_destroy_with_tx(&mut self, _tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        Ok(())
    }
    async fn after_destroy_with_tx(&mut self, _tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        Ok(())
    }
    async fn after_destroy_commit(&mut self) -> crate::Result<()> {
        Ok(())
    }
    async fn destroy_with_tx(&mut self, tx: &mut sqlx::Transaction<'_, crate::db::Database>) -> crate::Result<()> {
        self.before_destroy_with_tx(tx).await?;
        self.destroy_with_exec(tx.as_mut()).await?;
        self.after_destroy_with_tx(tx).await?;
        Ok(())
    }
    async fn destroy(&mut self) -> crate::Result<()>
    where
        Self: Sized,
    {
        Self::with_transaction(|tx| {
            Box::pin(async {
                self.destroy_with_tx(tx).await?;
                Ok(None)
            })
        })
        .await?;
        self.after_destroy_commit().await?;
        Ok(())
    }
}
