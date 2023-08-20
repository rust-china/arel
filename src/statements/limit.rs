use crate::statements::ArelStatement;

#[derive(Debug)]
pub struct Limit {
    num: usize,
}

impl ArelStatement for Limit {
    fn to_sql(&self) -> Option<crate::Sql> {
        let mut final_sql = crate::Sql::new("LIMIT ");
        final_sql.push_str(self.num.to_string());
        Some(final_sql)
    }
}

impl Limit {
    pub fn new(num: usize) -> Self {
        Self { num }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;
    #[test]
    fn to_sql() {
        struct User {}
        impl ArelBase for User {}
        impl ArelRecord for User {}
        impl ArelModel for User {}

        let limit = Limit::new(10);
        assert_eq!(limit.to_sql().unwrap().to_sql_string().unwrap(), "LIMIT 10");
    }
}
