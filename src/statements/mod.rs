pub mod filter;

pub mod delete;
pub mod group;
pub mod having;
pub mod increment;
pub mod insert;
pub mod join;
pub mod limit;
pub mod lock;
pub mod offset;
pub mod order;
pub mod select;
pub mod update;
pub mod r#where;

pub use delete::Delete;
pub use group::Group;
pub use having::Having;
pub use increment::Increment;
pub use insert::Insert;
pub use join::Join;
pub use limit::Limit;
pub use lock::Lock;
pub use offset::Offset;
pub use order::Order;
pub use r#where::Where;
pub use select::Select;
pub use update::Update;

pub trait ArelStatement {
    fn to_sql(&self) -> crate::Result<Option<crate::Sql>>;
}
