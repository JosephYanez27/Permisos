use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::task::{Context, Poll};
use crate::utils::jwt::{validate_jwt, Claims};

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {

            // 1️⃣ Obtener header Authorization
            let auth_header = req.headers().get("Authorization");

            if auth_header.is_none() {
                return Err(actix_web::error::ErrorUnauthorized("Token requerido"));
            }

            let auth_str = auth_header.unwrap().to_str().unwrap_or("");

            // 2️⃣ Validar formato Bearer
            if !auth_str.starts_with("Bearer ") {
                return Err(actix_web::error::ErrorUnauthorized("Formato inválido"));
            }

            let token = &auth_str[7..];

            // 3️⃣ Validar JWT
            let claims = match validate_jwt(token) {
                Ok(data) => data,
                Err(_) => {
                    return Err(actix_web::error::ErrorUnauthorized("Token inválido"));
                }
            };

            // 4️⃣ Insertar claims en request
            req.extensions_mut().insert::<Claims>(claims);

            // 5️⃣ Continuar
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}