use sqlx::PgPool;
use uuid::Uuid;
use std::result;

use crate::models::User;

impl User {
    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<User, String> {
        Ok(sqlx::query_as(&"SELECT * FROM users WHERE id = $1 LIMIT 1")
            .bind(id)
            .fetch_one(pool)
            .await?)
    }

    pub async fn find_by_id(email: String, pool: &PgPool) -> Result<User, String> {
        Ok(sqlx::query_as(&"SELECT * FROM users WHERE email = $1 LIMIT 1")
            .bind(email)
            .fetch_one(pool)
            .await?)
    }
}