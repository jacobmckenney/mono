use actix_service::{Service, Transform};
use actix_web::{
    body::EitherBody,
    dev::{Payload, ServiceRequest, ServiceResponse},
    Error, FromRequest, HttpRequest, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_identity::Identity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub email: String,
}

impl FromRequest for SessionUser {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let identity_future = Identity::from_request(req, payload);
        let fut = async move {
            if let Ok(identity) = identity_future.await {
                if let Ok(id) = identity.id() {
                    match serde_json::from_str::<SessionUser>(&id) {
                        Ok(user) => return Ok(user),
                        Err(_) => {
                            return Err(actix_web::error::ErrorUnauthorized("Invalid user data"))
                        }
                    }
                }
            }
            Err(actix_web::error::ErrorUnauthorized("No user identity"))
        };
        Box::pin(fut)
    }
}

pub struct AddUser;

impl AddUser {
    pub fn new() -> Self {
        AddUser {}
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
        })
    }
}

pub struct AddUserMiddleware<S> {
    service: Rc<S>,
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
        let service = Rc::clone(&self.service);

        let fut = async move {
            // Check if the identity exists
            let (http_req, mut payload) = req.into_parts();
            // TODO: handle errors here
            if let Ok(_) = SessionUser::from_request(&http_req, &mut payload).await {
                let res = service
                    .call(ServiceRequest::from_parts(http_req, payload))
                    .await?
                    .map_into_left_body();
                return Ok(res);
            }

            // Return 401 Unauthorized if identity does not exist or user is not found
            let response = ServiceResponse::new(
                http_req,
                HttpResponse::Unauthorized().finish().map_into_right_body(),
            );
            return Ok(response);
        };

        Box::pin(fut)
    }
}
