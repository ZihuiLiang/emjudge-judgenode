use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::{get, web, HttpResponse};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::settings::Settings;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        if let Some(auth) = req.headers().get("Authorization") {
            if let Ok(auth) = auth.to_str() {
                if auth
                    == format!(
                        "Bearer {}",
                        req.app_data::<web::Data<Settings>>().unwrap().license
                    )
                {
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    });
                }
            }
        }
        Box::pin(async move { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) })
    }
}
#[get("/is_authed")]
pub async fn is_authed() -> HttpResponse {
    HttpResponse::Ok().body("Authorized")
}
