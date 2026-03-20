use crate::entity::email;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set,
};

pub struct EmailRepository;

impl EmailRepository {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<email::Model>, DbErr> {
        email::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_by_person_id(
        db: &DatabaseConnection,
        person_id: i32,
    ) -> Result<Vec<email::Model>, DbErr> {
        email::Entity::find()
            .filter(email::Column::PersonId.eq(person_id))
            .all(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        person_id: i32,
        address: String,
        label: Option<String>,
    ) -> Result<email::Model, DbErr> {
        let active_model = email::ActiveModel {
            person_id: Set(person_id),
            address: Set(address),
            label: Set(label),
            ..Default::default()
        };

        active_model.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        address: String,
        label: Option<String>,
    ) -> Result<Option<email::Model>, DbErr> {
        let existing = email::Entity::find_by_id(id).one(db).await?;

        if let Some(existing) = existing {
            let mut active_model: email::ActiveModel = existing.into();
            active_model.address = Set(address);
            active_model.label = Set(label);

            let updated = active_model.update(db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<Option<email::Model>, DbErr> {
        let existing = email::Entity::find_by_id(id).one(db).await?;

        if let Some(existing) = existing {
            existing.clone().delete(db).await?;
            Ok(Some(existing))
        } else {
            Ok(None)
        }
    }
}
