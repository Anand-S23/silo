use std::sync::Arc;

use axum::{
    routing::{get, post}, 
    Router, Json,
    response::IntoResponse, middleware
};

use crate::{
    AppState, 
    handlers::{
        auth::{register_handler, login_handler}, slides::create_presentation_handler
    }, middleware::authorize::auth
};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/slides/create", post(create_presentation_handler)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)))
        .with_state(app_state)
}

pub async fn health_checker_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Silo backend api running!"
    }))
}
