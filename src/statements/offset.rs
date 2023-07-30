use crate::statements::ArelStatement;

pub struct Offset {
    value: usize,
}

impl ArelStatement for Offset {
    fn to_sql(&self) -> Option<crate::Sql> {
        let mut final_sql = crate::Sql::new("OFFSET ");
        final_sql.push_str(self.value.to_string());
        Some(final_sql)
    }
}

impl Offset {
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

        let offset = Offset::new(10);
        assert_eq!(offset.to_sql().unwrap().to_sql_string().unwrap(), "Offset 10");
    }
}
