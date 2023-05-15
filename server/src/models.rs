use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct BaseUserData {

    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}
