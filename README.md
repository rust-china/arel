# Arel &emsp;

[![ci](https://github.com/rust-china/arel/workflows/Rust/badge.svg)](https://github.com/rust-china/arel/actions)
[![Latest Version]][crates.io]
![downloads](https://img.shields.io/crates/d/arel.svg?style=flat-square)

[Latest Version]: https://img.shields.io/crates/v/arel.svg
[crates.io]: https://crates.io/crates/arel

### Install

```Cargo.toml
# db: sqlite|postgres|mysql
arel = { version = "0.2", features = ["runtime-tokio", "tls-rustls", "sqlite"] }
```

### Demo

```rust
use arel::prelude::*;
#[arel]
struct User {
		#[arel(primary_key)]
    id: i64,
    name: String,
    expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

let count = User::query().select_sql("COUNT(*)").fetch_count().await?;
println!("total: {}", count);

// create
let mut active_user = ArelActiveUser::default();
active_user.name.set("n1".into());
let ret = active_user.save().await?;
println!("{}", ret.rows_affected());

// select
let user: User = User::query().r#where("id", 1).fetch_one_as().await?;
let _active_user: ArelActiveUser = user.into();

let user: ArelUser = User::query().r#where("id", 1).fetch_one_as().await?;
let _active_user: ArelActiveUser = user.into();

// update
let user: User = User::query().fetch_one_as().await?;
let mut active_user: ArelActiveUser = user.into();
active_user.name.set("n-1".into());
let ret = active_user.save().await?;
println!("{}", ret.rows_affected());

// delete
let ret = active_user.destroy().await?;
println!("{}", ret.rows_affected());
```
