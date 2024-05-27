use actix_cors::Cors;

pub fn configure_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allowed_methods(vec!["GET", "POST"])
}
