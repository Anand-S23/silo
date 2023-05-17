use std::{sync::Arc};

use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use rand_core::OsRng;

use crate::{
    AppState, 
    models::{BaseUserData, User, LoginData}, 
    auth::{validate_register_data, sign_jwt}
};

pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    Json(mut input): Json<BaseUserData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = validate_register_data(&input);
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
                "errors": [
                    {
                        "msg": format!("Error while hashing password: {}", e)
                    }
                ]
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    input.password = hashed_password;
    let user = match User::create(input, &data.db).await {
        Some(u) => u,
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(
            serde_json::json!({
                "errors": [
                    {
                        "msg": "DB error, could not create account"
                    }
                ]
            })
        )))
    };

    let token = sign_jwt(user.id, data.env.jwt_secret.clone())
        .map_err(|e| {
            let error_response = serde_json::json!({
                "errors": [
                    {
                        "msg": format!("Error while signing token: {}", e)
                    }
                ]
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    Ok(Json(serde_json::json!({"token": token})))
}

pub async fn login_handler(
    State(data): State<Arc<AppState>>,
    Json(input): Json<LoginData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = match User::find_by_email(&input.email, &data.db).await {
        Some(u) => u,
        None => return Err((StatusCode::BAD_REQUEST, Json(
            serde_json::json!({
                "errors": [
                    {
                        "msg": "Invalid Credentials"
                    }
                ]
            })
        )))
    };

    let password_is_correct = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(input.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !password_is_correct {
        return Err((StatusCode::BAD_REQUEST, Json(
            serde_json::json!({
                "errors": [
                    {
                        "msg": "Invalid Credentials"
                    }
                ]
            })
        )));
    }

    let token = sign_jwt(user.id, data.env.jwt_secret.clone())
        .map_err(|e| {
            let error_response = serde_json::json!({
                "errors": [
                    {
                        "msg": format!("Error while signing token: {}", e)
                    }
                ]
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    Ok(Json(serde_json::json!({"token": token})))
}