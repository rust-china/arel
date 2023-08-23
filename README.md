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

// init db from Environment
std::env::set_var("DATABASE_URL", "sqlite::memory:");
// or init db from code
arel::visitor::get_or_init(|| Box::pin(async { arel::DatabasePoolOptions::new().max_connections(5).connect("sqlite::memory:").await })).await?;

// total count
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

let uesrs: Vec<User> = User::query().where_range("id", ..=10).fetch_all_as().await?;
let uesrs: Vec<ArelUser> = User::query().r#where("id", vec![1, 2, 3]).fetch_all_as().await?;

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

---

### Query

<details>
<summary>select</summary>

```rust
User::query().select(vec!(["id", "name"])).fetch;
```

</details>

<details>
<summary>where</summary>

```rust
let sql = User::query().r#where("name", "n1").r#where("id", 1).to_sql();
// where_not
let sql = User::query().where_not(id: vec![1, 2, 3]).to_sql();
// where_range
let sql = User::query().where_range("age", 18..25).to_sql;
```

</details>

<details>
<summary>joins</summary>

```rust
let sql = User::query().join::<Wallet>(arel::JoinType::InnerJoin).to_sql();
let sql = User::query().join_sql("INNER JOIN wallet on user.id = wallet.user_id").to_sql();
```

</details>

<details>
<summary>lock</summary>

```rust
let sql = User::query().r#where("name", "n1").lock().to_sql();
```

</details>

<details>
<summary>group & having</summary>

```rust
let sql = User::query().group(vec!["name"]).having("age", 18..).to_sql();
```

</details>

<details>
<summary>order</summary>

```rust
let sql = User::query().order("created_at", arel::SortType::Desc).to_sql();
let sql = User::query().order_asc().to_sql();
let sql = User::query().order_desc().to_sql();
```

</details>

<details>
<summary>limit & offset</summary>

```rust
let sql = User::query().limit(10).to_sql();
let sql = User::query().offset(10)();
let sql = User::query().paginate(1, 10).to_sql();
```

</details>

<details>
<summary>transaction</summary>

```rust
User::with_transaction(|tx| {
  Box::pin(async move {
    for entry in 1i32..=100 {
      sqlx::query("INSERT INTO user (name) VALUES ($1)")
          .bind(format!("name-{}", entry))
          .bind("Admin")
          .execute(tx.as_mut())
          .await?;
    }
    Ok(None)
  })
})
.await?;
```

</details>
