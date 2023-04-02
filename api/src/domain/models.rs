use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct Statement {
    #[sqlx(rename = "ID")]
    pub id: i32,
    #[sqlx(rename = "TransactionDate")]
    pub transaction_date: chrono::NaiveDate, 
    #[sqlx(rename = "PostDate")]
    pub post_date: chrono::NaiveDate,
    #[sqlx(rename = "Description")]
    pub description: String,
    #[sqlx(rename = "Category")]
    pub category: String,
    #[sqlx(rename = "Type")]
    pub r#type: String,
    #[sqlx(rename = "Memo")]
    pub memo: String,
    #[sqlx(rename = "Amount")]
    pub amount: Decimal,
    #[sqlx(rename = "AccountID")]
    pub account_id: i32
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChaseStatementCSV {
    #[serde(alias = "Transaction Date")]
    pub transaction_date: String, 
    #[serde(alias = "Post Date")]
    pub post_date: String,
    pub description: String,
    pub category: String,
    pub r#type: String,
    pub amount: f64,
    pub memo: String
}
