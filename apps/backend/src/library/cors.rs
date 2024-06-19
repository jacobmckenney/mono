use std::vec;

use actix_cors::Cors;
use actix_web::http::header::CONTENT_TYPE;

pub fn configure_cors(environment: &str) -> Cors {
    return match environment {
        "production" => Cors::default()
            .allowed_origin("https://app.ekklesia.dev")
            .allowed_headers(vec![CONTENT_TYPE])
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .supports_credentials()
            .max_age(3600),
        _ => Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_headers(vec![CONTENT_TYPE])
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .supports_credentials()
            .max_age(3600),
    };
}
