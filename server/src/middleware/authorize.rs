use std::sync::Arc;

use axum::{http::{Request, StatusCode, header}, middleware::Next, response::IntoResponse, Json, extract::State};
use axum_extra::extract::CookieJar;

use crate::{AppState, utils::jwt::verify_jwt, db::models::User};

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| {
        let error_response = serde_json::json!({
            "errors": [
                {
                    "msg": format!("Invalid request, must be logged in")
                }
            ]
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let jwt_payload = verify_jwt(&token.as_str(), data.env.jwt_secret.clone())
        .map_err(|e| {
            let error_response = serde_json::json!({
                "errors": [
                    {
                        "msg": format!("Error while verifying jwt: {}", e)
                    }
                ]
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user_id = uuid::Uuid::parse_str(&jwt_payload.sub.to_string().as_str())
        .map_err(|_| {
            let error_response = serde_json::json!({
                "errors": [
                    {
                        "msg": format!("Invalid Token")
                    }
                ]
            });
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;

    let user = match User::find_by_id(&user_id.to_string(), &data.db).await {
        Some(u) => u,
        None => return Err((StatusCode::UNAUTHORIZED, Json(
            serde_json::json!({
                "errors": [
                    {
                        "msg": "User with matching token does not exist"
                    }
                ]
            })
        )))
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
