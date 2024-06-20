use actix_session::{
    config::CookieContentSecurity, storage::CookieSessionStore, SessionMiddleware,
};
use actix_web::cookie::{Key, SameSite};

pub fn configure_session(
    environment: &str,
    cookie_name: &str,
    encryption_key: &Key,
) -> SessionMiddleware<CookieSessionStore> {
    return match environment {
        "production" => {
            SessionMiddleware::builder(CookieSessionStore::default(), encryption_key.clone())
                .cookie_domain(Some(String::from("app.ekklesia.dev")))
                .cookie_path(String::from("/"))
                .cookie_name(String::from(cookie_name))
                .cookie_secure(true)
                .cookie_content_security(CookieContentSecurity::Private)
                .cookie_same_site(SameSite::None)
                .build()
        }
        _ => SessionMiddleware::builder(CookieSessionStore::default(), encryption_key.clone())
            .cookie_domain(Some(String::from("localhost")))
            .cookie_path(String::from("/"))
            .cookie_name(String::from(cookie_name))
            .cookie_secure(true)
            .cookie_content_security(CookieContentSecurity::Private)
            // TODO: configure same site properly depending on the environment
            .cookie_same_site(SameSite::None)
            .build(),
    };
}
