use actix_web::{get, web, HttpRequest, HttpResponse, HttpMessage};
use sqlx::PgPool;
use crate::utils::jwt::Claims;
use crate::models::permisos::PermisoModulo;

#[get("/mis-permisos")]
pub async fn mis_permisos(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {

    // 🔹 Obtener claims del JWT
    let claims = req.extensions().get::<Claims>().cloned();

    if claims.is_none() {
        return HttpResponse::Unauthorized().body("No autorizado");
    }

    let id_perfil = claims.unwrap().id_perfil;

    // 🔥 AQUÍ ESTÁ EL CAMBIO (sin !)
    let permisos = sqlx::query_as::<_, PermisoModulo>(
        r#"
       SELECT DISTINCT 
    m.id,
    m.strnombremodulo
FROM modulo m
LEFT JOIN modulo h ON h.idpadre = m.id
LEFT JOIN permisosperfil p1 
    ON p1.idmodulo = m.id AND p1.idperfil = $1
LEFT JOIN permisosperfil p2 
    ON p2.idmodulo = h.id AND p2.idperfil = $1
WHERE
    (
        p1.bitconsulta = true
        OR p2.bitconsulta = true
    )
ORDER BY m.id
        "#
    )
    .bind(id_perfil)
    .fetch_all(pool.get_ref())
    .await;

    match permisos {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener permisos"),
    }
}