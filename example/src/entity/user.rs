use arel::prelude::*;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
    #[arel(primary_key)]
    name: String,
    age: Option<i32>,
    gender: Option<Gender>,
    r#type: Type,
    address: Option<String>,
    expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
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
							age   				 INT(11),
							gender         INT(1) NOT NULL DEFAULT 0,
							type           VARCHAR(255) NOT NULL default 'ADMIN',
							address        VARCHAR(255),
							expired_at     DATETIME
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
