mod domain;

use domain::{upload::{Uploader, upload_chase}, fetch::get_all};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

extern crate dotenv;
#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let connection = std::env::var("SQL_CONNECTION").expect("SQL_CONNECTION should be defined in dotenv file");
    let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&connection).await.expect("should be able to connect mysql");
    let uploader = Uploader::new();
    let _rocket = rocket::build()
        .manage(Box::new(uploader))
        .manage(pool)
        .mount("/", routes![upload_chase, get_all])
        .launch()
        .await?;

    Ok(())
}