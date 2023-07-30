use crate::statements::ArelStatement;

pub struct Limit {
    value: usize,
}

impl ArelStatement for Limit {
    fn to_sql(&self) -> Option<crate::Sql> {
        let mut final_sql = crate::Sql::new("LIMIT ");
        final_sql.push_str(self.value.to_string());
        Some(final_sql)
    }
}

impl Limit {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::ArelBase;

    use super::*;
    #[test]
    fn to_sql() {
        struct User {}
        impl ArelBase for User {}

        let limit = Limit::new(10);
        assert_eq!(limit.to_sql().unwrap().to_sql_string().unwrap(), "LIMIT 10");
    }
}
