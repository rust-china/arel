use crate::statements::ArelStatement;

pub struct Lock {
    value: String,
}

impl ArelStatement for Lock {
    fn to_sql(&self) -> Option<crate::Sql> {
        Some(crate::Sql::new(&self.value))
    }
}

impl Lock {
    pub fn new<T: ToString>(raw: T) -> Self {
        Self { value: raw.to_string() }
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

        let lock = Lock::new("FOR UPDATE");
        assert_eq!(lock.to_sql().unwrap().to_sql_string().unwrap(), "FOR UPDATE");
    }
}
