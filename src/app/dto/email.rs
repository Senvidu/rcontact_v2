use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: i32,
    pub person_id: i32,
    pub address: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEmailRequest {
    pub address: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmailRequest {
    pub address: String,
    pub label: Option<String>,
}
