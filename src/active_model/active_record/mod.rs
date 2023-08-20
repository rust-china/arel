// use crate::manager::SelectManager;
use crate::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct ActiveRecord<M: ArelModel> {
    table: M,
}

impl<M: ArelModel> Deref for ActiveRecord<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}
impl<M: ArelModel> DerefMut for ActiveRecord<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl<M: ArelModel> ActiveRecord<M> {
    pub fn new(table: M) -> Self {
        Self { table }
    }
    // pub fn select<T: AsRef<str>>(columns: Vec<T>) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.select(columns);
    //     select_manager
    // }
    // pub fn select_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.select_sql(sql);
    //     select_manager
    // }
    // pub fn join<U: ArelModel>(join_type: crate::JoinType) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.join::<U>(join_type);
    //     select_manager
    // }
    // pub fn join_sql<S: Into<crate::Sql>>(sql: S) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.join_sql(sql);
    //     select_manager
    // }
    // pub fn inner_join<U: ArelModel>() -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.join::<U>(crate::JoinType::InnerJoin);
    //     select_manager
    // }
    // pub fn left_join<U: ArelModel>() -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.join::<U>(crate::JoinType::LeftJoin);
    //     select_manager
    // }
    // pub fn r#where<K: AsRef<str>, V: Into<crate::Value>>(key: K, value: V) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.r#where(key, value);
    //     select_manager
    // }
    // pub fn where_not<K: AsRef<str>, V: Into<crate::Value>>(key: K, value: V) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.where_not(key, value);
    //     select_manager
    // }
    // pub fn where_or<K: AsRef<str>, V: Into<crate::Value>>(key: K, value: V) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.where_or(key, value);
    //     select_manager
    // }
    // pub fn group<T: AsRef<str>>(&mut self, columns: Vec<T>) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.group(columns);
    //     select_manager
    // }
    // pub fn having<K: AsRef<str>, V: Into<crate::Value>>(key: K, value: V) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.having(key, value);
    //     select_manager
    // }
    // pub fn having_not<K: AsRef<str>, V: Into<crate::Value>>(key: K, value: V) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.having_not(key, value);
    //     select_manager
    // }
    // pub fn having_or<K: AsRef<str>, V: Into<crate::Value>>(key: K, value: V) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.having_or(key, value);
    //     select_manager
    // }
    // pub fn order<T: AsRef<str>>(column: T, sort_type: crate::SortType) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.order(column, sort_type);
    //     select_manager
    // }
    // pub fn order_asc<T: AsRef<str>>(column: T) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.order_asc(column);
    //     select_manager
    // }
    // pub fn order_desc<T: AsRef<str>>(column: T) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.order_desc(column);
    //     select_manager
    // }
    // pub fn limit(num: usize) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.limit(num);
    //     select_manager
    // }
    // pub fn offset(num: usize) -> SelectManager<M> {
    //     let mut select_manager = SelectManager::<M>::default();
    //     select_manager.offset(num);
    //     select_manager
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::ArelBase;
    #[test]
    fn to_sql() {
        struct User {
            id: i32,
        }
        impl ArelBase for User {}
        impl ArelRecord for User {}
        impl ArelModel for User {}

        let active_record = ActiveRecord::new(User { id: 1 });
        assert_eq!(active_record.id, 1);
    }
}
