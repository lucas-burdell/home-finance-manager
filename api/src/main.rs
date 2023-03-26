use rocket::{fs::TempFile, form::Form, data::Capped};
use std::{io, fs};

extern crate dotenv;
#[macro_use] extern crate rocket;


#[derive(FromForm)]
struct CSVUpload<'r> {
    account_id: i64,
    file: Capped<TempFile<'r>>
}

#[derive(Debug, serde::Deserialize)]
struct ChaseStatementCSV {
    transaction_date: String, 
    post_date: String,
    description: String,
    category: String,
    r#type: String,
    amount: f64,
    memo: String
}

#[post("/upload?type=chase", data = "<upload>")]
async fn upload(mut upload: Form<CSVUpload<'_>>) -> io::Result<()> {
    if upload.account_id < 0 || upload.account_id > 1 {
        panic!("uploaded account id cannot be {0}", upload.account_id);
    }
    if upload.file.is_complete() {
        let timestamp = std::time::SystemTime::now();
        let path = format!("/tmp/{:?}.csv", timestamp.duration_since(std::time::UNIX_EPOCH).expect("time went backwards"));
        upload.file.persist_to(&path).await?;
        let file_handle = fs::File::open(&path).expect("file should exist");
        let reader = std::io::BufReader::new(&file_handle);
        let mut csv_reader = csv::Reader::from_reader(reader);
        for result in csv_reader.deserialize::<ChaseStatementCSV>() {
            let record = result.expect("record should deserialize");
        }
    }

    return Ok(())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![upload])
        .launch()
        .await?;

    Ok(())
}