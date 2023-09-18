use crate::prelude::*;
use std::marker::PhantomData;
use std::ops::RangeBounds;

#[derive(Debug)]
pub struct SelectManager<M: crate::Arel> {
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
    /// assert_eq!(select_manager.to_sql().unwrap().to_sql_string().unwrap(), r#"SELECT "user".* FROM "user""#);
    ///
    /// select_manager.select(vec!["id", "name"]);
    /// assert_eq!(select_manager.to_sql().unwrap().to_sql_string().unwrap(), r#"SELECT "user"."id", "user"."name" FROM "user""#);
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
    /// assert_eq!(select_manager.to_sql().unwrap().to_sql_string().unwrap(), r#"SELECT COUNT(*) FROM "user""#);
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
    /// assert_eq!(select_manager.to_sql().unwrap().to_sql_string().unwrap(), r#"SELECT DISTINCT "user".* FROM "user""#);
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
    /// select_manager.join::<Wallet>(arel::JoinConst::InnerJoin);
    /// assert_eq!(select_manager.to_sql().unwrap().to_sql_string().unwrap(), r#"SELECT "user".* FROM "user" INNER JOIN "wallet" ON "user"."id" = "wallet"."user_id""#);
    /// ```
    pub fn join<U: Arel>(&mut self, join_type: crate::JoinConst) -> &mut Self {
        if let Some(join) = &mut self.join {
            join.join::<U>(join_type);
        } else {
            let mut join = crate::statements::join::Join::<M>::default();
            join.join::<U>(join_type);
            self.join = Some(join);
        }
        self
    }
    pub fn inner_join<U: Arel>(&mut self) -> &mut Self {
        self.join::<U>(crate::JoinConst::InnerJoin)
    }
    pub fn left_join<U: Arel>(&mut self) -> &mut Self {
        self.join::<U>(crate::JoinConst::LeftJoin)
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
    /// assert_eq!(select_manager.to_sql().unwrap().to_sql_string().unwrap(), r#"SELECT "user".* FROM "user" LEFT JOIN wallet on user.id = wallet.user_id"#);
    /// ```
    pub fn join_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        if let Some(join) = &mut self.join {
            join.join_sql(sql);
        } else {
            let mut join = crate::statements::join::Join::<M>::default();
            join.join_sql(sql);
            self.join = Some(join);
        }
        self
    }
    pub fn r#where<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(r#where) = &mut self.r#where {
            r#where.and_filter(key, value);
        } else {
            let mut r#where = crate::statements::r#where::Where::<M>::default();
            r#where.and_filter(key, value);
            self.r#where = Some(r#where);
        }
        self
    }
    #[allow(non_snake_case)]
    pub fn Where<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        self.r#where(key, value)
    }
    pub fn where_range<K: AsRef<str>, V: ToString, R: RangeBounds<V>>(&mut self, key: K, range: R) -> &mut Self {
        if let Some(sql) = crate::Sql::range_sql(key, range) {
            self.where_sql(sql);
        }
        self
    }
    pub fn where_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        if let Some(r#where) = &mut self.r#where {
            r#where.and_filter_sql(sql);
        } else {
            let mut r#where = crate::statements::r#where::Where::<M>::default();
            r#where.and_filter_sql(sql);
            self.r#where = Some(r#where);
        }
        self
    }
    pub fn where_not<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(r#where) = &mut self.r#where {
            r#where.and_not_filter(key, value);
        } else {
            let mut r#where = crate::statements::r#where::Where::<M>::default();
            r#where.and_not_filter(key, value);
            self.r#where = Some(r#where);
        }
        self
    }
    pub fn where_or<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(r#where) = &mut self.r#where {
            r#where.or_filter(key, value);
        } else {
            let mut r#where = crate::statements::r#where::Where::<M>::default();
            r#where.or_filter(key, value);
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
            having.and_filter(key, value);
        } else {
            let mut having = crate::statements::having::Having::<M>::default();
            having.and_filter(key, value);
            self.having = Some(having);
        }
        self
    }
    pub fn having_sql<S: Into<crate::Sql>>(&mut self, sql: S) -> &mut Self {
        if let Some(having) = &mut self.having {
            having.and_filter_sql(sql);
        } else {
            let mut having = crate::statements::having::Having::<M>::default();
            having.and_filter_sql(sql);
            self.having = Some(having);
        }
        self
    }
    pub fn having_not<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(having) = &mut self.having {
            having.and_not_filter(key, value);
        } else {
            let mut having = crate::statements::having::Having::<M>::default();
            having.and_not_filter(key, value);
            self.having = Some(having);
        }
        self
    }
    pub fn having_or<K: AsRef<str>, V: Into<crate::Value>>(&mut self, key: K, value: V) -> &mut Self {
        if let Some(having) = &mut self.having {
            having.or_filter(key, value);
        } else {
            let mut having = crate::statements::having::Having::<M>::default();
            having.or_filter(key, value);
            self.having = Some(having);
        }
        self
    }
    pub fn order<T: AsRef<str>>(&mut self, column: T, sort_type: crate::SortConst) -> &mut Self {
        if let Some(order) = &mut self.order {
            order.append(column, sort_type);
        } else {
            let order = crate::statements::order::Order::<M>::new(column, sort_type);
            self.order = Some(order);
        }
        self
    }
    pub fn order_asc<T: AsRef<str>>(&mut self, column: T) -> &mut Self {
        self.order(column, crate::SortConst::Asc)
    }
    pub fn order_desc<T: AsRef<str>>(&mut self, column: T) -> &mut Self {
        self.order(column, crate::SortConst::Desc)
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
    pub fn to_sql(&self) -> crate::Result<crate::Sql> {
        let table_name = M::table_name();
        let mut final_sql = crate::Sql::new("");

        let mut select_sql = crate::Sql::new(format!(r#"SELECT "{}".* FROM "{}""#, table_name, table_name));
        if let Some(sql) = self.select.to_sql()? {
            select_sql = sql;
        }
        final_sql.push_sql(select_sql);

        if let Some(join) = &self.join {
            if let Some(sql) = join.to_sql()? {
                final_sql.push_str(" ").push_sql(sql);
            }
        }

        if let Some(r#where) = &self.r#where {
            if let Some(sql) = r#where.to_sql()? {
                final_sql.push_str(" ").push_sql(sql);
            }
        }

        if let Some(group) = &self.group {
            if let Some(sql) = group.to_sql()? {
                final_sql.push_str(" ").push_sql(sql);
            }
        }

        if let Some(having) = &self.having {
            if let Some(sql) = having.to_sql()? {
                final_sql.push_str(" ").push_sql(sql);
            }
        }

        if let Some(order) = &self.order {
            if let Some(sql) = order.to_sql()? {
                final_sql.push_str(" ").push_sql(sql);
            }
        }

        if let Some(limit) = &self.limit {
            if let Some(sql) = limit.to_sql()? {
                final_sql.push_str(" ").push_sql(sql);
            }
        }

        if let Some(offset) = &self.offset {
            if let Some(sql) = offset.to_sql()? {
                final_sql.push_str(" ").push_sql(sql);
            }
        }

        if let Some(lock) = &self.lock {
            if let Some(sql) = lock.to_sql()? {
                final_sql.push_str(" ").push_sql(sql);
            }
        }

        Ok(final_sql)
    }
}
