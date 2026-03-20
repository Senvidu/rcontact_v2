use crate::app::dto::person::{CreatePersonRequest, UpdatePersonRequest};
use crate::app::repository::person_repository::PersonRepository;
use crate::app::state::AppState;
use crate::entity::person;

// Service layer for managing Person entities
pub struct PersonService;

// Implementation of the PersonService
impl PersonService {
    // Get all persons
    pub async fn get_all(state: &AppState) -> Result<Vec<person::Model>, String> {
        PersonRepository::find_all(&state.db) //calls repo method to get all persons
            .await //waits for the async operation to complete
            .map_err(|e| e.to_string()) //converts any error to a string
    }

    // Get a person by ID
    pub async fn get_by_id(state: &AppState, id: i32) -> Result<person::Model, String> {
        PersonRepository::find_by_id(&state.db, id) //calls repo
            .await //waits for the async operation to complete
            .map_err(|e| e.to_string())? //converts any error to a string
            .ok_or_else(|| "Person not found".to_string())
    }

    // Create a new person
    pub async fn create(
        state: &AppState,
        request: CreatePersonRequest,
    ) -> Result<person::Model, String> {
        if request.name.trim().is_empty() {
            return Err("Person name is required".to_string()); //if name is empty, return an error
        }

        PersonRepository::create(&state.db, request.name.trim().to_string(), request.notes) //calls repo to create a new person
            .await //waits for the async operation to complete
            .map_err(|e| e.to_string()) //converts any error to a string
    }

    // Update an existing person
    pub async fn update(
        state: &AppState,
        id: i32,
        request: UpdatePersonRequest,
    ) -> Result<person::Model, String> {
        if request.name.trim().is_empty() {
            return Err("Person name is required".to_string());
        }

        //calls repo to update
        PersonRepository::update(
            &state.db,
            id,
            request.name.trim().to_string(),
            request.notes,
        )
        .await //waits for the async operation to complete
        .map_err(|e| e.to_string())? //converts any error to a string
        .ok_or_else(|| "Person not found".to_string()) //if person is not found return an error
    }

    // Delete a person by ID
    pub async fn delete(state: &AppState, id: i32) -> Result<person::Model, String> {
        PersonRepository::delete(&state.db, id) //calls repo to delete a person
            .await //waits for the async operation to complete
            .map_err(|e| e.to_string())? //converts any error to a string
            .ok_or_else(|| "Person not found".to_string()) //if person is not found return an error
    }
}
