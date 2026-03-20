use crate::app::dto::mobile::{CreateMobileRequest, UpdateMobileRequest};
use crate::app::repository::mobile_repository::MobileRepository;
use crate::app::repository::person_repository::PersonRepository;
use crate::app::state::AppState;
use crate::entity::mobile;

pub struct MobileService;

impl MobileService {
    pub async fn get_by_id(state: &AppState, id: i32) -> Result<mobile::Model, String> {
        MobileRepository::find_by_id(&state.db, id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Mobile not found".to_string())
    }

    pub async fn get_by_person_id(
        state: &AppState,
        person_id: i32,
    ) -> Result<Vec<mobile::Model>, String> {
        let person_exists = PersonRepository::find_by_id(&state.db, person_id)
            .await
            .map_err(|e| e.to_string())?
            .is_some();

        if !person_exists {
            return Err("Person not found".to_string());
        }

        MobileRepository::find_by_person_id(&state.db, person_id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create(
        state: &AppState,
        person_id: i32,
        request: CreateMobileRequest,
    ) -> Result<mobile::Model, String> {
        let person_exists = PersonRepository::find_by_id(&state.db, person_id)
            .await
            .map_err(|e| e.to_string())?
            .is_some();

        if !person_exists {
            return Err("Person not found".to_string());
        }

        if request.number.trim().is_empty() {
            return Err("Mobile number is required".to_string());
        }

        MobileRepository::create(
            &state.db,
            person_id,
            request.number.trim().to_string(),
            request.label,
        )
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn update(
        state: &AppState,
        id: i32,
        request: UpdateMobileRequest,
    ) -> Result<mobile::Model, String> {
        if request.number.trim().is_empty() {
            return Err("Mobile number is required".to_string());
        }

        MobileRepository::update(
            &state.db,
            id,
            request.number.trim().to_string(),
            request.label,
        )
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Mobile not found".to_string())
    }

    pub async fn delete(state: &AppState, id: i32) -> Result<mobile::Model, String> {
        MobileRepository::delete(&state.db, id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Mobile not found".to_string())
    }
}
