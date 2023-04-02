use rocket::{State};
use rocket::serde::json::Json;
use sqlx::{Pool, MySql, query_as};

use crate::{domain::{models::Statement, error::CustomError}, conv_err};


#[allow(dead_code)]
#[get("/all?<accountid>", format = "json")]
pub async fn get_all(accountid: &str, pool: &State<Pool<MySql>>) -> Result<Json<Vec<Statement>>, CustomError> {
    Ok(Json(conv_err!(query_as::<_, Statement>("SELECT * FROM Statement WHERE AccountID = ? ORDER BY TransactionDate ASC")
    .bind(accountid)
    .fetch_all(pool.inner())
    .await)?))
}