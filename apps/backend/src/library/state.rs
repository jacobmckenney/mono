use super::auth::{AuthClient, OAuthConfig};
use actix_web::cookie::Key;
use db::DB;

#[derive(Clone)]
pub struct AppState {
    pub auth_client: AuthClient,
    pub db: DB,
    pub environment: String,
    pub app_name: String,
    pub encryption_key: Key,
    pub app_auth_redirect_url: String,
}

pub async fn create_app_state() -> AppState {
    let environment = std::env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");
    let app_auth_redirect_url =
        std::env::var("APP_AUTH_REDIRECT_URL").expect("APP_AUTH_REDIRECT_URL must be set");

    // Session cookie encryption
    let key_seed = std::env::var("ENCRYPTION_KEY").expect("Encryption key must be set");
    let key = Key::derive_from(key_seed.as_bytes());

    // OAuth
    let google = OAuthConfig {
        client_id: std::env::var("GOOGLE_OAUTH_CLIENT_ID")
            .expect("GOOGLE_OAUTH_CLIENT_ID must be set"),
        client_secret: std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("GOOGLE_OAUTH_CLIENT_SECRET must be set"),
        redirect_uri: std::env::var("GOOGLE_OAUTH_REDIRECT_URI")
            .expect("GOOGLE_OAUTH_REDIRECT_URI must be set"),
    };

    let microsoft = OAuthConfig {
        client_id: std::env::var("MICROSOFT_OAUTH_CLIENT_ID")
            .expect("MICROSOFT_OAUTH_CLIENT_ID must be set"),
        client_secret: std::env::var("MICROSOFT_OAUTH_CLIENT_SECRET")
            .expect("MICROSOFT_OAUTH_CLIENT_SECRET must be set"),
        redirect_uri: std::env::var("MICROSOFT_OAUTH_REDIRECT_URI")
            .expect("MICROSOFT_OAUTH_REDIRECT_URI must be set"),
    };
    // Db connection
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DB::new(db_url).await.unwrap();

    let auth_client = AuthClient { google, microsoft };
    AppState {
        auth_client,
        db,
        environment: environment.clone(),
        app_name: String::from("ekklesia"),
        app_auth_redirect_url,
        encryption_key: key,
    }
}
