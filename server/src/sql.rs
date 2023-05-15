use sqlx::PgPool;
use uuid::Uuid;
use std::result;

use crate::models::{User, BaseUserData};

impl User {
    pub async fn create(data: BaseUserData, pool: &PgPool) -> Result<User, String> {
        let sql = "
            INSERT into users (username, email, password) 
            VALUES ($1, $2, $3) 
            RETURNING *";
        
        Ok(sqlx::query_as(&sql)
            .bind(data.name)
            .bind(data.email)
            .bind(data.password)
            .fetch_one(pool)
            .await?)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<User, String> {
        Ok(sqlx::query_as(&"SELECT * FROM users WHERE id = $1 LIMIT 1")
            .bind(id)
            .fetch_one(pool)
            .await?)
    }

    pub async fn find_by_email(email: String, pool: &PgPool) -> Result<User, String> {
        Ok(sqlx::query_as(&"SELECT * FROM users WHERE email = $1 LIMIT 1")
            .bind(email)
            .fetch_one(pool)
            .await?)
    }
}