use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "mobiles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub person_id: i32,
    pub number: String,
    pub label: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]

//defines the relation between the mobile and the person table
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::person::Entity",//mobile belongs to person
        from = "Column::PersonId",//mobile is connected to a person through person_id 
        to = "super::person::Column::Id",//person_id is from person table
        on_update = "Cascade",//if a person is updated update the mobile as well
        on_delete = "Cascade"//if a person is deleted delete the mobile as well
    )]
    Person,
}

//defines the relation between the mobile and the person table
impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
