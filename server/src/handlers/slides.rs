use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Extension};

use crate::{db::models::{PresentationInput, Presentation, User}, AppState};

pub async fn create_presentation_handler(
    Extension(user): Extension<User>,
    State(data): State<Arc<AppState>>,
    Json(mut input): Json<PresentationInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if input.name.len() == 0 {
        input.name = "Untitled presentation".to_string();
    }

    let presentation = Presentation::create(input, user.id, &data.db)
        .await 
        .map_err(|e| {
            e
        })?;
    
    let res = serde_json::to_string(&presentation).unwrap();
    Ok(Json(serde_json::json!({"presentation_data": res})))
}

pub async fn read_presentations_handler(
    Extension(user): Extension<User>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let presentations = Presentation::all(user.id, &data.db)
        .await 
        .map_err(|e| {
            e
        })?;

    let res = serde_json::to_string(&presentations).unwrap();
    Ok(Json(serde_json::json!({"presentation_data": res})))
}

