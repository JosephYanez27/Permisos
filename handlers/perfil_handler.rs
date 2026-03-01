use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use std::collections::HashMap;
use crate::models::perfil::{Perfil, CrearPerfil};

//
// 📌 GET PERFIL (Paginado 5 registros)
//
#[get("/perfil")]
pub async fn get_perfiles(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {

    let page: i64 = query.get("page")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);

    let offset = (page - 1) * 5;

    let perfiles = sqlx::query_as!(
        Perfil,
        r#"
        SELECT id, strnombreperfil, bitadministrador
        FROM perfil
        ORDER BY id
        LIMIT 5 OFFSET $1
        "#,
        offset
    )
    .fetch_all(pool.get_ref())
    .await;

    match perfiles {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener perfiles"),
    }
}

//
// 📌 CREATE PERFIL
//
#[post("/perfil")]
pub async fn create_perfil(
    pool: web::Data<PgPool>,
    data: web::Json<CrearPerfil>,
) -> HttpResponse {

    if data.strnombreperfil.trim().is_empty() {
        return HttpResponse::BadRequest().body("El nombre es obligatorio");
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO perfil (strnombreperfil, bitadministrador)
        VALUES ($1, $2)
        "#,
        data.strnombreperfil,
        data.bitadministrador
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Perfil creado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al crear perfil"),
    }
}

//
// 📌 UPDATE PERFIL
//
#[put("/perfil/{id}")]
pub async fn update_perfil(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    data: web::Json<CrearPerfil>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query!(
        r#"
        UPDATE perfil
        SET strnombreperfil = $1,
            bitadministrador = $2
        WHERE id = $3
        "#,
        data.strnombreperfil,
        data.bitadministrador,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().body("Perfil actualizado"),
        Ok(_) => HttpResponse::NotFound().body("Perfil no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al actualizar"),
    }
}

//
// 📌 DELETE PERFIL
//
#[delete("/perfil/{id}")]
pub async fn delete_perfil(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query!(
        "DELETE FROM perfil WHERE id = $1",
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().body("Perfil eliminado"),
        Ok(_) => HttpResponse::NotFound().body("Perfil no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar"),
    }
}
//
// 📌 DETALLE PERFIL
//
#[get("/perfil/{id}")]
pub async fn get_perfil_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let perfil = sqlx::query_as!(
        Perfil,
        r#"
        SELECT id, strnombreperfil, bitadministrador
        FROM perfil
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match perfil {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => HttpResponse::NotFound().body("Perfil no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener perfil"),
    }
}