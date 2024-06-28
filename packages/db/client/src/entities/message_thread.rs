//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "message_thread")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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
    #[sea_orm(has_many = "super::message_thread_to_group::Entity")]
    MessageThreadToGroup,
    #[sea_orm(has_many = "super::message_to_message_thread::Entity")]
    MessageToMessageThread,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<super::message_thread_to_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageThreadToGroup.def()
    }
}

impl Related<super::message_to_message_thread::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageToMessageThread.def()
    }
}