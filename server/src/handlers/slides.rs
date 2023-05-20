use std::sync::Arc;

use axum::{Extension, extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{db::models::{User, PresentationInput, Presentation}, AppState};


pub async fn create_presentation_handler(
    State(data): State<Arc<AppState>>,
    Json(mut input): Json<PresentationInput>,
    Extension(user): Extension<User>,
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