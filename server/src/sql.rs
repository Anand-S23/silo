use sqlx::PgPool;

use crate::models::{User, BaseUserData};

impl User {
    pub async fn create(data: BaseUserData, pool: &PgPool) -> Option<User> {
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

    pub async fn find_by_email(email: &String, pool: &PgPool) -> Option<User> {
        sqlx::query_as(&"SELECT * FROM users WHERE email = $1 LIMIT 1")
            .bind(email)
            .fetch_one(pool)
            .await
            .ok()
    }
}