use std::{sync::Arc};

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use rand_core::OsRng;

use crate::{models::{BaseUserData, User}, AppState, auth::AuthValidation};

pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    Json(mut input): Json<BaseUserData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = AuthValidation::validate_register_data(&input);
    if result.len != 0 {
        return Err((StatusCode::BAD_REQUEST, Json(result.errors)));
    }

    if User::find_by_email(&input.email, &data.db).await.is_some() {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!(
            {
                "errors": [
                    {
                        "msg": "A user with with this email already exists",
                        "params": "email"
                    }
                ]
            }
        ))));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "errors": [{
                    "msg": format!("Error while hashing password: {}", e)
                }]
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    input.password = hashed_password;
    println!("{} - {} - {}", input.email, input.username, input.password);
    if User::create(input, &data.db).await.is_none() {
        let error_response = serde_json::json!({
            "errors": [{
                "msg": "DB error, could not create account"
            }]
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    Ok(Json(serde_json::json!({"status": "success"})))
}