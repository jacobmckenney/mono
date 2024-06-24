use std::env;

pub fn get_base_url() -> String {
    let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");
    return match environment.as_str() {
        "local" => "http://localhost:3000",
        "production" => "app.ekklesia.dev",
        _ => panic!("Invalid environment"),
    }
    .to_string();
}
