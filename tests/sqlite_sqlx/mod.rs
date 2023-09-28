use arel::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
}
impl Default for Gender {
    fn default() -> Self {
        Self::Unknown
    }
}
impl ArelAttributeFromRow for Gender {
    fn from_row<'r, I>(row: &'r arel::db::DatabaseRow, index: I) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<arel::db::DatabaseRow>,
    {
        let value: u8 = row.try_get(index)?;
        let ret = match value {
            0 => Gender::Unknown,
            1 => Gender::Male,
            2 => Gender::Female,
            v @ _ => return Err(sqlx::Error::Decode(format!("{}: {} can not decode", std::any::type_name::<Self>(), v).into())),
        };
        Ok(ret)
    }
}
impl From<Gender> for arel::Value {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Unknown => 0.into(),
            Gender::Male => 1.into(),
            Gender::Female => 2.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Type {
    User,
    Admin,
}
impl Default for Type {
    fn default() -> Self {
        Self::User
    }
}
impl ArelAttributeFromRow for Type {
    fn from_row<'r, I>(row: &'r arel::db::DatabaseRow, index: I) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<arel::db::DatabaseRow>,
    {
        let value: String = row.try_get(index)?;
        let ret = match value.as_str() {
            "USER" => Self::User,
            "ADMIN" => Self::Admin,
            v @ _ => return Err(sqlx::Error::Decode(format!("{}: {} can not decode", std::any::type_name::<Self>(), v).into())),
        };
        Ok(ret)
    }
}
impl From<Type> for arel::Value {
    fn from(value: Type) -> Self {
        match value {
            Type::Admin => "ADMIN".into(),
            Type::User => "USER".into(),
        }
    }
}

#[arel(table_name = "users")]
pub struct User {
    #[arel(primary_key)]
    id: i32,
    name: String,
    age: Option<i32>,
    gender: Option<Gender>,
    r#type: Type,
    address: Option<String>,
    expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl Arel for User {}

// impl<'r> arel::sqlx::FromRow<'r, arel::db::DatabaseRow> for User {
//     fn from_row(row: &'r arel::db::DatabaseRow) -> Result<Self, sqlx::Error> {
//         let mut model = Self::default();
//         model.id = <i32 as arel::ArelAttributeFromRow>::from_row(row, "id")?;
//         model.name = <String as arel::ArelAttributeFromRow>::from_row(row, "name")?;
//         model.age = <Option<i32> as arel::ArelAttributeFromRow>::from_row(row, "age")?;
//         model.gender = <Gender as arel::ArelAttributeFromRow>::from_row(row, "gender")?;
//         model.r#type = <String as arel::ArelAttributeFromRow>::from_row(row, "type")?;
//         model.address = <Option<String> as arel::ArelAttributeFromRow>::from_row(row, "address")?;
//         model.expired_at = <Option<chrono::DateTime<chrono::FixedOffset>> as arel::ArelAttributeFromRow>::from_row(row, "expired_at")?;
//         Ok(model)
//     }
// }

pub async fn init_db() -> arel::Result<()> {
    let visitor = arel::db::visitor::get_or_init(|| Box::pin(async { arel::db::DatabasePoolOptions::new().max_connections(5).connect("sqlite::memory:").await })).await?;
    arel::sqlx::query(
        "CREATE TABLE IF NOT EXISTS users
					(
							id             INTEGER PRIMARY KEY NOT NULL,
							name           VARCHAR(255) NOT NULL,
							age   		   INT(11),
							gender         INT(1),
							type           VARCHAR(255) NOT NULL default 'ADMIN',
							address        VARCHAR(255),
							expired_at     DATETIME,
                            created_at     DATETIME DEFAULT CURRENT_TIMESTAMP
					);",
    )
    .execute(visitor.pool())
    .await?;

    User::with_transaction(|tx| {
        Box::pin(async move {
            for entry in 1i32..=100 {
                sqlx::query("INSERT INTO users (name) VALUES ($1)").bind(format!("name-{}", entry)).execute(tx.as_mut()).await?;
            }
            Ok(None)
        })
    })
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[tokio::test]
    async fn test_visitor() -> anyhow::Result<()> {
        init_db().await?;

        test_query().await?;
        test_insert().await?;
        test_update().await?;
        test_destroy().await?;

        Ok(())
    }
    async fn test_query() -> anyhow::Result<()> {
        let first_user = sqlx::query_as::<_, User>("SELECT * FROM users LIMIT 1").fetch_one(arel::db::get_pool()?).await?;
        assert_eq!(first_user.id, arel::ActiveValue::Unchanged(1.into()));
        assert_eq!(first_user.gender, Some(Gender::Unknown));
        assert_eq!(first_user.r#type, Type::Admin);

        let first_user2 = User::query().fetch_one().await?;
        let first_user_json = serde_json::to_string(&first_user2)?;
        assert_eq!(first_user_json, first_user_json);

        Ok(())
    }
    async fn test_insert() -> anyhow::Result<()> {
        let mut new_user = User {
            name: Set("hello"),
            gender: Set(Gender::Male),
            ..Default::default()
        };
        assert!(!new_user.persited());
        new_user.save().await?;
        assert!(new_user.persited());
        assert_eq!(new_user.name.value().unwrap(), "hello");
        assert!(new_user.id.value().is_some());
        Ok(())
    }
    async fn test_update() -> anyhow::Result<()> {
        let mut user = User::query().order_desc("id").fetch_one().await?;
        let old_name = user.name.clone();
        user.name.set("hello2");
        user.assign(&User { age: Set(20), ..Default::default() });
        assert_eq!(user.name, arel::ActiveValue::Changed("hello2".into(), Box::new(old_name)));
        user.save().await?;
        assert_eq!(user.name, arel::ActiveValue::Unchanged("hello2".into()));

        // increment
        user.increment("age", 5).await?;
        assert_eq!(user.age, arel::ActiveValue::Unchanged(25.into()));
        user.decrement("age", 5).await?;
        assert_eq!(user.age, arel::ActiveValue::Unchanged(20.into()));

        Ok(())
    }

    async fn test_destroy() -> anyhow::Result<()> {
        let mut user = User::query().order_desc("id").fetch_one().await?;
        let old_id = user.id.clone();
        user.destroy().await?;
        assert_eq!(user.id, arel::ActiveValue::Changed(old_id.value().unwrap().clone(), Box::new(arel::ActiveValue::NotSet)));

        Ok(())
    }
}
