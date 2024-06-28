use actix_identity::Identity;
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder, Scope,
};
use db::data::user::get_user;
use serde::de;

use crate::{api::middlewares::user_auth::SessionUser, library::state::AppState};

pub fn admin_router() -> Scope {
    web::scope("/admin").service(dev_action)
}

#[post("/dev-action")]
pub async fn dev_action(user: SessionUser, app: Data<AppState>) -> impl Responder {
    // TODO: factor into middleware
    let full_user = get_user(&app.db, &user.email).await.unwrap().unwrap();
    if full_user.is_admin {
        return HttpResponse::Unauthorized().finish();
    }
    println!("User: {:?}", user.email);
    HttpResponse::Ok().finish()
}
