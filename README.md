# NaturalQueryLib

![NaturalQueryLib](https://img.shields.io/badge/Rust-Async%20Query%20Builder-orange?style=for-the-badge)

Welcome to **NaturalQueryLib**, a highly flexible SQL query builder written in Rust! This library allows you to build and execute SQL queries with ease, while supporting dynamic parameters and JSON encoding for database compatibility.

---

## 🚀 Features

- Supports **SELECT**, **INSERT**, **UPDATE**, and **DELETE** queries.
- Fluent API for building complex SQL queries.
- JSON support via `serde_json` and `sqlx`.
- Asynchronous execution with `sqlx` connection pools.
- Lightweight and developer-friendly.

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
naturalquerylib = "0.1.0"
sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "json"] }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 🛠️ Usage

### **Basic Setup**

First, configure your SQLx connection pool:

```rust
use sqlx::{Pool, Postgres};
use naturalquerylib::Query;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = Pool::<Postgres>::connect("postgres://user:password@localhost/db_name").await?;
    
    // Example query
    let result = Query::select()
        .from("users")
        .columns(&["id", "name", "email"])
        .where_clause("active = true")
        .limit(10)
        .execute(&pool)
        .await?;

    println!("Rows affected: {}", result);
    Ok(())
}
```

---

### **Examples**

#### 1️⃣ **SELECT Query**

```rust
let select_query = Query::select()
    .from("users")
    .columns(&["id", "name", "email"])
    .where_clause("age > 18")
    .order_by(&["name ASC"])
    .limit(5)
    .build();

println!("Generated Query: {}", select_query);
// Output: SELECT id, name, email FROM users WHERE age > 18 ORDER BY name ASC LIMIT 5
```

#### 2️⃣ **INSERT Query**

```rust
let insert_query = Query::insert_into("users")
    .columns(&["name", "email", "age"])
    .values(&["John Doe", "john.doe@example.com", 30])
    .build();

println!("Generated Query: {}", insert_query);
// Output: INSERT INTO users (name, email, age) VALUES (?, ?, ?)
```

#### 3️⃣ **UPDATE Query**

```rust
let update_query = Query::update("users")
    .set(&[("name", "Jane Doe"), ("email", "jane.doe@example.com")])
    .where_clause("id = 1")
    .build();

println!("Generated Query: {}", update_query);
// Output: UPDATE users SET name = ?, email = ? WHERE id = 1
```

#### 4️⃣ **DELETE Query**

```rust
let delete_query = Query::delete_from("users")
    .where_clause("id = 1")
    .build();

println!("Generated Query: {}", delete_query);
// Output: DELETE FROM users WHERE id = 1
```

#### 5️⃣ **JOIN Query**

```rust
let join_query = Query::select()
    .from("users")
    .join(JoinType::Inner, "orders", "users.id = orders.user_id")
    .columns(&["users.id", "users.name", "orders.total"])
    .build();

println!("Generated Query: {}", join_query);
// Output: SELECT users.id, users.name, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id
```

---

### **Asynchronous Execution**

#### **Execute a Query**

```rust
let rows_affected = Query::update("users")
    .set(&[("active", true)])
    .where_clause("last_login < '2023-01-01'")
    .execute(&pool)
    .await?;

println!("Rows affected: {}", rows_affected);
```

#### **Fetch Results**

```rust
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

let users: Vec<User> = Query::select()
    .from("users")
    .columns(&["id", "name", "email"])
    .where_clause("active = true")
    .fetch_all(&pool)
    .await?;

for user in users {
    println!("{:?}", user);
}
```

---

## 🎨 Stickers for Clarity

| Query Type    | Example Output                                                                                         |
|---------------|-------------------------------------------------------------------------------------------------------|
| **SELECT**    | `SELECT id, name FROM users WHERE active = true LIMIT 5`                                               |
| **INSERT**    | `INSERT INTO users (name, email) VALUES (?, ?)`                                                        |
| **UPDATE**    | `UPDATE users SET name = ?, email = ? WHERE id = 1`                                                    |
| **DELETE**    | `DELETE FROM users WHERE id = 1`                                                                       |
| **JOIN**      | `SELECT users.id, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id`              |

---

## 🧩 Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

1. Fork the project.
2. Create your feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m 'Add YourFeature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Open a pull request.

---

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## 💬 Feedback

If you have any feedback, questions, or suggestions, feel free to reach out via [GitHub Issues](https://github.com/your-repo/issues).
