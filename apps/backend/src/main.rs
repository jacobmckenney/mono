mod api;
mod library;

use actix_identity::IdentityMiddleware;
use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use api::{
    auth,
    middlewares::{self, user_auth::SessionUser},
};
use library::{
    cors, session,
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
            .wrap(session::configure_session(
                environment.clone().as_str(),
                app_state.app_name.clone().as_str(),
                &app_state.encryption_key,
            ))
            .app_data(web::Data::new(app_state.clone()))
            .service(auth::auth_router())
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
