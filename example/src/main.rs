mod entity;

#[tokio::main]
async fn main() -> arel::Result<()> {
    pretty_env_logger::init();
    entity::user::init_db().await?;

    let first_user = sqlx::query_as::<_, entity::User>("SELECT * FROM users LIMIT 1").fetch_one(arel::db::get_pool()?).await?;
    eprintln!("{:?}", first_user);

    println!("{:?}", std::any::type_name::<std::option::Option<Vec<i32>>>());

    Ok(())
}
