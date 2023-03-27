mod domain;

use domain::upload::{ChaseStatementCSV, Uploader};
use rocket::{fs::TempFile, form::Form, data::Capped, State};
use std::{fs};
use dotenv::dotenv;

extern crate dotenv;
#[macro_use] extern crate rocket;


#[derive(FromForm)]
struct CSVUpload<'r> {
    account_id: i64,
    file: Capped<TempFile<'r>>
}


#[derive(Debug, Clone, Responder)]
#[response(status = 500, content_type = "json")]
struct MyError(String);

fn create_temp_dir_if_not_exists() {
    let mut path =  std::env::temp_dir().to_str().expect("temp dir should be valid string").to_string();
    path.push_str("\\HomeFinanceManager\\");
    let folder = std::path::Path::new(&path);
    if !folder.exists() {
        std::fs::create_dir(folder).expect("folder should be created");
    }
}

#[post("/upload?type=chase", data = "<upload>")]
async fn upload(mut upload: Form<CSVUpload<'_>>, uploader: &State<Box<Uploader>>) -> Result<(), MyError> {
    if upload.account_id < 0 {
        return Err(MyError("Account cannot be less than 0".to_string()))
    }
    if upload.file.is_complete() {
        let timestamp = std::time::SystemTime::now();
        let mut path =  std::env::temp_dir().to_str().expect("temp dir should be valid string").to_string();
        path.push_str("\\HomeFinanceManager\\");
        path.push_str(&timestamp.duration_since(std::time::UNIX_EPOCH).expect("time went backwards").as_millis().to_string());
        path.push_str(".csv");
        create_temp_dir_if_not_exists();
        let folder = std::path::Path::new(&path);
        upload.file.persist_to(&folder).await.expect("no fail");
        let file_handle = fs::File::open(&path).expect("file should exist");
        let reader = std::io::BufReader::new(&file_handle);
        let mut csv_reader = csv::Reader::from_reader(reader);
        let records = csv_reader.deserialize::<ChaseStatementCSV>();
        let mut good_records: Vec<ChaseStatementCSV> = Vec::new();
        for record in records {
            match record {
                Ok(r) => good_records.push(r),
                Err(e) => return Err(MyError(e.to_string()))
            }
        }
        uploader.upload_chase(good_records.as_slice(), &upload.account_id).await.expect("upload should work");
    }

    return Ok(())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let uploader = Uploader::new().await;
    let _rocket = rocket::build()
        .manage(Box::new(uploader))
        .mount("/", routes![upload])
        .launch()
        .await?;

    Ok(())
}