use crate::app::dto::person::{CreatePersonRequest, UpdatePersonRequest};
use crate::app::service::person_service::PersonService;
use crate::app::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde_json::{Value, json};

// Handler functions for person-related endpoints

// GET /persons - List all persons
pub async fn list_persons(
    State(state): State<AppState>, //gets app state from axum extractors
) -> Result<Json<Value>, (StatusCode, Json<Value>)> //returns a JSON response with either the list of persons or an error message
{
    match PersonService::get_all(&state).await //calls the service layer to get all persons
    {
        Ok(persons) => Ok(Json(json!(persons))),//if successful, returns the list of persons as JSON
        //if there's an error, returns a 500 Internal Server Error with the error message
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

// GET /persons/:id - Get a person by ID
pub async fn get_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> //returns a JSON response with either the person data or an error message
{
    match PersonService::get_by_id(&state, id).await//calls the service layer to get a person by ID
    {
        Ok(person) => Ok(Json(json!(person))),//if successful, returns the person data as JSON
        //if the person is not found, returns a 404 Not Found with the error message
        Err(message) if message == "Person not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        //if there's any other error, returns a 500 Internal Server Error with the error message
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

// POST /persons - Create a new person
pub async fn create_person(
    State(state): State<AppState>,
    Json(payload): Json<CreatePersonRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> //returns a JSON response with either the created person data or an error message
{
    match PersonService::create(&state, payload).await //calls the service layer to create a new person with the provided payload
    {
        //if successful, returns the created person data as JSON with a 201 Created status
        Ok(person) => Ok((StatusCode::CREATED, Json(json!(person)))),
        //if the person name is missing, returns a 400 Bad Request with the error message
        Err(message) if message == "Person name is required" => {
            Err((StatusCode::BAD_REQUEST, Json(json!({ "error": message }))))
        }
        //if there's any other error, returns a 500 Internal Server Error with the error message
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

// PUT /persons/:id - Update a person by ID
pub async fn update_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePersonRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> //returns a JSON response with either the updated person data or an error message
{
    //calls the service layer to update a person by ID with the provided payload
    match PersonService::update(&state, id, payload).await {
        //if successful, returns the updated person data as JSON
        Ok(person) => Ok(Json(json!(person))),
        //if the person is not found, returns a 404 Not Found with the error message
        Err(message) if message == "Person not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        //if the person name is missing, returns a 400 Bad Request with the error message
        Err(message) if message == "Person name is required" => {
            Err((StatusCode::BAD_REQUEST, Json(json!({ "error": message }))))
        }
        //if there's any other error, returns a 500 Internal Server Error with the error message
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}

// DELETE /persons/:id - Delete a person by ID
pub async fn delete_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> //returns a JSON response with either a success message or an error message
{
    //calls the service layer to delete a person by ID
    match PersonService::delete(&state, id).await {
        //if successful, returns a success message along with the deleted person data as JSON
        Ok(person) => Ok(Json(json!({
            "message": "Person deleted successfully",
            "person": person
        }))),
        //if the person is not found, returns a 404 Not Found with the error message
        Err(message) if message == "Person not found" => {
            Err((StatusCode::NOT_FOUND, Json(json!({ "error": message }))))
        }
        //if there's any other error, returns a 500 Internal Server Error with the error message
        Err(message) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": message })),
        )),
    }
}
