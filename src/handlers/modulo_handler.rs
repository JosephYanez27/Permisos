use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::{PgPool};
use std::collections::HashMap;
use crate::models::modulo::{Modulo, CrearModulo};

//
// 📌 GET MODULOS (Paginado 5 registros)
//
#[get("/modulo")]
pub async fn get_modulos(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {

    let page: i64 = query.get("page")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);

    let offset = (page - 1) * 5;

    let modulos = sqlx::query_as::<_, Modulo>(
        r#"
        SELECT id, strnombremodulo
        FROM modulo
        ORDER BY id
        LIMIT 5 OFFSET $1
        "#
    )
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match modulos {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener módulos")
        }
    }
}

//
// 📌 DETALLE MODULO
//
#[get("/modulo/{id}")]
pub async fn get_modulo_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let modulo = sqlx::query_as::<_, Modulo>(
        r#"
        SELECT id, strnombremodulo
        FROM modulo
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await;

    match modulo {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => HttpResponse::NotFound().body("Módulo no encontrado"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener módulo")
        }
    }
}

//
// 📌 CREATE MODULO
//
#[post("/modulo")]
pub async fn create_modulo(
    pool: web::Data<PgPool>,
    data: web::Json<CrearModulo>,
) -> HttpResponse {

    if data.strnombremodulo.trim().is_empty() {
        return HttpResponse::BadRequest().body("El nombre del módulo es obligatorio");
    }

    let result = sqlx::query(
        r#"
        INSERT INTO modulo (strnombremodulo)
        VALUES ($1)
        "#
    )
    .bind(&data.strnombremodulo)
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

//
// 📌 UPDATE MODULO
//
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
        SET strnombremodulo = $1
        WHERE id = $2
        "#
    )
    .bind(&data.strnombremodulo)
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            HttpResponse::Ok().body("Módulo actualizado")
        }
        Ok(_) => HttpResponse::NotFound().body("Módulo no encontrado"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error al actualizar módulo")
        }
    }
}

//
// 📌 DELETE MODULO
//
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
        Ok(r) if r.rows_affected() > 0 => {
            HttpResponse::Ok().body("Módulo eliminado")
        }
        Ok(_) => HttpResponse::NotFound().body("Módulo no encontrado"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error al eliminar módulo")
        }
    }
}