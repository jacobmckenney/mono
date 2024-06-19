mod api;
mod library;

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{
    config::CookieContentSecurity, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{
    cookie::SameSite,
    get,
    web::{self, Data},
    App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Result,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_state = state::create_app_state().await;
    let environment = app_state.environment.clone();
    let address = match environment.as_str() {
        "production" => "0.0.0.0",
        _ => "127.0.0.1",
    };
    println!(
        "Starting server in {} environment. Address: {}",
        environment.clone(),
        address
    );
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(cors::configure_cors(app_state.environment.clone().as_str()))
            .wrap(IdentityMiddleware::default())
            .wrap(match app_state.environment.clone().as_str() {
                "production" => SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    app_state.encryption_key.clone(),
                )
                .cookie_domain(Some(String::from("app.ekklesia.dev")))
                .cookie_path(String::from("/"))
                .cookie_name(app_state.app_name.clone())
                .cookie_secure(true)
                .cookie_http_only(false)
                .build(),
                _ => SessionMiddleware::builder(
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
            })
            .app_data(web::Data::new(app_state.clone()))
            .service(auth::auth_router())
            .service(set_cookie)
            .service(set_session)
            .service(
                web::scope("")
                    .wrap(middlewares::user_auth::AddUser::new())
                    .service(find_user)
                    .route("/", web::get().to(HttpResponse::Ok)),
            )
    })
    .bind((address, PORT))?
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

#[get("/set-cookie")]
async fn set_cookie() -> impl Responder {
    HttpResponse::Ok()
        .cookie(actix_web::cookie::Cookie::build("test", "test").finish())
        .finish()
}

#[get("set-session")]
async fn set_session(req: HttpRequest, session: Session) -> impl Responder {
    if let Some(user) = session.get::<SessionUser>("ekklesia").unwrap() {
        println!("User: {:?}", user);
    } else {
        session
            .insert::<SessionUser>(
                "ekklesia",
                SessionUser {
                    email: String::from("jake.g.mckenney@gmail.com"),
                },
            )
            .unwrap();
    }
    // let session_user = SessionUser {
    //     email: String::from("jake.g.mckenney@gmail.com"),
    // };
    // let serialize_session_user = serde_json::to_string(&session_user).unwrap();
    // Identity::login(&req.extensions(), serialize_session_user).unwrap();
    HttpResponse::Ok().finish()
}
