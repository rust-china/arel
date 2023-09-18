pub mod arel_attribute_from_row;

use std::future::Future;
use std::pin::Pin;

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
    fn primary_values(&self) -> Vec<crate::Value>;
    fn _pool() -> crate::Result<&'static sqlx::Pool<crate::db::Database>> {
        Ok(crate::db::get_pool()?)
    }
}

#[async_trait::async_trait]
pub trait Arel: SuperArel {
    fn table_name() -> String {
        Self::_table_name()
    }
    fn pool() -> crate::Result<&'static sqlx::Pool<crate::db::Database>> {
        Self::_pool()
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
}
