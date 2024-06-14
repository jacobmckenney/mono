mod api;
mod library;

use actix_identity::IdentityMiddleware;
use actix_session::{
    config::CookieContentSecurity, storage::CookieSessionStore, SessionMiddleware,
};
use actix_web::{
    cookie::SameSite,
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use api::{
    auth,
    middlewares::{self, user_auth::SessionUser},
};
use library::{
    cors,
    state::{self, AppState},
};

const NUM_WORKERS: usize = 4;
const PORT: u16 = 8080;

// https://mureithi.me/blog/simple-authentication-approach-with-actix-web

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_state = state::create_app_state().await;
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(cors::configure_cors())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    app_state.encryption_key.clone(),
                )
                .cookie_domain(Some(String::from("localhost")))
                .cookie_path(String::from("/"))
                .cookie_name(app_state.app_name.clone())
                .cookie_secure(true)
                .cookie_content_security(CookieContentSecurity::Private)
                // TODO: configure same site properly depending on the environment
                .cookie_same_site(SameSite::None)
                .cookie_http_only(false)
                .build(),
            )
            .app_data(web::Data::new(app_state.clone()))
            .service(auth::auth_router())
            .service(
                web::scope("")
                    .wrap(middlewares::user_auth::AddUser::new())
                    .service(find_user)
                    .route("/", web::get().to(HttpResponse::Ok)),
            )
    })
    .bind(("127.0.0.1", PORT))?
    .workers(NUM_WORKERS)
    .run()
    .await;
    Ok(())
}

#[get("/user")]
async fn find_user(user: SessionUser, app: Data<AppState>) -> Result<impl Responder> {
    let user = app.db.get_user(&user.email).await.unwrap().unwrap();
    return Ok(web::Json(user));
}
