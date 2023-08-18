use std::borrow::Cow;
use std::future::Future;
use std::pin::Pin;
///
/// # Examples Trait ArelBase
///
/// ```
/// use arel::prelude::*;
///#[derive(Debug, Default)]
/// struct User {
///     id: i64,
/// }
/// impl ArelBase for User {}
///
///#[derive(Debug, Default)]
/// struct UserWallet<T: ToString> {
///     id: i64,
///     data: T,
/// }
/// impl<T: ToString + Default> ArelBase for UserWallet<T>  {}
///
/// assert_eq!(User::struct_name(), "User");
/// assert_eq!(UserWallet::<String>::struct_name(), "UserWallet<alloc::string::String>");
/// assert_eq!(User::table_name(), "user");
/// assert_eq!(UserWallet::<String>::table_name(), "user_wallet");
/// ```
pub trait ArelBase {
    fn struct_name_full_path() -> Cow<'static, str>
    where
        Self: Sized,
    {
        let struct_name = std::any::type_name::<Self>();
        Cow::Borrowed(struct_name)
    }
    fn struct_name() -> Cow<'static, str>
    where
        Self: Sized,
    {
        let struct_name = Self::struct_name_full_path();
        let struct_name = regex::Regex::new(r#"((\w+)$)|(\w+<.+)"#)
            .unwrap()
            .find(&struct_name)
            .expect(&format!("match {} fail", struct_name))
            .as_str();
        Cow::Owned(struct_name.to_owned())
    }
    fn table_name() -> Cow<'static, str>
    where
        Self: Sized,
    {
        let struct_name: Cow<'_, str> = Self::struct_name();
        let struct_name = regex::Regex::new(r#"^\w+"#).unwrap().find(&struct_name).expect(&format!("match {} fail", struct_name)).as_str();
        let struct_name = regex::Regex::new(r#"([a-z])([A-Z])"#)
            .unwrap()
            .replace(&struct_name, |caps: &regex::Captures| format!("{}_{}", &caps[1], &caps[2]))
            .to_lowercase();
        Cow::Owned(struct_name.into())
    }
    fn primary_key() -> Option<Cow<'static, str>>
    where
        Self: Sized,
    {
        if Self::primary_keys().is_some() {
            return None;
        }
        Some(Cow::Borrowed("id"))
    }
    fn primary_keys() -> Option<Vec<Cow<'static, str>>>
    where
        Self: Sized,
    {
        None
    }
    fn validates(&self) -> anyhow::Result<()> {
        Ok(())
    }
    fn to_sql(&self) {
        ()
    }
}

/// # Examples Trait ArelRecord
///
/// ```
/// use arel::prelude::*;
///#[derive(Debug, Default, sqlx::FromRow)]
/// struct User {
///     id: i64,
/// }
/// impl ArelBase for User {}
/// impl ArelRecord for User {}
/// let user = User::default();
/// assert!(user.validates().is_ok());
pub trait ArelRecord: ArelBase {}
#[async_trait::async_trait]
pub trait ArelModel: ArelRecord {
    fn query() -> crate::manager::SelectManager<Self>
    where
        Self: Sized,
    {
        crate::manager::SelectManager::<Self>::default()
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres"))]
    fn pool() -> anyhow::Result<&'static sqlx::Pool<crate::Database>>
    where
        Self: Sized,
    {
        Ok(crate::visitor::get()?.pool())
    }
    #[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres"))]
    async fn with_transaction<'a, F: Send>(callback: F) -> anyhow::Result<Option<Self>>
    where
        Self: Sized,
        for<'c> F: FnOnce(&'c mut sqlx::Transaction<'a, crate::Database>) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Self>>> + Send + 'c>>,
    {
        let pool = Self::pool()?;
        let mut tx = pool.begin().await?;
        match callback(&mut tx).await {
            Ok(model) => match tx.commit().await {
                Ok(_) => Ok(model),
                Err(e) => Err(anyhow::anyhow!(e.to_string())),
            },
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, sqlx::FromRow)]
    struct User {}
    impl ArelBase for User {}
    impl ArelRecord for User {}
    impl ArelModel for User {}

    #[test]
    fn it_works() {
        let query = User::query().to_sql().to_sql_string();
        assert_eq!(query.unwrap(), r#"SELECT "user".* FROM "user""#);

        let user = User::default();
        let users: Vec<&dyn ArelBase> = vec![&user];
        // keep trait safe
        for user in users {
            user.to_sql();
        }
    }
}
