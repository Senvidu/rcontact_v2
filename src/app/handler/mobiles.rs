use crate::app::dto::mobile::{CreateMobileRequest, UpdateMobileRequest};
use crate::app::service::mobile_service::MobileService;
use crate::app::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde_json::{Value, json};

pub async fn list_mobiles_for_person(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match MobileService::get_by_person_id(&state, person_id).await {
        Ok(mobiles) => Ok(Json(json!(mobiles))),
        Err(message) if message == "Person not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

pub async fn get_mobile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match MobileService::get_by_id(&state, id).await {
        Ok(mobile) => Ok(Json(json!(mobile))),
        Err(message) if message == "Mobile not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

pub async fn create_mobile_for_person(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Json(payload): Json<CreateMobileRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match MobileService::create(&state, person_id, payload).await {
        Ok(mobile) => Ok((StatusCode::CREATED, Json(json!(mobile)))),
        Err(message) if message == "Person not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) if message == "Mobile number is required" => {
            Err((StatusCode::BAD_REQUEST, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

pub async fn update_mobile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMobileRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match MobileService::update(&state, id, payload).await {
        Ok(mobile) => Ok(Json(json!(mobile))),
        Err(message) if message == "Mobile not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) if message == "Mobile number is required" => {
            Err((StatusCode::BAD_REQUEST, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

pub async fn delete_mobile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match MobileService::delete(&state, id).await {
        Ok(mobile) => Ok(Json(json!({
            "message": "Mobile deleted successfully",
            "mobile": mobile
        }))),
        Err(message) if message == "Mobile not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}
