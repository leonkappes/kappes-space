use actix_web::{web, HttpResponse, Error, error};
use serde::Deserialize;
use serde_json::json;

use crate::{database::postgres::Pool, models::user::UserDTO};



#[derive(Deserialize)]
pub struct LoginFormData {
    username: String,
    password: String
}

// TODO: create jwt and return it
pub async fn login(db: web::Data<Pool>,  form: web::Form<LoginFormData>) -> Result<HttpResponse, Error> {
    let user = UserDTO::get_user(form.username.to_owned(), db.get().unwrap());
    if let Ok(user) = user {
        let password_is_correct = match user.login(form.password.to_owned()) {
            Ok(it) => it,
            Err(err) => return Err(error::ErrorInternalServerError(err.to_string())),
        };
        if password_is_correct {
            return Ok(HttpResponse::Ok().json(json!({
                "message": "successful login"
            })))
        }else {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "message": "login credentials invalid"
            })))
        }
    }

    Ok(HttpResponse::Unauthorized().json(json!({
        "message": "login credentials invalid"
    })))
}