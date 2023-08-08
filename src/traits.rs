use std::borrow::Cow;
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
    fn query() -> crate::manager::SelectManager<Self>
    where
        Self: Sized,
    {
        crate::manager::SelectManager::<Self>::default()
    }
    fn validates(&self) -> anyhow::Result<()> {
        Ok(())
    }
    fn to_sql(&self) {
        println!("todo");
    }
}

/// # Examples Trait ArelRecord
///
/// ```
/// use arel::prelude::*;
///#[derive(Debug, Default)]
/// struct User {
///     id: i64,
/// }
/// impl ArelBase for User {}
/// impl ArelRecord for User {}
/// let user = User::default();
/// assert!(user.validates().is_ok());
pub trait ArelRecord: ArelBase {}

/// # Examples Trait ArelModel
pub trait ArelModel: ArelRecord {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct User {}
    impl ArelBase for User {}
    impl ArelRecord for User {}
    impl ArelModel for User {}

    #[test]
    fn it_works() {
        let user = User::default();
        let query = User::query().to_sql().to_sql_string();
        assert_eq!(query.unwrap(), r#"SELECT "user".* FROM "user""#);

        let users: Vec<&dyn ArelModel> = vec![&user];
        // keep trait safe
        for user in users {
            user.to_sql();
        }
    }
}
