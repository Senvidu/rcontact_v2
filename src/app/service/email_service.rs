use crate::app::dto::email::{CreateEmailRequest, UpdateEmailRequest};
use crate::app::repository::email_repository::EmailRepository;
use crate::app::repository::person_repository::PersonRepository;
use crate::app::state::AppState;
use crate::entity::email;

pub struct EmailService;

impl EmailService {
    pub async fn get_by_id(state: &AppState, id: i32) -> Result<email::Model, String> {
        EmailRepository::find_by_id(&state.db, id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Email not found".to_string())
    }

    pub async fn get_by_person_id(
        state: &AppState,
        person_id: i32,
    ) -> Result<Vec<email::Model>, String> {
        let person_exists = PersonRepository::find_by_id(&state.db, person_id)
            .await
            .map_err(|e| e.to_string())?
            .is_some();

        if !person_exists {
            return Err("Person not found".to_string());
        }

        EmailRepository::find_by_person_id(&state.db, person_id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create(
        state: &AppState,
        person_id: i32,
        request: CreateEmailRequest,
    ) -> Result<email::Model, String> {
        let person_exists = PersonRepository::find_by_id(&state.db, person_id)
            .await
            .map_err(|e| e.to_string())?
            .is_some();

        if !person_exists {
            return Err("Person not found".to_string());
        }

        if request.address.trim().is_empty() {
            return Err("Email address is required".to_string());
        }

        EmailRepository::create(
            &state.db,
            person_id,
            request.address.trim().to_string(),
            request.label,
        )
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn update(
        state: &AppState,
        id: i32,
        request: UpdateEmailRequest,
    ) -> Result<email::Model, String> {
        if request.address.trim().is_empty() {
            return Err("Email address is required".to_string());
        }

        EmailRepository::update(
            &state.db,
            id,
            request.address.trim().to_string(),
            request.label,
        )
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Email not found".to_string())
    }

    pub async fn delete(state: &AppState, id: i32) -> Result<email::Model, String> {
        EmailRepository::delete(&state.db, id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Email not found".to_string())
    }
}
