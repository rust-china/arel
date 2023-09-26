use crate::ArelPersisted;

#[async_trait::async_trait]
pub trait ArelModel: ArelPersisted + Sized {
    type Model;
    fn primary_values(&self) -> Vec<crate::Value>;
    fn assign(&mut self, other: &Self) -> &mut Self;
    async fn insert_with_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn update_with_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn save_with_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        if self.persited() {
            self.update_with_exec(executor).await
        } else {
            self.insert_with_exec(executor).await
        }
    }
    async fn save(&mut self) -> crate::Result<()>;
    async fn increment_with_exec<'a, K: Send + ToString, E>(&mut self, key: K, step: i32, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn decrement_with_exec<'a, K: Send + ToString, E>(&mut self, key: K, step: i32, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        self.increment_with_exec(key, step * -1, executor).await
    }
    async fn increment<K: Send + ToString>(&mut self, key: K, step: i32) -> crate::Result<()>;
    async fn decrement<K: Send + ToString>(&mut self, key: K, step: i32) -> crate::Result<()> {
        self.increment(key, step * -1).await
    }
    async fn destroy_with_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn destroy(&mut self) -> crate::Result<()>;
}
