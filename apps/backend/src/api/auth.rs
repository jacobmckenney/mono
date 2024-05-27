use actix_web::{web, Scope};

pub fn auth_router() -> Scope {
    web::scope("/auth")
        .service(
            web::scope("/link")
                .service(auth_links::get_google_auth_link)
                .service(auth_links::get_microsoft_auth_link),
        )
        .service(
            web::scope("/callback")
                .service(auth_callbacks::google_callback)
                .service(auth_callbacks::microsoft_callback),
        )
}

mod auth_links {
    use actix_web::{get, web, HttpResponse};

    use crate::utils::state::AppState;
    #[get("/google")]
    async fn get_google_auth_link(data: web::Data<AppState>) -> HttpResponse {
        let url = data.auth_client.google_get_sign_in_link();
        return HttpResponse::Ok().json(url);
    }

    #[get("/microsoft")]
    async fn get_microsoft_auth_link(data: web::Data<AppState>) -> HttpResponse {
        let url = data.auth_client.microsoft_get_sign_in_link();
        return HttpResponse::Ok().json(url);
    }
}

mod auth_callbacks {

    use actix_web::{
        get,
        http::header::LOCATION,
        web::{self, Data, Query},
        HttpResponse, Responder,
    };
    use db::entities::user;

    use crate::{lib::auth::build_auth_cookie, utils::state::AppState};

    #[derive(serde::Deserialize, Debug)]
    struct GoogleCallbackResponse {
        code: String,
    }

    #[get("/google")]
    async fn google_callback(
        query: Query<GoogleCallbackResponse>,
        app: Data<AppState>,
    ) -> impl Responder {
        // Validate user anwd then get
        let tokens = match app.auth_client.google_get_tokens(query.code.as_str()).await {
            Ok(tokens) => tokens,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to get tokens: {}", e))
            }
        };

        let profile = match app
            .auth_client
            .google_get_profile(tokens.access_token.as_str())
            .await
        {
            Ok(profile) => profile,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to get profile: {}", e))
            }
        };
        if !profile.verified_email {
            return HttpResponse::Unauthorized().body("Email not verified by google");
        }
        let user = sign_in_or_sign_up(&app.db, &profile.email, &profile.name).await;
        let auth_cookie = build_auth_cookie(&user);
        return HttpResponse::Found()
            .append_header((LOCATION, "http://localhost:3000/home"))
            .cookie(auth_cookie)
            .finish();
    }
    #[derive(serde::Deserialize, Debug)]
    struct MicrosoftCallbackResponse {
        code: String,
    }

    #[get("/microsoft")]
    pub async fn microsoft_callback(
        query: Query<MicrosoftCallbackResponse>,
        app: Data<AppState>,
    ) -> impl Responder {
        // TODO: get sign-in or sign-up type via state
        let profile = app
            .auth_client
            .microsoft_get_tokens(query.code.as_str())
            .await
            .unwrap();
        // Use preferred_username for security reasons
        let email = profile.preferred_username;
        let name = profile.name;
        let user = sign_in_or_sign_up(&app.db, &email, &name).await;
        let auth_cookie = build_auth_cookie(&user);
        return HttpResponse::Found()
            .cookie(auth_cookie)
            .append_header((LOCATION, "http://localhost:3000/home"))
            .finish();
    }

    async fn sign_in(db: &db::DB, email: &str) -> Option<user::Model> {
        let lowercase_email = email.to_lowercase();
        let user = db.get_user(&lowercase_email).await.unwrap();
        return user;
    }

    async fn sign_up(db: &db::DB, email: &str, name: &str) -> Result<user::Model, String> {
        let lowercase_email = email.to_lowercase();
        let mut user = db.get_user(&lowercase_email).await.unwrap();
        if user.is_none() {
            db.insert_user(name, &lowercase_email).await.unwrap();
        }
        user = db.get_user(&lowercase_email).await.unwrap();
        return Ok(user.unwrap());
    }

    async fn sign_in_or_sign_up(db: &db::DB, email: &str, name: &str) -> user::Model {
        let user = match sign_in(db, email).await {
            Some(user) => user,
            None => sign_up(db, email, name).await.unwrap(),
        };
        return user;
    }
}
