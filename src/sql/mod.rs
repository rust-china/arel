mod query_builder;

pub struct Sql {
    pub raw_value: String,
    pub bind_indexs: Vec<usize>,
    pub bind_values: Vec<crate::Value>,
}

impl Default for Sql {
    fn default() -> Self {
        Self {
            raw_value: String::new(),
            bind_indexs: vec![],
            bind_values: vec![],
        }
    }
}

impl Sql {
    pub fn push<T: AsRef<str>>(&mut self, raw_str: T) -> &mut Self {
        self.raw_value.push_str(raw_str.as_ref());
        self
    }
    pub fn push_bind<V: Into<crate::Value>>(&mut self, bind_value: V) -> &mut Self {
        let bind_value: crate::Value = bind_value.into();
        self.push("?");
        self.bind_indexs.push(self.raw_value.len() - 1);
        self.bind_values.push(bind_value);
        self
    }
    pub fn push_with_bind<T: AsRef<str>, V: Into<crate::Value>>(&mut self, raw_str: T, bind_value: V) -> &mut Self {
        self.push(raw_str);
        self.push_bind(bind_value);
        self
    }
    pub fn push_sql(&mut self, sql: Sql) -> &mut Self {
        let raw_value_len = self.raw_value.len();
        self.push(sql.raw_value);
        self.bind_indexs.extend(sql.bind_indexs.into_iter().map(|idx| raw_value_len + idx - 1).collect::<Vec<usize>>());
        self.bind_values.extend(sql.bind_values);
        self
    }
}
