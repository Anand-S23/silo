use serde::{Deserialize, Serialize};

// Auth Models

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

// Auth Input Models

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}


// Slides

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Presentation {
    pub id: uuid::Uuid,
    pub owner: uuid::Uuid,
    pub name: String,
    pub filename: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    Text,
    Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub item_type: ItemType,
    pub x: i32,
    pub y: i32,
    pub h: i32,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slide {
    pub index: i32,
    pub items: Vec<Item>
}

// Slides Input Models

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationInput {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationJson {
    pub slides: Vec<Slide>
}

impl PresentationJson {
    pub fn new() -> Self {
        PresentationJson { 
            slides: Vec::new()
        }
    }
}
