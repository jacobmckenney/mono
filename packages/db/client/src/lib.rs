pub mod entities;
use entities::{prelude::*, *};
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct DB {
    connection: DatabaseConnection,
}

impl DB {
    pub async fn new(url: String) -> Result<Self, DbErr> {
        let db = Database::connect(url.as_str()).await?;
        Ok(DB { connection: db })
    }

    pub async fn insert_user(
        &self,
        name: &str,
        email: &str,
        image: Option<&str>,
    ) -> Result<(), DbErr> {
        let dbImage: Option<String> = match image {
            Some(image) => Some(String::from(image)),
            None => None,
        };
        let new_user = user::ActiveModel {
            // TODO: generate utility for creating unique ids
            id: Set(String::from(format!("user_{}", Uuid::new_v4()))),
            name: Set(String::from(name)),
            email: Set(String::from(email)),
            image: Set(dbImage),
            ..Default::default()
        };
        User::insert(new_user).exec(&self.connection).await?;
        Ok(())
    }

    pub async fn get_user(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
        let user = User::find()
            .select()
            .columns(vec![
                user::Column::Id,
                user::Column::Name,
                user::Column::Email,
                user::Column::Image,
            ])
            .filter(user::Column::Email.eq(email))
            .one(&self.connection)
            .await?;
        Ok(user)
    }
}
