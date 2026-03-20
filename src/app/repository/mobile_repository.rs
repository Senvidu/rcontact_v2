use crate::entity::mobile;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set,
};

//mobile repository struct to interact with the mobile entity in the database
pub struct MobileRepository;

//implementation of mobile repo
impl MobileRepository {
    //find mobile by id
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
        //returns an option of mobile model, if found, otherwise returns None
    ) -> Result<Option<mobile::Model>, DbErr> {
        mobile::Entity::find_by_id(id).one(db).await //waits for the database query to complete and returns the result
    }

    //find mobiles by person id
    pub async fn find_by_person_id(
        db: &DatabaseConnection,
        person_id: i32,
        //returns a vector of mobile models associated with the given person id
    ) -> Result<Vec<mobile::Model>, DbErr> {
        mobile::Entity::find()
            .filter(mobile::Column::PersonId.eq(person_id)) //filters the query to find mobiles with the person_id
            .all(db) // retrieves all matching records
            .await //waits for the database query to complete and returns the result
    }

    //create a new mobile
    pub async fn create(
        //parameters for creating a new mobile
        db: &DatabaseConnection,
        person_id: i32,
        number: String,
        label: Option<String>,
    ) -> Result<mobile::Model, DbErr> //returns the created mobile model or an error if the operation fails
    {
        let active_model = mobile::ActiveModel {
            person_id: Set(person_id), //sets the person_id field of the active model
            number: Set(number),       //sets the number field of the active model
            label: Set(label),         //sets the label field of the active model
            ..Default::default()       //initializes any remaining fields with their default values
        };

        active_model
            .insert(db) //inserts the new mobile records into the database
            .await //waits for the database operation to complete and returns the result
    }

    //update an existing mobile
    pub async fn update(
        //parameters for updating an existing mobile
        db: &DatabaseConnection,
        id: i32,
        number: String,
        label: Option<String>,
    ) -> Result<Option<mobile::Model>, DbErr> //Returns updated mobile or None if ID not found.
    {
        let existing = mobile::Entity::find_by_id(id).one(db).await?; //Finds mobile by ID and awaits the result.
        if let Some(existing) = existing {
            //if mobile exists stores it in existing
            let mut active_model: mobile::ActiveModel = existing.into(); //converts mobile model into an active model for updating
            active_model.number = Set(number); //sets the number field of the active model to the new value
            active_model.label = Set(label); //sets the label field of the active model to the new value

            let updated = active_model.update(db).await?; //updates the mobile recorod and awits if errors occurs retunrs it
            Ok(Some(updated)) //status ok when update is done
        } else {
            Ok(None) //returns None if the mobile with the given ID does not exist
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<Option<mobile::Model>, DbErr> {
        let existing = mobile::Entity::find_by_id(id).one(db).await?; //find the mobile by ID and awaits the result 

        //Returns deleted record or None if not found.
        if let Some(existing) = existing {
            //if mobile exists stores it in existing
            existing.clone().delete(db).await?; //Deletes record and awaits result; returns error if it fails.
            Ok(Some(existing)) //Returns the deleted record wrapped in Some if deletion is successful.
        } else {
            Ok(None) //Returns None if the mobile with the given ID does not exist.
        }
    }
}
