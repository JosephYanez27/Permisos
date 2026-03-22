use actix_web::{get, web,post, put, delete, HttpRequest, HttpResponse, HttpMessage};
use sqlx::PgPool;
use crate::models::modulo::{CrearModulo, Modulo};
use crate::utils::jwt::Claims;
use serde_json::json;

#[get("/menu")]
pub async fn get_menu(
    req: HttpRequest,
    pool: web::Data<PgPool>
) -> HttpResponse {

    // 🔹 Obtener perfil del JWT
let claims = match req.extensions().get::<Claims>().cloned() {
    Some(c) => c,
    None => return HttpResponse::Unauthorized().body("No autorizado"),
};

let id_perfil = claims.id_perfil;

    // 🔹 Obtener módulos con permisos
let result = sqlx::query_as::<_, Modulo>(
    r#"
    SELECT DISTINCT
        m.id,
        m.strnombremodulo,
        m.idmodulopadre
    FROM modulo m
    WHERE
        m.id IN (
            SELECT p.idmodulo
            FROM permisosperfil p
            WHERE p.idperfil = $1
            AND (
                p.bitconsulta = true OR
                p.bitdetalle = true OR
                p.bitagregar = true OR
                p.biteditar = true OR
                p.biteliminar = true
            )
        )

        OR m.id IN (
            SELECT padre.id
            FROM modulo hijo
            JOIN modulo padre ON padre.id = hijo.idmodulopadre
            JOIN permisosperfil p ON p.idmodulo = hijo.id
            WHERE p.idperfil = $1
            AND (
                p.bitconsulta = true OR
                p.bitdetalle = true OR
                p.bitagregar = true OR
                p.biteditar = true OR
                p.biteliminar = true
            )
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

#[get("/modulo")]
pub async fn get_modulos(pool: web::Data<PgPool>) -> HttpResponse {

    let result = sqlx::query_as::<_, Modulo>(
        r#"
        SELECT 
            m.id,
            m.strnombremodulo,
            m.idmodulopadre
        FROM modulo m
        WHERE NOT EXISTS (
            SELECT 1 
            FROM modulo hijo
            WHERE hijo.idmodulopadre = m.id
        )
        ORDER BY m.id
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener módulos"),
    }
}
#[post("/modulo")]
pub async fn create_modulo(
    pool: web::Data<PgPool>,
    data: web::Json<CrearModulo>,
) -> HttpResponse {

    let result = sqlx::query(
        r#"
        INSERT INTO modulo (strnombremodulo, idmodulopadre)
        VALUES ($1, $2)
        "#
    )
    .bind(&data.strnombremodulo)
    .bind(data.idmodulopadre)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Módulo creado"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error al crear módulo")
        }
    }
}
#[put("/modulo/{id}")]
pub async fn update_modulo(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    data: web::Json<CrearModulo>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE modulo
        SET strnombremodulo = $1,
            idmodulopadre = $2
        WHERE id = $3
        "#
    )
    .bind(&data.strnombremodulo)
    .bind(data.idmodulopadre)
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Módulo actualizado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al actualizar"),
    }
}
#[delete("/modulo/{id}")]
pub async fn delete_modulo(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM modulo WHERE id = $1"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Módulo eliminado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar"),
    }
}