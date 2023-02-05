use std::sync::atomic::Ordering;

use actix_web::{
    error,
    web::{self, Data},
    Error, HttpResponse,
};
use serde_json::json;

use crate::{
    auth::UserToken,
    config::Config,
    database::postgres::Pool,
    models::user::{Permission, UserDTO},
};

pub async fn toggle_registration(
    db: web::Data<Pool>,
    creds: Option<Data<UserToken>>,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    let user = if let Some(creds) = creds {
        match UserDTO::get_user_by_id(creds.user_id, db.get().unwrap()) {
            Ok(it) => it,
            Err(_) => {
                return Err(error::ErrorForbidden(json!({
                    "message": "No permission to create a post"
                })))
            }
        }
    } else {
        return Err(error::ErrorForbidden(json!({
            "message": "No permission to create a post"
        })));
    };

    if user.permission != Permission::Admin {
        return Err(error::ErrorForbidden(json!({
            "message": "No permission to create a post"
        })));
    }

    let current = config.allow_signups.load(Ordering::SeqCst);

    config.allow_signups.store(!current, Ordering::SeqCst);

    Ok(HttpResponse::Ok().json(json!({
        "message":
            format!(
                "Toggled registration {}",
                if current { "off" } else { "on" }
            )
    })))
}
