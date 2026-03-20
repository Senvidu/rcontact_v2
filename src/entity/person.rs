use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "persons")] //database table name

//structure of the table
pub struct Model {
    #[sea_orm(primary_key)] //sets id as primary key
    pub id: i32,
    pub name: String,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)] //adds features to the relation enum

//defines the relation between the person and the mobile and email tables
pub enum Relation {
    #[sea_orm(has_many = "super::mobile::Entity")] //one person has many mobiles
    Mobile,
    #[sea_orm(has_many = "super::email::Entity")] //one person has many emails
    Email,
}

/*
defines the relation between the person and the mobile table
person entity is connected to the mobile entities through the relation enum
*/
impl Related<super::mobile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Mobile.def()
    }
}

/*
defines the relation between the person and the email table
person entity is connected to the email entities through the relation enum
*/
impl Related<super::email::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Email.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
