use std::fmt::Debug;
use std::marker::{Send, Sync};

#[async_trait::async_trait]
pub trait ArelActiveModel {
    type Model;
    type ArelModel;

    fn assign(&mut self, other: &Self) -> &mut Self;
    fn to_insert_sql(&self) -> anyhow::Result<crate::Sql>;
    fn to_update_sql(&self) -> anyhow::Result<crate::Sql>;
    fn to_destroy_sql(&self) -> anyhow::Result<crate::Sql>;
    async fn save_exec<'a, E>(&mut self, executor: E) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        E: sqlx::Executor<'a, Database = crate::Database>;
    async fn save(&mut self) -> anyhow::Result<crate::DatabaseQueryResult>;
    async fn destroy_exec<'a, E>(&mut self, executor: E) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        E: sqlx::Executor<'a, Database = crate::Database>;
    async fn destroy(&mut self) -> anyhow::Result<crate::DatabaseQueryResult>;
    async fn increment_save_exec<'a, K: Send, V: Send + Sync, F: Send, E>(&mut self, key: K, step: V, update_self_cb: F, executor: E) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        K: ToString,
        V: Debug,
        F: FnOnce(&mut Self) -> (),
        E: sqlx::Executor<'a, Database = crate::Database>;
    async fn increment_save<K: Send, V: Send + Sync, F: Send>(&mut self, key: K, step: V, update_self_cb: F) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        K: ToString,
        V: Debug,
        F: FnOnce(&mut Self) -> ();
    async fn decrement_save_exec<'a, K: Send, V: Send + Sync, F: Send, E>(&mut self, key: K, step: V, update_self_cb: F, executor: E) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        K: ToString,
        V: Debug + std::ops::Neg<Output = V>,
        F: FnOnce(&mut Self) -> (),
        E: sqlx::Executor<'a, Database = crate::Database>,
    {
        self.increment_save_exec(key, -step, update_self_cb, executor).await
    }
    async fn decrement_save<K: Send, V: Send + Sync, F: Send>(&mut self, key: K, step: V, update_self_cb: F) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        K: ToString,
        V: Debug + std::ops::Neg<Output = V>,
        F: FnOnce(&mut Self) -> (),
    {
        self.increment_save(key, -step, update_self_cb).await
    }
}
