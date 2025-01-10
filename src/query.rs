use sqlx::Executor;
use sqlx::Type;
use sqlx::types::JsonValue;
use async_trait::async_trait;
use sqlx::{Database, FromRow, IntoArguments, Pool};
use std::marker::PhantomData;
use serde::Serialize;
use serde_json::Value;
use sqlx::types::Json;

/// Represents the type of SQL query to execute.
#[derive(Debug, Clone)]
enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

/// Represents the type of SQL join.
#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

/// Structure that holds information about a SQL join.
#[derive(Debug, Clone)]
struct Join {
    join_type: JoinType,
    table: String,
    condition: String,
}

/// Main structure for building SQL queries.
#[derive(Debug, Clone)]
pub struct Query<DB>
where
    DB: Database,
{
    query_type: QueryType,
    columns: Vec<String>,
    table: Option<String>,
    conditions: Vec<String>,
    params: Vec<Json<Value>>, // Changed to sqlx::types::Json<Value>
    joins: Vec<Join>,
    values: Vec<Json<Value>>, // Changed to sqlx::types::Json<Value>
    group_by: Vec<String>,
    having: Option<String>,
    order_by: Vec<String>,
    limit: Option<u64>,
    offset: Option<u64>,
    _db_marker: PhantomData<DB>,
}

impl<DB> Query<DB>
where
    DB: Database,
{
    /// Constructor for SELECT queries.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select();
    /// ```
    pub fn select() -> Self {
        Query {
            query_type: QueryType::Select,
            columns: Vec::new(),
            table: None,
            conditions: Vec::new(),
            params: Vec::new(),
            joins: Vec::new(),
            values: Vec::new(),
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            _db_marker: PhantomData,
        }
    }

    /// Constructor for INSERT queries.
    ///
    /// # Arguments
    /// * `table` - The name of the table to insert into.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::insert_into("users");
    /// ```
    pub fn insert_into(table: &str) -> Self {
        Query {
            query_type: QueryType::Insert,
            table: Some(table.to_string()),
            columns: Vec::new(),
            conditions: Vec::new(),
            params: Vec::new(),
            joins: Vec::new(),
            values: Vec::new(),
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            _db_marker: PhantomData,
        }
    }

    /// Constructor for UPDATE queries.
    ///
    /// # Arguments
    /// * `table` - The name of the table to update.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::update("users");
    /// ```
    pub fn update(table: &str) -> Self {
        Query {
            query_type: QueryType::Update,
            table: Some(table.to_string()),
            columns: Vec::new(),
            conditions: Vec::new(),
            params: Vec::new(),
            joins: Vec::new(),
            values: Vec::new(),
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            _db_marker: PhantomData,
        }
    }

    /// Constructor for DELETE queries.
    ///
    /// # Arguments
    /// * `table` - The name of the table to delete from.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::delete_from("users");
    /// ```
    pub fn delete_from(table: &str) -> Self {
        Query {
            query_type: QueryType::Delete,
            table: Some(table.to_string()),
            columns: Vec::new(),
            conditions: Vec::new(),
            params: Vec::new(),
            joins: Vec::new(),
            values: Vec::new(),
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            _db_marker: PhantomData,
        }
    }

    /// Specifies the columns to select or manipulate.
    ///
    /// # Arguments
    /// * `cols` - A slice of column names.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select().columns(&["id", "name", "age"]);
    /// ```
    pub fn columns(mut self, cols: &[&str]) -> Self {
        self.columns = cols.iter().map(|&s| s.to_string()).collect();
        self
    }

    /// Sets the table for the query.
    ///
    /// # Arguments
    /// * `table` - The name of the table.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select().from("users");
    /// ```
    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    /// Adds a WHERE clause to the query.
    ///
    /// # Arguments
    /// * `condition` - The condition as a string.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select().where_clause("age > 18");
    /// ```
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    /// Adds a parameter to the query.
    ///
    /// # Arguments
    /// * `param` - The value of the parameter.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::insert_into("users").add_param(18);
    /// ```
    pub fn add_param<T>(mut self, param: T) -> Self
    where
        T: Send + Sync + Serialize + 'static,
    {
        let value = serde_json::to_value(param).expect("Error serializing the parameter");
        self.params.push(Json(value));
        self
    }

    /// Sets the values for INSERT or UPDATE queries.
    ///
    /// # Arguments
    /// * `vals` - A slice of values.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::insert_into("users").values(&["John Doe", 30]);
    /// ```
    pub fn values<T>(mut self, vals: &[T]) -> Self
    where
        T: Send + Sync + Serialize + 'static,
    {
        for val in vals {
            let value = serde_json::to_value(val).expect("Error serializing the value");
            self.values.push(Json(value));
        }
        self
    }

    /// Sets column-value pairs for UPDATE queries.
    ///
    /// # Arguments
    /// * `col_vals` - A slice of tuples containing column names and their corresponding values.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::update("users").set(&[("name", "Jane Doe"), ("age", 25)]);
    /// ```
    pub fn set<T>(mut self, col_vals: &[(&str, T)]) -> Self
    where
        T: Send + Sync + Serialize + Clone + 'static,
    {
        for &(col, ref val) in col_vals {
            self.columns.push(col.to_string());
            let value = serde_json::to_value(val.clone()).expect("Error serializing the value");
            self.values.push(Json(value));
        }
        self
    }

    /// Adds a JOIN clause to the query.
    ///
    /// # Arguments
    /// * `join_type` - The type of join (`Inner`, `Left`, `Right`, `Full`).
    /// * `table` - The name of the table to join.
    /// * `condition` - The join condition.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::JoinType;
    /// let query = Query::select().join(JoinType::Inner, "orders", "users.id = orders.user_id");
    /// ```
    pub fn join(mut self, join_type: JoinType, table: &str, condition: &str) -> Self {
        self.joins.push(Join {
            join_type,
            table: table.to_string(),
            condition: condition.to_string(),
        });
        self
    }

    /// Adds GROUP BY clauses to the query.
    ///
    /// # Arguments
    /// * `cols` - A slice of column names.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select().group_by(&["department"]);
    /// ```
    pub fn group_by(mut self, cols: &[&str]) -> Self {
        self.group_by = cols.iter().map(|&s| s.to_string()).collect();
        self
    }

    /// Adds a HAVING clause to the query.
    ///
    /// # Arguments
    /// * `condition` - The condition as a string.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select().having("COUNT(*) > 5");
    /// ```
    pub fn having(mut self, condition: &str) -> Self {
        self.having = Some(condition.to_string());
        self
    }

    /// Adds ORDER BY clauses to the query.
    ///
    /// # Arguments
    /// * `cols` - A slice of column names with sorting directions.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select().order_by(&["name ASC", "age DESC"]);
    /// ```
    pub fn order_by(mut self, cols: &[&str]) -> Self {
        self.order_by = cols.iter().map(|&s| s.to_string()).collect();
        self
    }

    /// Sets a limit on the number of results.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of rows to return.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select().limit(10);
    /// ```
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets an offset for the results.
    ///
    /// # Arguments
    /// * `offset` - The number of rows to skip.
    ///
    /// # Example
    /// ```
    /// use naturalquerylib::Query;
    /// let query = Query::select().offset(5);
    /// ```
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Builds the SQL query string.
    ///
    /// # Returns
    /// The constructed SQL query as a `String`.
    pub fn build(&self) -> String {
        let mut query = String::new();

        match self.query_type {
            QueryType::Select => {
                let cols = if self.columns.is_empty() {
                    "*".to_string()
                } else {
                    self.columns.join(", ")
                };
                query.push_str(&format!("SELECT {} ", cols));
            }
            QueryType::Insert => {
                let cols = self.columns.join(", ");
                let placeholders: Vec<String> = (0..self.values.len())
                    .map(|_| "?".to_string())
                    .collect();
                let placeholders_str = placeholders.join(", ");

                query.push_str(&format!(
                    "INSERT INTO {} ({}) VALUES ({}) ",
                    self.table.as_ref().unwrap(),
                    cols,
                    placeholders_str
                ));
            }
            QueryType::Update => {
                let set_clauses: Vec<String> = self
                    .columns
                    .iter()
                    .map(|col| format!("{} = ?", col))
                    .collect();

                query.push_str(&format!(
                    "UPDATE {} SET {} ",
                    self.table.as_ref().unwrap(),
                    set_clauses.join(", ")
                ));
            }
            QueryType::Delete => {
                query.push_str(&format!("DELETE FROM {} ", self.table.as_ref().unwrap()));
            }
        }

        if let Some(table) = &self.table {
            if matches!(self.query_type, QueryType::Select) {
                query.push_str(&format!("FROM {} ", table));
            }
        }

        if !self.joins.is_empty() {
            for join in &self.joins {
                let join_str = match join.join_type {
                    JoinType::Inner => "INNER JOIN",
                    JoinType::Left => "LEFT JOIN",
                    JoinType::Right => "RIGHT JOIN",
                    JoinType::Full => "FULL JOIN",
                };
                query.push_str(&format!(
                    "{} {} ON {} ",
                    join_str, join.table, join.condition
                ));
            }
        }

        if !self.conditions.is_empty() {
            query.push_str("WHERE ");
            query.push_str(&self.conditions.join(" AND "));
            query.push(' ');
        }

        if !self.group_by.is_empty() {
            query.push_str(&format!("GROUP BY {} ", self.group_by.join(", ")));
        }

        if let Some(having) = &self.having {
            query.push_str(&format!("HAVING {} ", having));
        }

        if !self.order_by.is_empty() {
            query.push_str(&format!("ORDER BY {} ", self.order_by.join(", ")));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!("LIMIT {} ", limit));
        }

        if let Some(offset) = self.offset {
            query.push_str(&format!("OFFSET {} ", offset));
        }

        query.trim_end().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{Sqlite, SqlitePool, SqlitePoolOptions};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow, PartialEq)]
    struct User {
        id: i64,
        name: String,
        age: i32,
    }

    /// Helper function to create an in-memory SQLite database for testing.
    async fn get_test_pool() -> SqlitePool {
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect(":memory:")
            .await
            .unwrap()
    }

    /// Test building a SELECT query with multiple clauses.
    #[tokio::test]
    async fn test_select_query_build() {
        let query = Query::<Sqlite>::select()
            .columns(&["id", "name", "age"])
            .from("users")
            .where_clause("age > ?")
            .order_by(&["age DESC"])
            .limit(10)
            .offset(5);

        let sql = query.build();

        assert_eq!(
            sql,
            "SELECT id, name, age FROM users WHERE age > ? ORDER BY age DESC LIMIT 10 OFFSET 5"
        );
    }

    /// Test building a query with JOIN clauses.
    #[tokio::test]
    async fn test_join_query_build() {
        let query = Query::<Sqlite>::select()
            .columns(&["u.name", "o.order_date"])
            .from("users u")
            .join(JoinType::Inner, "orders o", "u.id = o.user_id")
            .where_clause("u.id = ?")
            .order_by(&["o.order_date DESC"]);

        let sql = query.build();

        assert_eq!(
            sql,
            "SELECT u.name, o.order_date FROM users u INNER JOIN orders o ON u.id = o.user_id WHERE u.id = ? ORDER BY o.order_date DESC"
        );
    }

    /// Test constructing a query with a subquery in the WHERE clause.
    #[tokio::test]
    async fn test_subquery_build() {
        let subquery = Query::<Sqlite>::select()
            .columns(&["id"])
            .from("users")
            .where_clause("age > ?")
            .build();

        let main_query = Query::<Sqlite>::select()
            .columns(&["name"])
            .from("employees")
            .where_clause(&format!("user_id IN ({})", subquery));

        let sql = main_query.build();

        assert_eq!(
            sql,
            "SELECT name FROM employees WHERE user_id IN (SELECT id FROM users WHERE age > ?)"
        );
    }
}








