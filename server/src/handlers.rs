use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
// use std::result;

use crate::{models::{BaseUserData}, AppState, auth::{validate_username, validate_email, validate_password}};

pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    Json(input): Json<BaseUserData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("{} - {} - {}", input.email, input.username, input.password);
    let vu = validate_username(&input.username);
    let ve = validate_email(&input.email);
    let vp = validate_password(&input.password);
    println!("{} - {} - {}", vu, ve, vp);
    Ok(Json(serde_json::json!({"status": "success"})))
}