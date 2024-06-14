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
        email: &str,
        name: Option<&str>,
        image: Option<&str>,
    ) -> Result<(), DbErr> {
        let db_image = image.map(|s| s.to_string());
        let db_name = name.map(|s| s.to_string());
        let new_user = user::ActiveModel {
            // TODO: generate utility for creating unique ids
            id: Set(String::from(format!("user_{}", Uuid::new_v4()))),
            name: Set(db_name),
            email: Set(String::from(email)),
            image: Set(db_image),
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
