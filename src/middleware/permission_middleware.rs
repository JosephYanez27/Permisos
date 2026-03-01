use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,HttpMessage
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::task::{Context, Poll};
use sqlx::PgPool;
use crate::utils::jwt::Claims;

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
    type InitError = ();
    type Transform = PermissionMiddlewareService<S>;
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
        let service = Rc::clone(&self.service);
        let pool = self.pool.clone();

        Box::pin(async move {

            // 🔹 Obtener claims del JWT
            let claims = req.extensions().get::<Claims>().cloned();

            if claims.is_none() {
                return Err(actix_web::error::ErrorUnauthorized("No autorizado"));
            }

            let claims = claims.unwrap();
            let id_perfil = claims.id_perfil;

            // 🔹 Obtener nombre del módulo desde la ruta
            let path = req.path(); // ej: /api/perfil
            let segments: Vec<&str> = path.split('/').collect();

            if segments.len() < 3 {
                return Err(actix_web::error::ErrorUnauthorized("Ruta inválida"));
            }

            let nombre_modulo = segments[2]; // perfil, usuario, modulo...

            // 🔹 Obtener método HTTP
            let method = req.method().as_str();

            // 🔹 Consultar permisos en BD
            let permiso = sqlx::query!(
                r#"
                SELECT p.bitagregar, p.biteditar, p.bitconsulta,
                       p.biteliminar, p.bitdetalle
                FROM permisosperfil p
                JOIN modulo m ON m.id = p.idmodulo
                WHERE p.idperfil = $1
                AND LOWER(m.strnombremodulo) = LOWER($2)
                "#,
                id_perfil,
                nombre_modulo
            )
            .fetch_optional(&pool)
            .await
            .unwrap();

            if permiso.is_none() {
                return Err(actix_web::error::ErrorUnauthorized("Sin permisos"));
            }

            let p = permiso.unwrap();

            // 🔥 Validar según método
            let autorizado = match method {
                "POST" => p.bitagregar,
                "PUT" => p.biteditar,
                "DELETE" => p.biteliminar,
                "GET" => {
                    if path.split('/').count() > 3 {
                        p.bitdetalle
                    } else {
                        p.bitconsulta
                    }
                }
                _ => false,
            };

            if !autorizado {
                return Err(actix_web::error::ErrorUnauthorized("Acción no permitida"));
            }

            let res = service.call(req).await?;
            Ok(res)
        })
    }
}