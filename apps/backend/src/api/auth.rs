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

    use crate::{lib::auth::AuthType, utils::state::AppState};
    #[get("/google")]
    async fn get_google_auth_link(data: web::Data<AppState>) -> HttpResponse {
        let url = data.auth_client.google_get_sign_in_link(AuthType::SignIn);
        return HttpResponse::Ok().json(url);
    }

    #[get("/microsoft")]
    async fn get_microsoft_auth_link(data: web::Data<AppState>) -> HttpResponse {
        let url = data
            .auth_client
            .microsoft_get_sign_in_link(AuthType::SignUp);
        return HttpResponse::Ok().json(url);
    }
}

mod auth_callbacks {
    use std::str::FromStr;

    use actix_web::{
        get,
        http::header::LOCATION,
        web::{self, Redirect},
        HttpResponse, Responder,
    };
    use db::entities::user;

    use crate::{lib::auth::AuthType, utils::state::AppState};

    #[derive(serde::Deserialize, Debug)]
    struct GoogleCallbackResponse {
        code: String,
        state: String,
    }

    #[get("/google")]
    async fn google_callback(
        query: web::Query<GoogleCallbackResponse>,
        app: web::Data<AppState>,
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
        match AuthType::from_str(&query.state) {
            Ok(AuthType::SignIn) => {
                let user = sign_in(&app.db, profile.email.as_str()).await;
            }
            Ok(AuthType::SignUp) => {
                let user = sign_up(&app.db, profile.email.as_str(), profile.name.as_str()).await;
                println!("{:?}", user);
            }
            _ => {
                // TODO: reroute to auth with error displayed
                return HttpResponse::BadRequest().body("Invalid state");
            }
        }

        return HttpResponse::Found()
            .append_header((LOCATION, "http://localhost:3000/home"))
            .finish();
    }
    #[derive(serde::Deserialize, Debug)]
    struct MicrosoftCallbackResponse {
        code: String,
    }

    #[get("/microsoft")]
    pub async fn microsoft_callback(
        query: web::Query<MicrosoftCallbackResponse>,
        app: web::Data<AppState>,
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
        let user = sign_in(&app.db, email.as_str()).await;
        println!("{:?}", user);
        // TODO: sign in and redirect to link where cookies are set
        return Redirect::to("http://localhost:3000/home");
    }

    async fn sign_in(db: &db::DB, email: &str) -> Result<user::Model, String> {
        let lowercase_email = email.to_lowercase();
        let user = db.get_user(&lowercase_email).await.unwrap();
        match user {
            Some(user) => Ok(user),
            None => Err("User not found".to_string()),
        }
    }

    async fn sign_up(db: &db::DB, email: &str, name: &str) -> Result<user::Model, String> {
        let lowercase_email = email.to_lowercase();
        let user = db.get_user(&lowercase_email).await.unwrap();
        if user.is_none() {
            db.insert_user(name, &lowercase_email).await.unwrap();
        }
        return sign_in(db, &lowercase_email).await;
    }
}
