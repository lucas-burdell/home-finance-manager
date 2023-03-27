use std::io::Error;
use sqlx::{Pool, MySql, mysql::MySqlPoolOptions};
use chrono::NaiveDate;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChaseStatementCSV {
    #[serde(alias = "Transaction Date")]
    transaction_date: String, 
    #[serde(alias = "Post Date")]
    post_date: String,
    description: String,
    category: String,
    r#type: String,
    amount: f64,
    memo: String
}

#[allow(dead_code)]
pub struct Statement {
    id: Option<i32>,
    transaction_date: String, 
    post_date: String,
    description: String,
    category: String,
    r#type: String,
    memo: String,
    amount: f64,
    account_id: i32
}

pub struct Uploader {
    pool: Pool<MySql>
}
// The number of parameters in MySQL must fit in a `u16`.
const BIND_LIMIT: usize = 65535;

impl Uploader {
    pub async fn new() -> Self {
        let connection = std::env::var("SQL_CONNECTION").expect("SQL_CONNECTION should be defined in dotenv file");
        let pool = MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&connection).await.expect("should be able to connect mysql");
        Self { 
            pool
        }
    }
    pub async fn upload_chase(&self, statements: &[ChaseStatementCSV], account_id: &i64) -> Result<(), Error> {
        let mut query_builder: sqlx::QueryBuilder<MySql> = sqlx::QueryBuilder::new(
            "INSERT INTO Statement(TransactionDate, PostDate, Description, Category, Type, Memo, Amount, AccountID) ");
        query_builder.push_values(statements.into_iter().take(BIND_LIMIT / 8), |mut b, statement| {
            let transaction_date = NaiveDate::parse_from_str(&statement.transaction_date, "%m/%d/%Y").expect("should parse").to_string();
            let post_date =  NaiveDate::parse_from_str(&statement.post_date, "%m/%d/%Y").expect("should parse").to_string();
            b.push_bind(transaction_date)
            .push_bind(post_date)
            .push_bind(&statement.description)
            .push_bind(&statement.category)
            .push_bind(&statement.r#type)
            .push_bind(&statement.memo)
            .push_bind(&statement.amount)
            .push_bind(account_id);
        });
        let query = query_builder.build();
        query.execute(&self.pool).await.expect("insert should succeed");
        return Ok(());
    }
}