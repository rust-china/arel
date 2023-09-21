use crate::ArelPersisted;

#[async_trait::async_trait]
pub trait ArelModel: ArelPersisted + Sized {
    type Model;
    fn primary_values(&self) -> Vec<crate::Value>;
    async fn insert_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn update_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn save_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>,
    {
        if self.persited() {
            self.update_exec(executor).await
        } else {
            self.insert_exec(executor).await
        }
    }
    async fn save(&mut self) -> crate::Result<()>;
    async fn destroy_exec<'a, E>(&mut self, executor: E) -> crate::Result<()>
    where
        E: sqlx::Executor<'a, Database = crate::db::Database>;
    async fn destroy(&mut self) -> crate::Result<()>;
}
