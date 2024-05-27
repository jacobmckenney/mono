use db::DB;

use crate::lib::auth;
use auth::{AuthClient, OAuthConfig};

#[derive(Clone)]
pub struct AppState {
    pub auth_client: AuthClient,
    pub db: DB,
}

pub async fn create_app_state() -> AppState {
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
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DB::new(db_url).await.unwrap();

    let auth_client = AuthClient { google, microsoft };
    AppState { auth_client, db }
}
