use actix_web::{web::Data, Error, HttpResponse};
use diesel::{prelude::*, sql_query};
use serde_json::json;

use crate::database::postgres::Pool;

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("DOCS to be implemented here"))
}

pub async fn health(db: Data<Pool>) -> Result<HttpResponse, Error> {
    let data = sql_query("SELECT 1").execute(&mut db.get().unwrap());
    if let Err(e) = data {
        Ok(HttpResponse::InternalServerError().json(json!({
            "message": "Error",
            "data": format!("{}", e)
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "message": "Healthy",
        })))
    }
}
