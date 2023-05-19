use chrono::{Utc, Duration};
use jsonwebtoken::{Header, EncodingKey, DecodingKey, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct JWTPayload {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl JWTPayload {
    pub fn new(id: Uuid) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign_jwt(id: Uuid, jwt_secret: String) -> Result<String, Error> {
    return jsonwebtoken::encode(
        &Header::default(),
        &JWTPayload::new(id),
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    );
}

pub fn verify_jwt(token: &str, jwt_secret: String) -> Result<JWTPayload, Error> {
    return jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims);
}