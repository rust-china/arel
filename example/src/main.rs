use arel::Arel;

mod entity;

#[tokio::main]
async fn main() -> arel::Result<()> {
    pretty_env_logger::init();
    entity::user::init_db().await?;

    let first_user = sqlx::query_as::<_, entity::User>("SELECT * FROM users LIMIT 1").fetch_one(arel::db::get_pool()?).await?;
    println!("{:?}", first_user);
    println!("{:?}", entity::User::primary_keys());
    println!("{:?}", first_user.primary_values());

    println!("{:?}", std::any::type_name::<std::option::Option<Vec<i32>>>());

    Ok(())
}
