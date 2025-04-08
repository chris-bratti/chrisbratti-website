use lazy_static::lazy_static;
use std::future::{ready, Ready};

lazy_static! {
    static ref API_KEY: String = get_env_variable("API_KEY").expect("API_KEY was not set!");
}

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

use crate::server_functions::get_env_variable;

pub struct VerifyApiKey;

impl<S, B> Transform<S, ServiceRequest> for VerifyApiKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyApiKeyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifyApiKeyMiddleware { service }))
    }
}

pub struct VerifyApiKeyMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for VerifyApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if !req.headers().contains_key("apiKey") {
            println!("No API key supplied for path: {}", req.path());
            return Box::pin(async move {
                Err(actix_web::error::ErrorBadRequest(
                    "Auth credentials not supplied",
                ))
            });
        }

        if req.headers().get("apikey").unwrap() != API_KEY.as_str() {
            println!("Unauthorized attempt for path: {}", req.path());
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized("Authentication failed"))
            });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
