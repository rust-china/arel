# Arel &emsp;

[![ci](https://github.com/rust-china/arel/workflows/Rust/badge.svg)](https://github.com/rust-china/arel/actions)
[![Latest Version]][crates.io]
![downloads](https://img.shields.io/crates/d/arel.svg?style=flat-square)

[Latest Version]: https://img.shields.io/crates/v/arel.svg
[crates.io]: https://crates.io/crates/arel

### Install

```rust
// Cargo.toml
serde = { version = "1.0", features = ["derive"] }
arel = { version = "0.3", features = ["runtime-tokio-native-tls", "sqlite"] } // sqlte | mysql | postgres
```

### Demo

```rust
use arel::prelude::*;
#[arel]
struct User {
  #[arel(primary_key)]
  id: i64,
  name: String,
  #[arel(rename = "type")]
  r#type: String,
  expired_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
impl Arel for User {}

// init db from Environment
std::env::set_var("DATABASE_URL", "sqlite::memory:");
arel::db::visitor::init().await?;
// or init db from code
arel::db::visitor::get_or_init(|| Box::pin(async { arel::db::DatabasePoolOptions::new().max_connections(5).connect("sqlite::memory:").await })).await?;

// total count
let count = User::query().select_sql("COUNT(*)").fetch_count().await?;
println!("total: {}", count);

// create
let mut active_user = User {
  name: Set("n1"),
  ..Default::default()
};
active_user.save().await?;

// select
let user: User = User::query().r#where("id", 1).fetch_one().await?;
let uesrs: Vec<User> = User::query().where_range("id", ..=10).fetch_all().await?;

// update
let mut user: User = User::query().fetch_one().await?;
// active_user.name.set("n-1");
user.assign(&User {
    name: Set("n-1"),
    ..Default::default()
});
user.save().await?;

// destroy
user.destroy().await?;
```

---

### Query

<details>
<summary>select</summary>

```rust
User::query().select(vec!(["id", "name"])).to_sql();
```

</details>

<details>
<summary>where</summary>

```rust
let sql = User::query().r#where("name", "n1").r#where("id", 1).to_sql();
// where_not
let sql = User::query().where_not(id: vec![1, 2, 3]).to_sql();
// where_range
let sql = User::query().where_range("age", 18..25).to_sql();
```

</details>

<details>
<summary>joins</summary>

```rust
let sql = User::query().join::<Wallet>(arel::JoinConst::InnerJoin).to_sql();
let sql = User::query().join_sql("INNER JOIN wallet on user.id = wallet.user_id").to_sql();
```

</details>

<details>
<summary>lock</summary>

```rust
// should use in transaction
let user: User = User::query().r#where("name", "n1").lock().fetch_one_with_exec(tx);
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
let sql = User::query().order("created_at", arel::SortConst::Desc).to_sql();
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

---

### Insert

<details>
<summary>transaction</summary>

```rust
User::with_transaction(|tx| {
  Box::pin(async move {
    // for entry in 1i32..=100 {
    //   sqlx::query("INSERT INTO user (name) VALUES ($1)")
    //       .bind(format!("name-{}", entry))
    //       .bind("Admin")
    //       .execute(tx.as_mut())
    //       .await?;
    // }
    let mut active_user = User {
      name: Set("n1"),
      r#type: Set("ADMIN"),
      ..Default::default()
    };
    active_user.save_with_exec(tx.as_mut()).await?;
    Ok(None)
  })
})
.await?;
```

</details>

### Update

<details>
<summary>increment</summary>

```rust
let active_user = User::query().r#where("id", 1).fetch_one().await?;
active_user.increment_save("lock_version", 5).await?;
```

</details>
