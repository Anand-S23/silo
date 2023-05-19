use std::sync::Arc;

use axum::{
    routing::{get, post}, 
    Router, Json,
    response::IntoResponse
};

use crate::{
    AppState, 
    handlers::{
        auth::{register_handler, login_handler}
    }
};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .with_state(app_state)
}

pub async fn health_checker_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Silo backend api running!"
    }))
}
