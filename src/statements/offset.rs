use crate::statements::ArelStatement;

#[derive(Debug)]
pub struct Offset {
    num: usize,
}

impl ArelStatement for Offset {
    fn to_sql(&self) -> Option<crate::Sql> {
        let mut final_sql = crate::Sql::new("OFFSET ");
        final_sql.push_str(self.num.to_string());
        Some(final_sql)
    }
}

impl Offset {
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
        impl SuperArel for User {}
        impl Arel for User {}

        let offset = Offset::new(10);
        assert_eq!(offset.to_sql().unwrap().to_sql_string().unwrap(), "OFFSET 10");
    }
}
