pub mod active;
pub mod data;
pub mod entities;

use entities::{prelude::*, *};
use sea_orm::*;

pub async fn create_db_connection(url: String) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(url.as_str()).await?;
    return Ok(db);
}
