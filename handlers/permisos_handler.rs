use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use std::collections::HashMap;
use crate::models::permisos::{PermisoPerfil, CrearPermisoPerfil};

//
// 📌 GET PERMISOS (Paginado 5 registros)
// ❗ Sin filtro como pide la evaluación
//
#[get("/permisosperfil")]
pub async fn get_permisos(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {

    let page: i64 = query.get("page")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);

    let offset = (page - 1) * 5;

    let permisos = sqlx::query_as!(
        PermisoPerfil,
        r#"
        SELECT id, idperfil, idmodulo,
               bitagregar, biteditar, bitconsulta,
               biteliminar, bitdetalle
        FROM permisosperfil
        ORDER BY id
        LIMIT 5 OFFSET $1
        "#,
        offset
    )
    .fetch_all(pool.get_ref())
    .await;

    match permisos {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener permisos"),
    }
}

//
// 📌 DETALLE PERMISO
//
#[get("/permisosperfil/{id}")]
pub async fn get_permiso_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let permiso = sqlx::query_as!(
        PermisoPerfil,
        r#"
        SELECT id, idperfil, idmodulo,
               bitagregar, biteditar, bitconsulta,
               biteliminar, bitdetalle
        FROM permisosperfil
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match permiso {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => HttpResponse::NotFound().body("Permiso no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener permiso"),
    }
}

//
// 📌 CREATE PERMISO
//
#[post("/permisosperfil")]
pub async fn create_permiso(
    pool: web::Data<PgPool>,
    data: web::Json<CrearPermisoPerfil>,
) -> HttpResponse {

    let result = sqlx::query!(
        r#"
        INSERT INTO permisosperfil (
            idperfil, idmodulo,
            bitagregar, biteditar, bitconsulta,
            biteliminar, bitdetalle
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        data.idperfil,
        data.idmodulo,
        data.bitagregar,
        data.biteditar,
        data.bitconsulta,
        data.biteliminar,
        data.bitdetalle
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Permiso creado correctamente"),
        Err(_) => HttpResponse::InternalServerError().body("Error al crear permiso"),
    }
}

//
// 📌 UPDATE PERMISO
//
#[put("/permisosperfil/{id}")]
pub async fn update_permiso(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    data: web::Json<CrearPermisoPerfil>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query!(
        r#"
        UPDATE permisosperfil
        SET idperfil = $1,
            idmodulo = $2,
            bitagregar = $3,
            biteditar = $4,
            bitconsulta = $5,
            biteliminar = $6,
            bitdetalle = $7
        WHERE id = $8
        "#,
        data.idperfil,
        data.idmodulo,
        data.bitagregar,
        data.biteditar,
        data.bitconsulta,
        data.biteliminar,
        data.bitdetalle,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            HttpResponse::Ok().body("Permiso actualizado")
        }
        Ok(_) => HttpResponse::NotFound().body("Permiso no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al actualizar permiso"),
    }
}

//
// 📌 DELETE PERMISO
//
#[delete("/permisosperfil/{id}")]
pub async fn delete_permiso(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query!(
        "DELETE FROM permisosperfil WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            HttpResponse::Ok().body("Permiso eliminado")
        }
        Ok(_) => HttpResponse::NotFound().body("Permiso no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar permiso"),
    }
}