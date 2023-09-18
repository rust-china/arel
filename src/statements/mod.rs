pub mod filter;

pub mod group;
pub mod having;
pub mod join;
pub mod limit;
pub mod lock;
pub mod offset;
pub mod order;
pub mod select;
pub mod r#where;

pub use group::Group;
pub use having::Having;
pub use join::Join;
pub use limit::Limit;
pub use lock::Lock;
pub use offset::Offset;
pub use order::Order;
pub use r#where::Where;
pub use select::Select;

pub trait ArelStatement {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>>;
}
