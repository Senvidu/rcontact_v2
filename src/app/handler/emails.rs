use crate::app::dto::email::{CreateEmailRequest, UpdateEmailRequest};
use crate::app::service::email_service::EmailService;
use crate::app::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde_json::{Value, json};

pub async fn list_emails_for_person(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match EmailService::get_by_person_id(&state, person_id).await {
        Ok(emails) => Ok(Json(json!(emails))),
        Err(message) if message == "Person not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

pub async fn get_email(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match EmailService::get_by_id(&state, id).await {
        Ok(email) => Ok(Json(json!(email))),
        Err(message) if message == "Email not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

pub async fn create_email_for_person(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Json(payload): Json<CreateEmailRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match EmailService::create(&state, person_id, payload).await {
        Ok(email) => Ok((StatusCode::CREATED, Json(json!(email)))),
        Err(message) if message == "Person not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) if message == "Email address is required" => {
            Err((StatusCode::BAD_REQUEST, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

pub async fn update_email(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateEmailRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match EmailService::update(&state, id, payload).await {
        Ok(email) => Ok(Json(json!(email))),
        Err(message) if message == "Email not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) if message == "Email address is required" => {
            Err((StatusCode::BAD_REQUEST, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

pub async fn delete_email(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match EmailService::delete(&state, id).await {
        Ok(email) => Ok(Json(json!({
            "message": "Email deleted successfully",
            "email": email
        }))),
        Err(message) if message == "Email not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}
