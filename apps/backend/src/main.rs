mod api;
mod lib;
mod utils;

use actix_web::{
    get,
    http::StatusCode,
    web::{self, Data},
    App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use api::{auth, middlewares};
use lib::auth::{get_user, AuthCookieExtractor};
use utils::{
    cors,
    state::{self, AppState},
};

const NUM_WORKERS: usize = 4;
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Starting server at: http://localhost:{}", PORT);

    let app_state = state::create_app_state().await;
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(cors::configure_cors())
            .app_data(web::Data::new(app_state.clone()))
            .service(auth::auth_router())
            .service(find_user)
            .service(
                web::scope("")
                    .wrap(middlewares::user_auth::AddUser::new(app_state.clone()))
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
async fn find_user(req: HttpRequest, app: Data<AppState>) -> Result<impl Responder> {
    let email = req.extract_auth_cookie().unwrap();
    let user = app.db.get_user(&email).await.unwrap();
    return Ok(web::Json(user));
}
