use actix_web::{error, web, Error, HttpResponse};
use diesel::result::Error::NotFound;
use serde::Deserialize;
use serde_json::json;

use crate::{auth::UserToken, database::postgres::Pool, models::user::{UserDTO, CreateUserDTO}};

#[derive(Deserialize)]
pub struct AuthFormData {
    username: String,
    password: String,
}

pub async fn login(
    db: web::Data<Pool>,
    form: web::Form<AuthFormData>,
) -> Result<HttpResponse, Error> {
    let user = UserDTO::get_user(form.username.to_owned(), db.get().unwrap());
    if let Ok(user) = user {
        let password_is_correct = match user.login(form.password.to_owned()) {
            Ok(it) => it,
            Err(err) => return Err(error::ErrorInternalServerError(err.to_string())),
        };
        if password_is_correct {
            let jwt = UserToken::generate_token(&user);
            return Ok(HttpResponse::Ok().json(json!({
                "message": "Sucessfully logged in",
                "jwt": &jwt
            })));
        } else {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "message": "Invalid username/password"
            })));
        }
    }

    Ok(HttpResponse::Unauthorized().json(json!({
        "message": "Invalid username/password"
    })))
}

pub async fn signup(
    db: web::Data<Pool>,
    form: web::Form<AuthFormData>,
) -> Result<HttpResponse, Error> {
    let user_query = UserDTO::get_user(form.username.to_owned(), db.get().unwrap());
    if let Err(err) = user_query {
        if err != NotFound {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "message": "Internal error while creating account, please try again later",
                "jwt": ""
            })))
        }
        match CreateUserDTO::new_user(form.username.to_owned(), form.password.to_owned(), crate::models::user::Permission::User, db.get().unwrap()) {
            Ok(user) => {
                let jwt = UserToken::generate_token(&user);
                return Ok(HttpResponse::Created().json(json!({
                    "message": "account created",
                    "jwt": &jwt
                })))
            }
            Err(_) => {
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "message": "Internal error while creating account, please try again later",
                    "jwt": ""
                })))
            }
        };
    }

    Ok(HttpResponse::BadRequest().json(json!({
        "message": "Username is already taken",
        "jwt": ""
    })))
}
