use std::sync::Arc;

use axum::{
    routing::{get, post}, 
    Router, Json,
    response::IntoResponse, middleware
};

use crate::{
    AppState, 
    handlers::{
        auth::{register, login}, 
        slides::{create_presentation, read_presentations}
    }, 
    middleware::authorize::auth
};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/slides/create", post(create_presentation)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)))
        .route("/api/slides/all", get(read_presentations)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)))
        .with_state(app_state)
}

pub async fn health_checker_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Silo backend api running!"
    }))
}
