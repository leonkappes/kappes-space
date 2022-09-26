use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, TokenData, Validation, DecodingKey};
use serde::{Serialize, Deserialize};

use crate::models::user::User;

pub static KEY: [u8; 16] = *b"asd1235234234234";
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize, Clone)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub user_id: u64,
}

impl UserToken {
    pub fn generate_token(login: &User) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.username.clone(),
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

pub fn validate_token(cred: &str, ) -> Result<UserToken, String> {
    if let Ok(token_data) = decode_token(cred.to_owned()) {
        return Ok(token_data.claims);
    }
    return Err("Invalid Token".to_owned());
}