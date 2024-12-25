use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: i8,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/*
Notable: for Nullable column, you can use type Option<T> 
         so it value can be Null or T
Notable: BOOLEAN in MySQL is TINYINT(1) 
         so we can use i8 to retrieve the record and later we can parse to Boolean

*/