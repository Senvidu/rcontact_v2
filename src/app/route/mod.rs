use crate::app::handler::emails::{
    create_email_for_person, delete_email, get_email, list_emails_for_person, update_email,
};
use crate::app::handler::healths::{livez, readyz};
use crate::app::handler::mobiles::{
    create_mobile_for_person, delete_mobile, get_mobile, list_mobiles_for_person, update_mobile,
};
use crate::app::handler::persons::{
    create_person, delete_person, get_person, list_persons, update_person,
};
use crate::app::state::AppState;
use axum::Router;
use axum::routing::get;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/livez", get(livez))
        .route("/readyz", get(readyz))
        .route("/api/v1/persons", get(list_persons).post(create_person))
        .route(
            "/api/v1/persons/{id}",
            get(get_person).put(update_person).delete(delete_person),
        )
        .route(
            "/api/v1/persons/{person_id}/mobiles",
            get(list_mobiles_for_person).post(create_mobile_for_person),
        )
        .route(
            "/api/v1/mobiles/{id}",
            get(get_mobile).put(update_mobile).delete(delete_mobile),
        )
        .route(
            "/api/v1/persons/{person_id}/emails",
            get(list_emails_for_person).post(create_email_for_person),
        )
        .route(
            "/api/v1/emails/{id}",
            get(get_email).put(update_email).delete(delete_email),
        )
        .with_state(state)
}
