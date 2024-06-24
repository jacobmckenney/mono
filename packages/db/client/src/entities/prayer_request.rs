//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "prayer_request")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub user_id: String,
    pub title: String,
    pub content: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::passage_to_prayer_request::Entity")]
    PassageToPrayerRequest,
    #[sea_orm(has_many = "super::prayer_request_to_group::Entity")]
    PrayerRequestToGroup,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::passage_to_prayer_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PassageToPrayerRequest.def()
    }
}

impl Related<super::prayer_request_to_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PrayerRequestToGroup.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
