use sea_orm::{prelude::*, EntityOrSelect, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{user, User};

pub struct InsertUserData {
    pub email: String,
    pub name: Option<String>,
    pub image: Option<String>,
}
pub async fn insert_user(db: &DatabaseConnection, data: InsertUserData) -> Result<(), DbErr> {
    let db_image = data.image.map(|s| s.to_string());
    let db_name = data.name.map(|s| s.to_string());
    let new_user = user::ActiveModel {
        // TODO: generate utility for creating unique ids
        id: Set(String::from(format!("user_{}", Uuid::new_v4()))),
        name: Set(db_name),
        email: Set(String::from(data.email)),
        image: Set(db_image),
        ..Default::default()
    };
    user::ActiveModel::insert(new_user, db).await?;
    Ok(())
}

// TODO: factor shared code into shared library
pub fn is_admin(email: &str) -> bool {
    let admins = vec!["jake.g.mckenney@gmail.com"];
    return admins.contains(&email);
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    #[serde(rename(serialize = "isAdmin"))]
    pub is_admin: bool,
    #[serde(flatten)]
    pub user: user::Model,
}

pub async fn get_user(db: &DatabaseConnection, email: &str) -> Result<Option<UserData>, DbErr> {
    let user = User::find()
        .select()
        .columns(vec![
            user::Column::Id,
            user::Column::Name,
            user::Column::Email,
            user::Column::Image,
        ])
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?;
    Ok(user.map(|user| UserData {
        is_admin: is_admin(&user.email),
        user,
    }))
}
