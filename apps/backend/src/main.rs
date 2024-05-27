mod api;
mod lib;
mod utils;

use actix_web::{web, App, HttpResponse, HttpServer};
use api::auth;
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
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", PORT))?
    .workers(NUM_WORKERS)
    .run()
    .await;
    Ok(())
}
