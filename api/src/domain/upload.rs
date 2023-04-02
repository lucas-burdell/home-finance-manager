#![allow(dead_code)]
use std::io::{BufReader};
use sqlx::{Pool, MySql, query};
use chrono::NaiveDate;
use std::fs::File;
use csv::Reader;
use std::path::{Path, PathBuf};
use std::env::temp_dir;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::{form::Form, data::Capped, fs::TempFile, State, FromForm};
use crate::{conv_err, expect_err};
use crate::domain::error::CustomError;
use super::models::ChaseStatementCSV;


#[derive(FromForm)]
pub struct CSVUpload<'r> {
    account_id: i64,
    file: Capped<TempFile<'r>>
}

pub struct Uploader {
}
// The number of parameters in MySQL must fit in a `u16`.
const BIND_LIMIT: usize = 65535;

impl Uploader {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn upload_chase(&self, pool: &Pool<MySql>, statements: &[ChaseStatementCSV], account_id: &i64) -> Result<(), CustomError> {
        conv_err!(query("TRUNCATE TABLE Statement").execute(pool).await)?;
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
        query.execute(pool).await.expect("insert should succeed");
        return Ok(());
    }
}

fn create_temp_dir_if_not_exists() -> Result<(), CustomError> {
    let mut path =  temp_dir().to_str().expect("temp dir should be valid string").to_string();
    path.push_str("\\HomeFinanceManager\\");
    let folder = Path::new(&path);
    if !folder.exists() {
        return conv_err!(std::fs::create_dir(folder))
    }
    Ok(())
}

fn get_temp_file_path() -> Result<PathBuf, CustomError> {
    let timestamp = SystemTime::now();
    let mut path = match temp_dir().to_str() {
        Some(r) => r.to_string(),
        None => return Err(CustomError::msg("temp dir should be valid string".to_string()))
    };
    path.push_str("\\HomeFinanceManager\\");
    path.push_str(&conv_err!(timestamp.duration_since(UNIX_EPOCH))?.as_millis().to_string());
    path.push_str(".csv");
    create_temp_dir_if_not_exists()?;
    let mut buf = PathBuf::new();
    buf.push(path);
    return Ok(buf);
}


#[allow(dead_code)]
#[post("/upload?type=chase", data = "<upload>")]
pub async fn upload_chase(mut upload: Form<CSVUpload<'_>>, uploader: &State<Box<Uploader>>, pool: &State<Pool<MySql>>) -> Result<(), CustomError> {
    if upload.account_id < 0 {
        return Err(CustomError::msg(String::from("Account cannot be less than 0")))
    }
    if upload.file.is_complete() {
        let path = get_temp_file_path()?;
        let folder = Path::new(&path);
        conv_err!(upload.file.persist_to(&folder).await)?;
        let file_handle = expect_err!(File::open(&path), "File should exist".to_string())?;
        let reader = BufReader::new(&file_handle);
        let mut csv_reader = Reader::from_reader(reader);
        let records = csv_reader.deserialize::<ChaseStatementCSV>();
        let mut good_records: Vec<ChaseStatementCSV> = Vec::new();
        for record in records {
            match record {
                Ok(r) => good_records.push(r),
                Err(_) => ()
            }
        }
        conv_err!(uploader.upload_chase(pool.inner(), good_records.as_slice(), &upload.account_id).await)?;
    }

    return Ok(())
}