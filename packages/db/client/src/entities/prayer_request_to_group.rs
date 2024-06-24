//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "prayer_request_to_group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub prayer_request_id: String,
    pub group_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Group,
    #[sea_orm(
        belongs_to = "super::prayer_request::Entity",
        from = "Column::PrayerRequestId",
        to = "super::prayer_request::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PrayerRequest,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<super::prayer_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PrayerRequest.def()
    }
}
