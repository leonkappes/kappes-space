use actix_web::{HttpResponse, Error, web::Data};
use diesel::{prelude::*, sql_query};
use serde_json::json;

use crate::{database::postgres::Pool};

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("DOCS to be implemented here"))
}

pub async fn health(db: Data<Pool>) -> Result<HttpResponse, Error> {
    let conn_res = db.get();
    if let Ok(mut conn) = conn_res {
        let data = sql_query("SELECT 1").execute(&mut conn);
        match data {
            Ok(_) => Ok(HttpResponse::Ok().json(json!({
                "message": "Healthy",
            }))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                "message": "Error",
                "data": format!("{}", e)
            })))
        }
    }else if let Err(e) = conn_res {
        Ok(HttpResponse::InternalServerError().json(json!({
            "message": "Error",
            "data": format!("{}", e)
        })))
    } else {
        Ok(HttpResponse::InternalServerError().json(json!({
            "message": "Error",
        })))
    }
    
}