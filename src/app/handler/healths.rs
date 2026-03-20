use axum::Json; // importing jason from axum framework
use serde_json::{Value, json}; // importing Value and json from serde_json crate

//async function to check if the application is live
pub async fn livez() -> Json<Value> {
    Json(json!({ "status": "live" })) //sending a json resopnse
}

//async function to check if the application is ready
pub async fn readyz() -> Json<Value> {
    Json(json!({ "status": "ready" })) //sending a json resopnse
}
