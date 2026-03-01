use actix_web::{get, web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use crate::utils::jwt::Claims;
use crate::models::permisos::PermisoModulo;

#[get("/mis-permisos")]
pub async fn mis_permisos(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {

    // 🔹 Obtener claims del JWT (insertado por middleware)
    let claims = req.extensions().get::<Claims>().cloned();

    if claims.is_none() {
        return HttpResponse::Unauthorized().body("No autorizado");
    }

    let id_perfil = claims.unwrap().id_perfil;

    // 🔹 Traer solo módulos con al menos un permiso activo
    let permisos = sqlx::query_as!(
        PermisoModulo,
        r#"
        SELECT 
            m.strnombremodulo as modulo,
            p.bitagregar,
            p.biteditar,
            p.bitconsulta,
            p.biteliminar,
            p.bitdetalle
        FROM permisosperfil p
        JOIN modulo m ON m.id = p.idmodulo
        WHERE p.idperfil = $1
        AND (
            p.bitagregar = true OR
            p.biteditar = true OR
            p.bitconsulta = true OR
            p.biteliminar = true OR
            p.bitdetalle = true
        )
        ORDER BY m.strnombremodulo
        "#,
        id_perfil
    )
    .fetch_all(pool.get_ref())
    .await;

    match permisos {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener permisos"),
    }
}