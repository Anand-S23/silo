use axum::{http::StatusCode, Json};
use sqlx::PgPool;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    db::models::{
        User, 
        Presentation, 
        RegisterInput, 
        PresentationInput,
        PresentationJson
    }
};

impl User {
    pub async fn create(data: RegisterInput, pool: &PgPool) -> Option<User> {
        let sql = "
            INSERT into users (username, email, password) 
            VALUES ($1, $2, $3) 
            RETURNING *";
        
        sqlx::query_as(&sql)
            .bind(data.username)
            .bind(data.email)
            .bind(data.password)
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn find_by_id(id: &String, pool: &PgPool) -> Option<User> {
        sqlx::query_as(&"SELECT * FROM users WHERE id = $1 LIMIT 1")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn find_by_email(email: &String, pool: &PgPool) -> Option<User> {
        sqlx::query_as(&"SELECT * FROM users WHERE email = $1 LIMIT 1")
            .bind(email)
            .fetch_one(pool)
            .await
            .ok()
    }
}

impl Presentation {
    pub async fn create(
        data: PresentationInput, 
        owner_id: Uuid, pool: &PgPool) 
    -> Result<Presentation, (StatusCode, Json<serde_json::Value>)> {
        let filename = Uuid::new_v4().to_string();
        let file_path = format!("./presentations/{}.json", filename);

        let new_presentation = PresentationJson::new();
        let presentation_json = serde_json::to_string(&new_presentation).unwrap();

        let mut file = File::create(file_path)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "errors": [
                        {
                            "msg": format!("Error while creating slide json file: {}", e)
                        }
                    ]
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;
        
        file.write_all(presentation_json.as_bytes())
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "errors": [
                        {
                            "msg": format!("Error while creating slide json file: {}", e)
                        }
                    ]
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

        let sql = "
            INSERT into presentations (owner, name, file_name) 
            VALUES ($1, $2, $3) 
            RETURNING *";

        let presentation =  sqlx::query_as(&sql)
            .bind(owner_id.to_string())
            .bind(data.name)
            .bind(filename)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "errors": [
                        {
                            "msg": format!("Error while updating db: {}", e)
                        }
                    ]
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;
        
        Ok(presentation)
    }
}