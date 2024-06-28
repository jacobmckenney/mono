use actix_identity::Identity;
use actix_web::{
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder, Scope,
};
use auth_callbacks::sign_in_or_sign_up;

use crate::library::state::AppState;

pub fn auth_router() -> Scope {
    web::scope("/auth")
        .service(logout)
        .service(email_login)
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

#[post("/logout")]
pub async fn logout(user: Identity) -> impl Responder {
    Identity::logout(user);
    return HttpResponse::Ok().finish();
}

#[derive(serde::Deserialize, serde::Serialize)]
struct EmailEndpointBody {
    email: String,
}

#[post("email")]
pub async fn email_login(
    info: web::Json<EmailEndpointBody>,
    app: Data<AppState>,
    request: HttpRequest,
) -> impl Responder {
    let user = sign_in_or_sign_up(request, &app.db, &info.email, None, None).await;
    match user {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::Unauthorized().finish(),
    }
}

mod auth_links {
    use actix_web::{get, web, HttpResponse};

    use crate::library::state::AppState;
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

    use std::collections::HashMap;

    use actix_identity::Identity;
    use actix_web::{
        get,
        http::header::LOCATION,
        web::{Data, Query},
        HttpMessage, HttpRequest, HttpResponse, Responder,
    };
    use db::{
        data::user::{get_user, insert_user, InsertUserData},
        entities::user,
    };
    use sea_orm::DatabaseConnection;

    use crate::{
        api::middlewares::user_auth::SessionUser,
        library::{env::get_base_url, state::AppState},
    };

    #[derive(serde::Deserialize, Debug)]
    struct GoogleCallbackResponse {
        code: String,
    }

    #[get("/google")]
    async fn google_callback(app: Data<AppState>, request: HttpRequest) -> impl Responder {
        let query = Query::<HashMap<String, String>>::from_query(request.query_string()).unwrap();
        let code = query.get("code");
        if code.is_none() {
            return HttpResponse::Found()
                .append_header(
                    // TODO: make this env agnostic, and incorporate sign-in type,
                    // as well as down below
                    (LOCATION, format!("{}/auth/sign-in", get_base_url())),
                )
                .finish();
        }
        // Validate user anwd then get
        let tokens = match app
            .auth_client
            .google_get_tokens(code.unwrap().as_str())
            .await
        {
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
        let user = sign_in_or_sign_up(
            request,
            &app.db,
            &profile.email,
            Some(&profile.name),
            Some(&profile.picture),
        )
        .await;
        return match user {
            Some(_) => HttpResponse::Found()
                .append_header((LOCATION, app.app_auth_redirect_url.as_str()))
                .finish(),
            None => HttpResponse::Found()
                .append_header(
                    // TODO: make this env agnostic, and incorporate sign-in type,
                    // as well as down below
                    (LOCATION, format!("{}/auth/sign-in", get_base_url())),
                )
                .finish(),
        };
    }
    #[derive(serde::Deserialize, Debug)]
    struct MicrosoftCallbackResponse {
        code: String,
    }

    #[get("/microsoft")]
    pub async fn microsoft_callback(app: Data<AppState>, request: HttpRequest) -> impl Responder {
        // Don't use built-in query so we can re-direct
        let query = Query::<HashMap<String, String>>::from_query(request.query_string()).unwrap();
        let code = query.get("code");
        if code.is_none() {
            return HttpResponse::Found()
                .append_header(
                    // TODO: make this env agnostic, and incorporate sign-in type,
                    // as well as down below
                    (LOCATION, format!("{}/auth/sign-in", get_base_url())),
                )
                .finish();
        }
        // TODO: get sign-in or sign-up type via state
        let profile = app
            .auth_client
            .microsoft_get_tokens(code.unwrap().as_str())
            .await
            .unwrap();
        // Use preferred_username for security reasons
        let email = profile.preferred_username;
        let name = profile.name;
        let user = sign_in_or_sign_up(request, &app.db, &email, Some(&name), None).await;

        return match user {
            Some(_) => HttpResponse::Found()
                .append_header((LOCATION, app.app_auth_redirect_url.as_str()))
                .finish(),
            None => HttpResponse::Found()
                .append_header(
                    // TODO: make this env agnostic, and incorporate sign-in type,
                    // as well as down below
                    (LOCATION, format!("{}/auth/sign-in", get_base_url())),
                )
                .finish(),
        };
    }

    pub async fn sign_in(db: &DatabaseConnection, email: &str) -> Option<user::Model> {
        let lowercase_email = email.to_lowercase();
        let user = get_user(db, &lowercase_email).await.unwrap();
        return user.map(|user| user.user);
    }

    async fn sign_up(
        db: &DatabaseConnection,
        email: &str,
        name: Option<&str>,
        image: Option<&str>,
    ) -> Result<user::Model, String> {
        let lowercase_email = email.to_lowercase();
        let mut user = get_user(db, &lowercase_email).await.unwrap();
        if user.is_none() {
            insert_user(
                db,
                InsertUserData {
                    email: String::from(lowercase_email.clone()),
                    name: name.map(|s| s.to_string()),
                    image: image.map(|s| s.to_string()),
                },
            )
            .await
            .unwrap();
        }
        user = get_user(db, &lowercase_email).await.unwrap();
        return Ok(user.unwrap().user);
    }

    pub async fn sign_in_or_sign_up(
        request: HttpRequest,
        db: &DatabaseConnection,
        email: &str,
        name: Option<&str>,
        image: Option<&str>,
    ) -> Option<user::Model> {
        if email.is_empty() {
            return None;
        }
        let user = match sign_in(db, email).await {
            Some(user) => user,
            None => sign_up(db, email, name, image).await.unwrap(),
        };

        // Store user in session
        let session_user = SessionUser {
            email: user.email.clone(),
        };
        let serialized_user = serde_json::to_string::<SessionUser>(&session_user).unwrap();
        Identity::login(&request.extensions(), serialized_user).unwrap();
        return Some(user);
    }
}
