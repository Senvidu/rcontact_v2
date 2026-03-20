use crate::entity::person;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, Set};

//person repository struct to interact with the person entity in the database
pub struct PersonRepository;

impl PersonRepository {
    //find all persons in the database
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<person::Model>, DbErr> //Returns all persons as a vector from the database or an error
    {
        person::Entity::find().all(db).await //finds all records and waits for the database query to complete, returning the result
    }

    //find person by id
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<person::Model>, DbErr> {
        person::Entity::find_by_id(id).one(db).await //finds by id and awaits
    }

    //create a new person
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        notes: Option<String>,
    ) -> Result<person::Model, DbErr> {
        let active_model = person::ActiveModel //creates a new active model for the person entity
        {
            name: Set(name),//sets the name field of the active model
            notes: Set(notes),// sets the notes field of the active model
            ..Default::default()// initializes any remaining fields with their default values
        };

        active_model.insert(db).await //inserts into database and awaits 
    }

    //update an existing person
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        name: String,
        notes: Option<String>,
    ) -> Result<Option<person::Model>, DbErr> {
        let existing = person::Entity::find_by_id(id).one(db).await?; //finds the person assigns it to existing and awaiting

        if let Some(existing) = existing
        //if the person exists, it updates the record
        {
            let mut active_model: person::ActiveModel = existing.into(); // converts the existing person model into an active model for updating
            active_model.name = Set(name); //sets the name field of the active model
            active_model.notes = Set(notes); // sets the notes field of the active model

            let updated = active_model.update(db).await?; // updates the record and awaits the result
            Ok(Some(updated)) //returns the updated person model wrapped in Some if the update was successful
        } else {
            Ok(None) //returns None if the person with the specified ID was not found
        }
    }

    //delete a person by id
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<Option<person::Model>, DbErr> {
        let existing = person::Entity::find_by_id(id).one(db).await?; //finds the peron and returns it as existing and awaits the result

        if let Some(existing) = existing
        //if the person exists on existing
        {
            existing.clone().delete(db).await?; //if the person exsist delete the record and awaits
            Ok(Some(existing)) // return the deleted person
        } else {
            Ok(None)
        }
    }
}
