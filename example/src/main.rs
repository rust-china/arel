use arel::prelude::*;

mod entity;

#[tokio::main]
async fn main() -> arel::Result<()> {
    pretty_env_logger::init();
    entity::user::init_db().await?;

    let first_user = sqlx::query_as::<_, entity::User>("SELECT * FROM users LIMIT 1").fetch_one(arel::db::get_pool()?).await?;
    println!("{:?}", first_user);
    println!("{:?}", entity::User::primary_keys());

    println!("{:?}", std::any::type_name::<std::option::Option<Vec<i32>>>());

    let first_user: entity::User = entity::User::query().fetch_one().await?;
    let mut arel_first_user: entity::user::ArelUser = first_user.into();
    println!("{:?}", arel_first_user.primary_values());
    println!("db -->: {:?}", arel_first_user);

    arel_first_user.r#type = Set(entity::user::Type::User);
    arel_first_user.save().await?;
    println!("update -->: {:?}", arel_first_user);

    let mut arel_new_user: entity::user::ArelUser = entity::user::ArelUser {
        name: Set("hello"),
        gender: Set(entity::user::Gender::Male),
        ..Default::default()
    };

    arel_new_user.save().await?;
    println!("insert -->: {:?}", arel_new_user);

    arel_new_user.destroy().await?;
    println!("destroy -->: {:?}", arel_new_user);

    let first_user: entity::User = entity::User::query().fetch_one().await?;
    println!("{:?}", serde_json::to_string(&first_user));
    let active_user: entity::user::ArelUser = first_user.into();

    let json = match serde_json::to_string(&active_user) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };
    println!("{:?}", json);
    let to_user: entity::user::User = serde_json::from_str(&json)?;
    println!("{:?}", to_user);
    let to_arel_user: entity::user::ArelUser = serde_json::from_str(&json)?;
    println!("{:?}", to_arel_user);

    Ok(())
}
