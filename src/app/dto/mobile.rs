use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mobile {
    pub id: i32,
    pub person_id: i32,
    pub number: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMobileRequest {
    pub number: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMobileRequest {
    pub number: String,
    pub label: Option<String>,
}
