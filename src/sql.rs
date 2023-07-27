#[derive(Clone, Debug)]
pub struct Sql {
    pub value: String,
    pub prepare_value: Option<Vec<crate::Value>>,
}
impl Default for Sql {
    fn default() -> Self {
        Self {
            value: String::new(),
            prepare_value: None,
        }
    }
}

impl Sql {
    pub fn new<T: ToString>(value: T) -> Self {
        Self {
            value: value.to_string(),
            prepare_value: None,
        }
    }
    pub fn new_with_prepare<T: ToString>(value: T, prepare_value: Vec<crate::Value>) -> Self {
        Self {
            value: value.to_string(),
            prepare_value: Some(prepare_value),
        }
    }
    pub fn push(&mut self, r#char: char) -> &mut Self {
        self.value.push(r#char);
        self
    }
    pub fn push_str(&mut self, sub_str: &str) -> &mut Self {
        self.value.push_str(sub_str);
        self
    }
    pub fn push_prepare_value(&mut self, sub_prepare_value: Vec<crate::Value>) -> &mut Self {
        if let Some(prepare_value) = &mut self.prepare_value {
            prepare_value.extend_from_slice(&sub_prepare_value);
        } else {
            self.prepare_value = Some(sub_prepare_value);
        }
        self
    }
    pub fn push_str_with_prepare_value(&mut self, sub_str: &str, sub_prepare_value: Vec<crate::Value>) -> &mut Self {
        self.value.push_str(sub_str);
        self.push_prepare_value(sub_prepare_value);
        self
    }
    pub fn push_sql(&mut self, sql: Sql) -> &mut Self {
        if let Some(prepare_value) = sql.prepare_value {
            self.push_str_with_prepare_value(&sql.value, prepare_value);
        } else {
            self.push_str(&sql.value);
        }
        self
    }
    pub fn push_sqls(&mut self, sqls: Vec<Sql>, join_str: &str) -> &mut Self {
        let len = sqls.len();
        for (idx, sql) in sqls.into_iter().enumerate() {
            self.push_sql(sql);
            if idx != len - 1 {
                self.push_str(join_str);
            }
        }
        self
    }
    pub fn to_sql_string(&self) -> anyhow::Result<String> {
        if let Some(prepare_value) = &self.prepare_value {
            let mut replace_idx = 0;
            let raw_sql = self
                .value
                .chars()
                .map(|char| match char {
                    '?' => {
                        let use_replace_value = prepare_value.get(replace_idx).ok_or_else(|| anyhow::anyhow!("参数不足"))?;
                        replace_idx += 1;
                        Ok(use_replace_value.to_sql().value)
                    }
                    _ => Ok(char.to_string()),
                })
                .collect::<anyhow::Result<String>>()?;
            if replace_idx == prepare_value.len() {
                Ok(raw_sql)
            } else {
                Err(anyhow::anyhow!("prepare sql params count not match: {}", raw_sql))
            }
        } else {
            Ok(self.value.clone())
        }
    }
}

impl TryFrom<Sql> for String {
    type Error = anyhow::Error;
    fn try_from(sql: Sql) -> Result<Self, Self::Error> {
        sql.to_sql_string()
    }
}

impl<T: ToString> From<T> for Sql {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sql = Sql::default();
        sql.push_str("select")
            .push(' ')
            .push_str_with_prepare_value(r#"* from users where users.id = ? and name = ?"#, vec![1.into(), "sanmu".into()]);
        assert_eq!(&sql.to_sql_string().unwrap(), r#"select * from users where users.id = 1 and name = "sanmu""#);
    }
}
