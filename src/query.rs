pub struct Query {
    query_type: QueryType,
    columns: Vec<String>,
    table: Option<String>,
    conditions: Vec<String>,
    params: Vec<String>,
}

enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

impl Query {
    pub fn select() -> Self {
        Query {
            query_type: QueryType::Select,
            columns: Vec::new(),
            table: None,
            conditions: Vec::new(),
            params: Vec::new(),
        }
    }

    pub fn columns(mut self, cols: &[&str]) -> Self {
        self.columns = cols.iter().map(|&s| s.to_string()).collect();
        self
    }

    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn add_param(mut self, param: &str) -> Self {
        self.params.push(param.to_string());
        self
    }

    pub fn build(self) -> String {
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
            _ => {}
        }

        if let Some(table) = self.table {
            query.push_str(&format!("FROM {} ", table));
        }

        if !self.conditions.is_empty() {
            query.push_str("WHERE ");
            query.push_str(&self.conditions.join(" AND "));
            query.push(' ');
        }

        query.trim_end().to_string()
    }
}