use sqlx::PgPool;
// use uuid::Uuid;

use crate::{
    db::models::{
        User, 
        Presentation, 
        RegisterInput, 
        // PresentationInput
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

    pub async fn find_by_email(email: &String, pool: &PgPool) -> Option<User> {
        sqlx::query_as(&"SELECT * FROM users WHERE email = $1 LIMIT 1")
            .bind(email)
            .fetch_one(pool)
            .await
            .ok()
    }
}

impl Presentation {
    /*
    pub async fn create(data: PresentationInput, pool: &PgPool) -> Option<Presentation> {
        
        let filename = Uuid::new_v4().to_string();
        let file_path = format!("./presentations/{}.json", filename);

        let sql = "
            INSERT into presentations (owner, name, file_name) 
            VALUES ($1, $2, $3) 
            RETURNING *";

        sqlx::query_as(&sql)
            .bind(data.owner.to_string())
            .bind(data.name)
            .bind(file_path)
            .fetch_one(pool)
            .await
            .ok()
    }
     */
}