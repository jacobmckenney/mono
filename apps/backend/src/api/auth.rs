use actix_identity::Identity;
use actix_web::{
    cookie::Cookie,
    http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, LOCATION},
    post,
    web::{self, Data},
    HttpMessage, HttpRequest, HttpResponse, Responder, Scope,
};
use auth_callbacks::sign_in;

use crate::{api::middlewares::user_auth::SessionUser, library::state::AppState};

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
    if (request.method() == "OPTIONS") {
        return HttpResponse::Ok()
            // .append_header((ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:3000"))
            .finish();
    }
    // let user = sign_in(&app.db, &info.email).await;
    // if user.is_none() {
    //     return HttpResponse::Unauthorized().body("User not found");
    // }

    let serialized_user = serde_json::to_string::<SessionUser>(&SessionUser {
        email: String::from("jake.g.mckenney@gmail.com"),
    })
    .unwrap();

    Identity::login(&request.extensions(), serialized_user).unwrap();
    // Identity::login(&request.extensions(), serialized_user).unwrap();
    // TODO: login
    return HttpResponse::Found()
        .append_header((ACCESS_CONTROL_ALLOW_ORIGIN, "localhost"))
        .append_header((LOCATION, "http://localhost:3000/app"))
        .finish();
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

    use actix_identity::Identity;
    use actix_web::{
        get,
        http::header::LOCATION,
        web::{Data, Query},
        HttpMessage, HttpRequest, HttpResponse, Responder,
    };
    use db::entities::user;

    use crate::{api::middlewares::user_auth::SessionUser, library::state::AppState};

    #[derive(serde::Deserialize, Debug)]
    struct GoogleCallbackResponse {
        code: String,
    }

    #[get("/google")]
    async fn google_callback(
        query: Query<GoogleCallbackResponse>,
        app: Data<AppState>,
        request: HttpRequest,
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
        sign_in_or_sign_up(
            request,
            &app.db,
            &profile.email,
            &profile.name,
            Some(&profile.picture),
        )
        .await;
        return HttpResponse::Found()
            .append_header((LOCATION, "http://localhost:3000/app"))
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
        request: HttpRequest,
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
        sign_in_or_sign_up(request, &app.db, &email, &name, None).await;

        return HttpResponse::PermanentRedirect()
            .append_header((LOCATION, "http://localhost:3000/app"))
            .finish();
    }

    pub async fn sign_in(db: &db::DB, email: &str) -> Option<user::Model> {
        let lowercase_email = email.to_lowercase();
        let user = db.get_user(&lowercase_email).await.unwrap();
        return user;
    }

    async fn sign_up(
        db: &db::DB,
        email: &str,
        name: &str,
        image: Option<&str>,
    ) -> Result<user::Model, String> {
        let lowercase_email = email.to_lowercase();
        let mut user = db.get_user(&lowercase_email).await.unwrap();
        if user.is_none() {
            db.insert_user(name, &lowercase_email, image).await.unwrap();
        }
        user = db.get_user(&lowercase_email).await.unwrap();
        return Ok(user.unwrap());
    }

    async fn sign_in_or_sign_up(
        request: HttpRequest,
        db: &db::DB,
        email: &str,
        name: &str,
        image: Option<&str>,
    ) -> user::Model {
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
        return user;
    }
}
