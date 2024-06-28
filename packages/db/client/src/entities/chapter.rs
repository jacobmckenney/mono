//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "chapter")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub translation_id: String,
    pub book_id: String,
    pub title: String,
    pub verse_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::book::Entity",
        from = "Column::BookId",
        to = "super::book::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Book,
    #[sea_orm(
        belongs_to = "super::translation::Entity",
        from = "Column::TranslationId",
        to = "super::translation::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Translation,
    #[sea_orm(has_many = "super::verse::Entity")]
    Verse,
}

impl Related<super::book::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Book.def()
    }
}

impl Related<super::translation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Translation.def()
    }
}

impl Related<super::verse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Verse.def()
    }
}