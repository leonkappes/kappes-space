use crate::models::user::UserDTO;
use actix_web::{dev::ServiceRequest, error, Error, HttpMessage};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth, Config},
    AuthenticationError,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::json;

static KEY: [u8; 16] = *b"asd1235234234234";
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize, Clone)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub user_id: i64,
}

impl UserToken {
    pub fn generate_token(login: &UserDTO) -> String {
        let now = Utc::now().timestamp();
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.name.clone(),
            user_id: login.id.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )
        .unwrap()
    }
}

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub fn validate_token(cred: &str) -> Result<UserToken, String> {
    if let Ok(token_data) = decode_token(cred.to_owned()) {
        return Ok(token_data.claims);
    }
    return Err("Invalid Token".to_owned());
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(it) => return Ok(it.to_string()),
        Err(err) => return Err(error::ErrorInternalServerError(err)),
    }
}

pub async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match credentials {
        None => {
            return Err((
                error::ErrorUnauthorized(json!({"msg": "Unauthorized"})),
                req,
            ))
        }
        Some(cred) => match validate_token(cred.token()) {
            Ok(res) => {
                req.extensions_mut().insert(res);
                return Ok(req);
            }
            Err(msg) => {
                return Err((error::ErrorUnauthorized(json!({ "msg": msg })), req));
            }
        },
    };
}
