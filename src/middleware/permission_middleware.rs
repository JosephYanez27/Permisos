use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use sqlx::{PgPool, Row};
use actix_web::error::ErrorUnauthorized;
use std::task::{Context, Poll};
use std::rc::Rc;

pub struct PermissionMiddleware {
    pub pool: PgPool,
}

impl<S, B> Transform<S, ServiceRequest> for PermissionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PermissionMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(PermissionMiddlewareService {
            service: Rc::new(service),
            pool: self.pool.clone(),
        })
    }
}

pub struct PermissionMiddlewareService<S> {
    service: Rc<S>,
    pool: PgPool,
}

impl<S, B> Service<ServiceRequest> for PermissionMiddlewareService<S>
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

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let srv = self.service.clone();
        let pool = self.pool.clone();

        Box::pin(async move {

            let id_perfil = match req.extensions().get::<i32>() {
                Some(p) => *p,
                None => return Err(ErrorUnauthorized("No perfil")),
            };

            let path = req.path().replace("/api/", "");
            let nombre_modulo = path.split('/').next().unwrap_or("");

            let permiso = sqlx::query(
                r#"
                SELECT bitagregar, biteditar, biteliminar,
                       bitdetalle, bitconsulta
                FROM permisosperfil pp
                JOIN modulo m ON m.id = pp.idmodulo
                WHERE pp.idperfil = $1
                AND m.strnombremodulo = $2
                "#
            )
            .bind(id_perfil)
            .bind(nombre_modulo)
            .fetch_optional(&pool)
            .await
            .unwrap();

            if permiso.is_none() {
                return Err(ErrorUnauthorized("Sin permisos"));
            }

            let p = permiso.unwrap();

            let bitagregar: bool = p.get("bitagregar");
            let biteditar: bool = p.get("biteditar");
            let biteliminar: bool = p.get("biteliminar");
            let bitdetalle: bool = p.get("bitdetalle");
            let bitconsulta: bool = p.get("bitconsulta");

            let metodo = req.method().as_str();

            let autorizado = match metodo {
                "POST" => bitagregar,
                "PUT" => biteditar,
                "DELETE" => biteliminar,
                "GET" => bitconsulta || bitdetalle,
                _ => false,
            };

            if !autorizado {
                return Err(ErrorUnauthorized("Acceso denegado"));
            }

            srv.call(req).await
        })
    }
}