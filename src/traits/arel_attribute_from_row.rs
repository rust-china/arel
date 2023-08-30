use sqlx::Row;

pub trait ArelAttributeFromRow {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>;
}

impl ArelAttributeFromRow for bool {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for i8 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for i16 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for i32 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for i64 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for u8 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        match row.try_get::<i64, _>(index) {
            Ok(v) => Ok(v as u8),
            Err(e) => Err(e),
        }
    }
}

impl ArelAttributeFromRow for u16 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        match row.try_get::<i64, _>(index) {
            Ok(v) => Ok(v as u16),
            Err(e) => Err(e),
        }
    }
}

impl ArelAttributeFromRow for u32 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        match row.try_get::<i64, _>(index) {
            Ok(v) => Ok(v as u32),
            Err(e) => Err(e),
        }
    }
}

impl ArelAttributeFromRow for u64 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        match row.try_get::<i64, _>(index) {
            Ok(v) => Ok(v as u64),
            Err(e) => Err(e),
        }
    }
}

impl ArelAttributeFromRow for f32 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for f64 {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for char {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        match row.try_get::<i64, _>(index) {
            Ok(v) => Ok(v as u8 as char),
            Err(e) => Err(e),
        }
    }
}

impl ArelAttributeFromRow for String {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for bytes::Bytes {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        match row.try_get::<Vec<u8>, _>(index) {
            Ok(v) => Ok(bytes::Bytes::from(v)),
            Err(e) => Err(e),
        }
    }
}

impl ArelAttributeFromRow for serde_json::Value {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for chrono::DateTime<chrono::Utc> {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        row.try_get(index)
    }
}

impl ArelAttributeFromRow for chrono::DateTime<chrono::FixedOffset> {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        match row.try_get::<chrono::DateTime<chrono::Utc>, _>(index) {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(e),
        }
    }
}

impl<T: ArelAttributeFromRow> ArelAttributeFromRow for Option<T> {
    fn from_row<'r, I>(row: &'r crate::DatabaseRow, index: I) -> sqlx::Result<Self, sqlx::Error>
    where
        Self: Sized,
        I: sqlx::ColumnIndex<crate::DatabaseRow>,
    {
        match <T as ArelAttributeFromRow>::from_row(row, index) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }
}
