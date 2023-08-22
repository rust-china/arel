use crate::statements::ArelStatement;

#[derive(Debug)]
pub struct Lock {
    value: String,
}

impl ArelStatement for Lock {
    fn to_sql(&self) -> Option<crate::Sql> {
        Some(crate::Sql::new(&self.value))
    }
}

impl Lock {
    pub fn new() -> Self {
        Self { value: "FOR UPDATE".to_string() }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::SuperArel;

    use super::*;
    #[test]
    fn to_sql() {
        struct User {}
        impl SuperArel for User {}

        let lock = Lock::new();
        assert_eq!(lock.to_sql().unwrap().to_sql_string().unwrap(), "FOR UPDATE");
    }
}
