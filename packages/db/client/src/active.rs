use chrono::Utc;
use sea_orm::{
    prelude::async_trait::async_trait, ActiveModelBehavior, ActiveValue, ConnectionTrait, DbErr,
};

use crate::entities::{
    book, chapter, comment, group, group_membership, message, message_thread,
    message_thread_to_group, message_to_message_thread, passage, passage_to_prayer,
    passage_to_prayer_request, prayer, prayer_request, prayer_request_to_group, session,
    translation, user, verse,
};

#[async_trait]
pub trait CustomBehavior: ActiveModelBehavior + HasTimestamps {
    async fn before_save_custom<A, C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
        Self: Sized,
    {
        println!("before_save_custom");
        let current_time = Utc::now().naive_utc();
        self.set_updated_at(current_time);
        if insert {
            self.set_created_at(current_time)
        }

        Ok(self)
    }
}

#[async_trait]
impl<T> CustomBehavior for T where T: ActiveModelBehavior + Send + HasTimestamps {}

pub trait HasTimestamps {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime>;
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime>;
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime);
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime);
}

impl HasTimestamps for chapter::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }

    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }

    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }

    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for chapter::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<chapter::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for book::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }

    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }

    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }

    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for book::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<book::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for comment::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }

    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }

    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }

    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for comment::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<comment::ActiveModel, C>(self, db, insert).await
    }
}
impl HasTimestamps for group::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for group::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<group::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for group_membership::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for group_membership::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<group_membership::ActiveModel, C>(self, db, insert)
            .await
    }
}

impl HasTimestamps for session::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for session::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<session::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for translation::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for translation::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<translation::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for verse::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for verse::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<verse::ActiveModel, C>(self, db, insert).await
    }
}
impl HasTimestamps for message::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for message::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<message::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for message_thread::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for message_thread::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<message_thread::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for message_thread_to_group::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for message_thread_to_group::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<message_thread_to_group::ActiveModel, C>(
            self, db, insert,
        )
        .await
    }
}

impl HasTimestamps for message_to_message_thread::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for message_to_message_thread::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<message_to_message_thread::ActiveModel, C>(
            self, db, insert,
        )
        .await
    }
}

impl HasTimestamps for passage::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for passage::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<passage::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for passage_to_prayer::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for passage_to_prayer::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<passage_to_prayer::ActiveModel, C>(self, db, insert)
            .await
    }
}

impl HasTimestamps for prayer::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for prayer::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<prayer::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for prayer_request::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for prayer_request::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<prayer_request::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for prayer_request_to_group::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for prayer_request_to_group::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<prayer_request_to_group::ActiveModel, C>(
            self, db, insert,
        )
        .await
    }
}

impl HasTimestamps for user::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for user::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<user::ActiveModel, C>(self, db, insert).await
    }
}

impl HasTimestamps for passage_to_prayer_request::ActiveModel {
    fn created_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.created_at.clone()
    }
    fn updated_at(&self) -> ActiveValue<chrono::NaiveDateTime> {
        self.updated_at.clone()
    }
    fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
        self.created_at = ActiveValue::Set(created_at);
    }
    fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
        self.updated_at = ActiveValue::Set(updated_at);
    }
}

#[async_trait]
impl ActiveModelBehavior for passage_to_prayer_request::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        CustomBehavior::before_save_custom::<passage_to_prayer_request::ActiveModel, C>(
            self, db, insert,
        )
        .await
    }
}
