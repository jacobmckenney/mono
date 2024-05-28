use actix_service::{Service, Transform};
use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{ServiceRequest, ServiceResponse},
    http::header::{
        ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
        LOCATION,
    },
    Error, HttpMessage, HttpResponse,
};
use db::entities::user;
use futures::future::{ok, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::{lib, utils::state};

pub struct AddUser {
    state: state::AppState,
}

impl AddUser {
    pub fn new(state: state::AppState) -> Self {
        AddUser { state }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AddUser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static, // Add this constraint
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AddUserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AddUserMiddleware {
            service: Rc::new(service),
            state: self.state.clone(),
        })
    }
}

pub struct AddUserMiddleware<S> {
    service: Rc<S>,
    state: state::AppState,
}

impl<S, B> Service<ServiceRequest> for AddUserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static, // Add this constraint
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let state = self.state.clone();
        let service = Rc::clone(&self.service);

        let fut = async move {
            // Right now auth_cookie = email
            // TODO: encode auth cookie and then decode and extract
            // email
            println!("method {:?}", req.method());
            println!("cookies: {:?}", req.cookies());
            if let Ok(cookie) = lib::auth::extract_auth_cookie(&req) {
                println!("cookie: {:?}", cookie);
                if let Ok(user) = state.db.get_user(&cookie).await {
                    println!("user: {:?}", user);
                    req.extensions_mut()
                        .insert::<db::entities::user::Model>(user.unwrap());
                    let res = service.call(req).await?.map_into_left_body();

                    return Ok(res);
                }
            };

            let response = ServiceResponse::new(
                req.into_parts().0,
                HttpResponse::Found()
                    .append_header((LOCATION, "http://localhost:3000/sign-in"))
                    .finish()
                    .map_into_right_body(),
            );
            Ok(response)
        };

        Box::pin(fut)
    }
}
