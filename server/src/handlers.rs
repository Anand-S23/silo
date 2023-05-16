use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
// use std::result;

use crate::{models::{BaseUserData}, AppState, auth::{validate_register_data}};

pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    Json(input): Json<BaseUserData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("{} - {} - {}", input.email, input.username, input.password);
    let vec = validate_register_data(&input);
    if vec.len() != 0 {
        let errors_json_string = serde_json::to_string(&vec);
        match errors_json_string {
           Ok(v) => println!("{v:?}"),
           Err(e) => println!("{e:?}") 
        }
    }

    Ok(Json(serde_json::json!({"status": "success"})))
}