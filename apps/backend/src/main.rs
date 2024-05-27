mod api;
mod lib;
mod utils;

use actix_web::{
    get, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use api::{auth, middlewares};
use db::entities::user;
use lib::auth::get_user;
use utils::{cors, state};

const NUM_WORKERS: usize = 4;
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Starting server at: http://localhost:{}", PORT);

    let app_state = state::create_app_state().await;
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors::configure_cors())
            .service(auth::auth_router())
            .wrap(middlewares::user_auth::AddUser::new(app_state.clone()))
            .service(test)
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", PORT))?
    .workers(NUM_WORKERS)
    .run()
    .await;
    Ok(())
}

#[get("/bruh")]
async fn test(req: HttpRequest) -> Result<HttpResponse> {
    let user = get_user(req).unwrap();
    println!("{:?}", user);
    return Ok(HttpResponse::Ok().body("bruh"));
}
