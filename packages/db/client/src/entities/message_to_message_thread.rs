//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "message_to_message_thread")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub message_id: String,
    pub message_thread_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::message::Entity",
        from = "Column::MessageId",
        to = "super::message::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Message,
    #[sea_orm(
        belongs_to = "super::message_thread::Entity",
        from = "Column::MessageThreadId",
        to = "super::message_thread::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MessageThread,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::message_thread::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageThread.def()
    }
}
