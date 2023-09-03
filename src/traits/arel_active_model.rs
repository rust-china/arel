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
}
