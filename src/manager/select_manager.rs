use crate::prelude::*;
use std::marker::PhantomData;
use std::ops::RangeBounds;

#[derive(Debug)]
pub struct SelectManager<M: Arel> {
    select: crate::statements::select::Select<M>,
    join: Option<crate::statements::join::Join<M>>,
    r#where: Option<crate::statements::r#where::Where<M>>,
    group: Option<crate::statements::group::Group<M>>,
    having: Option<crate::statements::having::Having<M>>,
    order: Option<crate::statements::order::Order<M>>,
    limit: Option<crate::statements::limit::Limit>,
    offset: Option<crate::statements::offset::Offset>,
    lock: Option<crate::statements::lock::Lock>,
    _marker: PhantomData<M>,
}

impl<M: Arel> Default for SelectManager<M> {
    fn default() -> Self {
        Self {
            select: crate::statements::select::Select::<M>::default(),
            join: None,
            r#where: None,
            group: None,
            having: None,
            order: None,
            limit: None,
            offset: None,
            lock: None,
            _marker: PhantomData,
        }
    }
}

impl<M: Arel> SelectManager<M> {
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::manager::SelectManager;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut select_manager = SelectManager::<User>::default();
    /// assert_eq!(select_manager.to_sql().to_sql_string().unwrap(), r#"SELECT "user".* FROM "user""#);
    ///
    /// select_manager.select(vec!["id", "name"]);
    /// assert_eq!(select_manager.to_sql().to_sql_string().unwrap(), r#"SELECT "user"."id", "user"."name" FROM "user""#);
    ///
    /// ```
    pub fn select<T: AsRef<str>>(&mut self, columns: Vec<T>) -> &mut Self {
        let select = crate::statements::select::Select::<M>::new(columns);
        self.select = select;
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::manager::SelectManager;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut select_manager = SelectManager::<User>::default();
    /// select_manager.select_sql("COUNT(*)");
    /// assert_eq!(select_manager.to_sql().to_sql_string().unwrap(), r#"SELECT COUNT(*) FROM "user""#);
    ///
    /// ```
    pub fn select_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        let select = crate::statements::select::Select::<M>::new_sql(sql);
        self.select = select;
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::manager::SelectManager;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut select_manager = SelectManager::<User>::default();
    /// select_manager.distinct();
    /// assert_eq!(select_manager.to_sql().to_sql_string().unwrap(), r#"SELECT DISTINCT "user".* FROM "user""#);
    ///
    /// ```
    pub fn distinct(&mut self) -> &mut Self {
        self.select.distinct();
        self
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::manager::SelectManager;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// #[arel]
    /// struct Wallet {}
    /// impl Arel for Wallet {}
    /// let mut select_manager = SelectManager::<User>::default();
    /// select_manager.join::<Wallet>(arel::JoinType::InnerJoin);
    /// assert_eq!(select_manager.to_sql().to_sql_string().unwrap(), r#"SELECT "user".* FROM "user" INNER JOIN "wallet" ON "user"."id" = "wallet"."user_id""#);
    /// ```
    pub fn join<U: Arel>(&mut self, join_type: crate::JoinType) -> &mut Self {
        if let Some(join) = &mut self.join {
            join.join::<U>(join_type);
        } else {
            let mut join = crate::statements::join::Join::<M>::new();
            join.join::<U>(join_type);
            self.join = Some(join);
        }
        self
    }
    pub fn inner_join<U: Arel>(&mut self) -> &mut Self {
        self.join::<U>(crate::JoinType::InnerJoin)
    }
    pub fn left_join<U: Arel>(&mut self) -> &mut Self {
        self.join::<U>(crate::JoinType::LeftJoin)
    }
    /// # Examples
    ///
    /// ```
    /// use arel::prelude::*;
    /// use arel::manager::SelectManager;
    /// #[arel]
    /// struct User {}
    /// impl Arel for User {}
    /// let mut select_manager = SelectManager::<User>::default();
    /// select_manager.join_sql("LEFT JOIN wallet on user.id = wallet.user_id");
    /// assert_eq!(select_manager.to_sql().to_sql_string().unwrap(), r#"SELECT "user".* FROM "user" LEFT JOIN wallet on user.id = wallet.user_id"#);
    /// ```
    pub fn join_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        if let Some(join) = &mut self.join {
            join.join_sql(sql);
        } else {
            let mut join = crate::statements::join::Join::<M>::new();
            join.join_sql(sql);
            self.join = Some(join);
        }
        self
    }
    pub fn r#where<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(r#where) = &mut self.r#where {
            r#where.filter_and(key, value);
        } else {
            let mut r#where = crate::statements::r#where::Where::<M>::new();
            r#where.filter_and(key, value);
            self.r#where = Some(r#where);
        }
        self
    }
    pub fn where_range<K: AsRef<str>, V: ToString, R: RangeBounds<V>>(&mut self, key: K, range: R) -> &mut Self {
        if let Some(sql) = crate::Sql::range_sql(key, range) {
            self.where_sql(sql);
        }
        self
    }
    pub fn where_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        if let Some(r#where) = &mut self.r#where {
            r#where.filter_and_sql(sql);
        } else {
            let mut r#where = crate::statements::r#where::Where::<M>::new();
            r#where.filter_and_sql(sql);
            self.r#where = Some(r#where);
        }
        self
    }
    pub fn where_not<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(r#where) = &mut self.r#where {
            r#where.filter_and_not(key, value);
        } else {
            let mut r#where = crate::statements::r#where::Where::<M>::new();
            r#where.filter_and_not(key, value);
            self.r#where = Some(r#where);
        }
        self
    }
    pub fn where_or<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(r#where) = &mut self.r#where {
            r#where.filter_or(key, value);
        } else {
            let mut r#where = crate::statements::r#where::Where::<M>::new();
            r#where.filter_or(key, value);
            self.r#where = Some(r#where);
        }
        self
    }
    pub fn group<T: AsRef<str>>(&mut self, columns: Vec<T>) -> &mut Self {
        let group = crate::statements::group::Group::<M>::new(columns);
        self.group = Some(group);
        self
    }
    pub fn having<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(having) = &mut self.having {
            having.filter_and(key, value);
        } else {
            let mut having = crate::statements::having::Having::<M>::new();
            having.filter_and(key, value);
            self.having = Some(having);
        }
        self
    }
    pub fn having_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        if let Some(having) = &mut self.having {
            having.filter_and_sql(sql);
        } else {
            let mut having = crate::statements::having::Having::<M>::new();
            having.filter_and_sql(sql);
            self.having = Some(having);
        }
        self
    }
    pub fn having_not<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(having) = &mut self.having {
            having.filter_and_not(key, value);
        } else {
            let mut having = crate::statements::having::Having::<M>::new();
            having.filter_and_not(key, value);
            self.having = Some(having);
        }
        self
    }
    pub fn having_or<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(having) = &mut self.having {
            having.filter_or(key, value);
        } else {
            let mut having = crate::statements::having::Having::<M>::new();
            having.filter_or(key, value);
            self.having = Some(having);
        }
        self
    }
    pub fn order<T: AsRef<str>>(&mut self, column: T, sort_type: crate::SortType) -> &mut Self {
        if let Some(order) = &mut self.order {
            order.append(column, sort_type);
        } else {
            let order = crate::statements::order::Order::<M>::new(column, sort_type);
            self.order = Some(order);
        }
        self
    }
    pub fn order_asc<T: AsRef<str>>(&mut self, column: T) -> &mut Self {
        self.order(column, crate::SortType::Asc)
    }
    pub fn order_desc<T: AsRef<str>>(&mut self, column: T) -> &mut Self {
        self.order(column, crate::SortType::Desc)
    }
    pub fn limit(&mut self, num: usize) -> &mut Self {
        let limit = crate::statements::limit::Limit::new(num);
        self.limit = Some(limit);
        self
    }
    pub fn offset(&mut self, num: usize) -> &mut Self {
        let offset = crate::statements::offset::Offset::new(num);
        self.offset = Some(offset);
        self
    }
    pub fn paginate(&mut self, page: usize, page_size: usize) -> &mut Self {
        let offset = (std::cmp::max(page, 1) - 1) * page_size;
        self.limit(page_size);
        self.offset(offset)
    }
    pub fn lock(&mut self) -> &mut Self {
        let lock = crate::statements::lock::Lock::new();
        self.lock = Some(lock);
        self
    }
    pub fn to_sql(&self) -> crate::Sql {
        let table_name = M::table_name();
        let mut final_sql = crate::Sql::new("");

        let mut select_sql = crate::Sql::new(format!(r#"SELECT "{}".* FROM "{}""#, table_name, table_name));
        if let Some(sql) = self.select.to_sql() {
            select_sql = sql;
        }
        final_sql.push_sql(select_sql);

        if let Some(join) = &self.join {
            if let Some(sql) = join.to_sql() {
                final_sql.push(' ').push_sql(sql);
            }
        }

        if let Some(r#where) = &self.r#where {
            if let Some(sql) = r#where.to_sql() {
                final_sql.push(' ').push_sql(sql);
            }
        }

        if let Some(group) = &self.group {
            if let Some(sql) = group.to_sql() {
                final_sql.push(' ').push_sql(sql);
            }
        }

        if let Some(having) = &self.having {
            if let Some(sql) = having.to_sql() {
                final_sql.push(' ').push_sql(sql);
            }
        }

        if let Some(order) = &self.order {
            if let Some(sql) = order.to_sql() {
                final_sql.push(' ').push_sql(sql);
            }
        }

        if let Some(limit) = &self.limit {
            if let Some(sql) = limit.to_sql() {
                final_sql.push(' ').push_sql(sql);
            }
        }

        if let Some(offset) = &self.offset {
            if let Some(sql) = offset.to_sql() {
                final_sql.push(' ').push_sql(sql);
            }
        }

        if let Some(lock) = &self.lock {
            if let Some(sql) = lock.to_sql() {
                final_sql.push(' ').push_sql(sql);
            }
        }

        final_sql
    }
}

impl<M: Arel> SelectManager<M> {
    pub fn to_query_builder<'a>(&self) -> anyhow::Result<crate::sql::QueryBuilder<'a>> {
        self.to_sql().to_query_builder()
    }
    pub async fn fetch_one(&self) -> anyhow::Result<crate::DatabaseRow> {
        self.to_sql().fetch_one_exec(M::pool()?).await
    }
    pub async fn fetch_count(&self) -> anyhow::Result<i64> {
        let row: (i64,) = self.to_sql().fetch_one_exec_as(M::pool()?).await?;
        Ok(row.0)
    }
    pub async fn fetch_one_as<T: crate::traits::ArelPersisted>(&self) -> anyhow::Result<T>
    where
        for<'b> T: Send + Unpin + sqlx::FromRow<'b, crate::DatabaseRow>,
    {
        let mut ret: T = self.to_sql().fetch_one_exec_as(M::pool()?).await?;
        ret.set_persisted(true);
        Ok(ret)
    }
    pub async fn fetch_all(&self) -> anyhow::Result<Vec<crate::DatabaseRow>> {
        self.to_sql().fetch_all_exec(M::pool()?).await
    }
    pub async fn fetch_all_as<T: crate::traits::ArelPersisted>(&self) -> anyhow::Result<Vec<T>>
    where
        for<'b> T: Send + Unpin + sqlx::FromRow<'b, crate::DatabaseRow>,
    {
        let mut array: Vec<T> = self.to_sql().fetch_all_exec_as(M::pool()?).await?;
        array.iter_mut().for_each(|v| v.set_persisted(true));
        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(sqlx::FromRow)]
    struct User {}
    impl SuperArel for User {}
    impl Arel for User {}

    #[test]
    fn to_sql() {
        let mut select_manager = SelectManager::<User>::default();
        select_manager.select(vec!["name", "age"]);
        assert_eq!(select_manager.to_sql().to_sql_string().unwrap(), r#"SELECT "user"."name", "user"."age" FROM "user""#);

        let mut select_manager = SelectManager::<User>::default();
        select_manager.r#where("name", "sanmu");
        assert_eq!(
            select_manager.to_sql().to_sql_string().unwrap(),
            r#"SELECT "user".* FROM "user" WHERE ("user"."name" = ?{"String":"sanmu"})"#
        );
        select_manager.group(vec!["name", "age"]);
        assert_eq!(
            select_manager.to_sql().to_sql_string().unwrap(),
            r#"SELECT "user".* FROM "user" WHERE ("user"."name" = ?{"String":"sanmu"}) GROUP BY "user"."name", "user"."age""#
        );
        select_manager.having("age", 18);
        assert_eq!(
            select_manager.to_sql().to_sql_string().unwrap(),
            r#"SELECT "user".* FROM "user" WHERE ("user"."name" = ?{"String":"sanmu"}) GROUP BY "user"."name", "user"."age" HAVING ("user"."age" = 18)"#
        );
        select_manager.order("age", crate::SortType::Asc);
        assert_eq!(
            select_manager.to_sql().to_sql_string().unwrap(),
            r#"SELECT "user".* FROM "user" WHERE ("user"."name" = ?{"String":"sanmu"}) GROUP BY "user"."name", "user"."age" HAVING ("user"."age" = 18) ORDER BY "user"."age" ASC"#
        );
        select_manager.limit(10);
        assert_eq!(
            select_manager.to_sql().to_sql_string().unwrap(),
            r#"SELECT "user".* FROM "user" WHERE ("user"."name" = ?{"String":"sanmu"}) GROUP BY "user"."name", "user"."age" HAVING ("user"."age" = 18) ORDER BY "user"."age" ASC LIMIT 10"#
        );
        select_manager.offset(0);
        assert_eq!(
            select_manager.to_sql().to_sql_string().unwrap(),
            r#"SELECT "user".* FROM "user" WHERE ("user"."name" = ?{"String":"sanmu"}) GROUP BY "user"."name", "user"."age" HAVING ("user"."age" = 18) ORDER BY "user"."age" ASC LIMIT 10 OFFSET 0"#
        );
        select_manager.lock();
        assert_eq!(
            select_manager.to_sql().to_sql_string().unwrap(),
            r#"SELECT "user".* FROM "user" WHERE ("user"."name" = ?{"String":"sanmu"}) GROUP BY "user"."name", "user"."age" HAVING ("user"."age" = 18) ORDER BY "user"."age" ASC LIMIT 10 OFFSET 0 FOR UPDATE"#
        );
    }
}
