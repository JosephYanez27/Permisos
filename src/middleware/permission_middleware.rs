use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use sqlx::{PgPool, Row};
use actix_web::error::ErrorUnauthorized;
use std::task::{Context, Poll};
use std::rc::Rc;
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
        // --- PASO 1: Extraer datos y liberar el préstamo ---
        let id_perfil = {
            let extensions = req.extensions();
            let claims = extensions.get::<Claims>()
                .ok_or_else(|| ErrorUnauthorized("No perfil"))?;
            
            // Extraemos solo el valor que necesitamos (asumiendo que id_perfil es Copy o Clone)
            claims.id_perfil 
        }; // <--- Aquí 'extensions' sale de ámbito y el préstamo de 'req' termina.

        // --- PASO 2: Lógica de base de datos ---
        let path = req.path().replace("/api/", "");
        let nombre_modulo = path.split('/').next().unwrap_or("");

        let row = sqlx::query(
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
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?; // Manejo de error de DB

        let p = row.ok_or_else(|| ErrorUnauthorized("Sin permisos"))?;

        // --- PASO 3: Validación de métodos ---
        let autorizado = match *req.method() {
            actix_web::http::Method::POST => p.get::<bool, _>("bitagregar"),
            actix_web::http::Method::PUT => p.get::<bool, _>("biteditar"),
            actix_web::http::Method::DELETE => p.get::<bool, _>("biteliminar"),
            actix_web::http::Method::GET => p.get::<bool, _>("bitconsulta") || p.get::<bool, _>("bitdetalle"),
            _ => false,
        };

        if !autorizado {
            return Err(ErrorUnauthorized("Acceso denegado"));
        }

        // --- PASO 4: Mover 'req' al siguiente servicio ---
        // Ahora req está libre de préstamos y puede moverse con éxito
        srv.call(req).await
    })
}
}