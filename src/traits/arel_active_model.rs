#[async_trait::async_trait]
pub trait ArelActiveModel {
    type Model;
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
    async fn increment_exec<'a, K: ToString + Send, F: Send, E>(&mut self, key: K, step: i32, update_self_cb: F, executor: E) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        F: FnOnce(&mut Self, i32) -> (),
        E: sqlx::Executor<'a, Database = crate::Database>;
    async fn increment<K: ToString + Send, F: Send>(&mut self, key: K, step: i32, update_self_cb: F) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        F: FnOnce(&mut Self, i32) -> ();
    async fn decrement_exec<'a, K: ToString + Send, F: Send, E>(&mut self, key: K, step: i32, update_self_cb: F, executor: E) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        F: FnOnce(&mut Self, i32) -> (),
        E: sqlx::Executor<'a, Database = crate::Database>,
    {
        self.increment_exec(key, -step, update_self_cb, executor).await
    }
    async fn decrement<K: ToString + Send, F: Send>(&mut self, key: K, step: i32, update_self_cb: F) -> anyhow::Result<crate::DatabaseQueryResult>
    where
        F: FnOnce(&mut Self, i32) -> (),
    {
        self.increment(key, -step, update_self_cb).await
    }
}
