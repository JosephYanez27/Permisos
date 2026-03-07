use actix_web::{get, web, HttpRequest, HttpResponse, HttpMessage};
use sqlx::PgPool;
use crate::models::modulo::Modulo;
use crate::utils::jwt::Claims;
use serde_json::json;

#[get("/menu")]
pub async fn get_menu(
    req: HttpRequest,
    pool: web::Data<PgPool>
) -> HttpResponse {

    // 🔹 Obtener perfil del JWT
    let claims = match req.extensions().get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("No autorizado"),
    };

    let id_perfil = claims.id_perfil;

    // 🔹 Obtener módulos con permisos
    let result = sqlx::query_as::<_, Modulo>(
        r#"
        SELECT m.id, m.strnombremodulo, m.idmodulopadre
        FROM permisosperfil p
        JOIN modulo m ON m.id = p.idmodulo
        WHERE p.idperfil = $1
        AND (
            p.bitconsulta = true OR
            p.bitdetalle = true OR
            p.bitagregar = true OR
            p.biteditar = true OR
            p.biteliminar = true
        )
        ORDER BY m.idmodulopadre NULLS FIRST, m.id
        "#
    )
    .bind(id_perfil)
    .fetch_all(pool.get_ref())
    .await;

    let modulos: Vec<Modulo> = match result {
        Ok(m) => m,
        Err(e) => {
            println!("Error cargando menú: {:?}", e);
            return HttpResponse::InternalServerError().body("Error cargando menú");
        }
    };

    // 🔹 Construir menú jerárquico
    let mut menu = Vec::new();

    for padre in modulos.iter().filter(|m| m.idmodulopadre.is_none()) {

        let hijos: Vec<_> = modulos
            .iter()
            .filter(|m| m.idmodulopadre == Some(padre.id))
            .map(|h| {
                json!({
                    "id": h.id,
                    "nombre": h.strnombremodulo
                })
            })
            .collect();

        // 🔥 Solo mostrar padres si tienen hijos
        if !hijos.is_empty() {
            menu.push(json!({
                "id": padre.id,
                "nombre": padre.strnombremodulo,
                "hijos": hijos
            }));
        }
    }

    HttpResponse::Ok().json(menu)
}