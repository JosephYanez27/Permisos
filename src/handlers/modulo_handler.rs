use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use std::collections::HashMap;
use crate::models::modulo::{Modulo, CrearModulo};

//
// 📌 GET MODULOS (Paginado)
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

    let limit = 5;
    let offset = (page - 1) * limit;

    let result = sqlx::query_as::<_, Modulo>(
        r#"
        SELECT id, strnombremodulo
        FROM modulo
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(modulos) => HttpResponse::Ok().json(modulos),
        Err(e) => {
            println!("Error obteniendo módulos: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener módulos")
        }
    }
}

//
// 📌 GET MODULO POR ID
//
#[get("/modulo/{id}")]
pub async fn get_modulo_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query_as::<_, Modulo>(
        r#"
        SELECT id, strnombremodulo
        FROM modulo
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(modulo)) => HttpResponse::Ok().json(modulo),
        Ok(None) => HttpResponse::NotFound().body("Módulo no encontrado"),
        Err(e) => {
            println!("Error obteniendo módulo: {:?}", e);
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
        return HttpResponse::BadRequest().body("Nombre obligatorio");
    }

    let result = sqlx::query_as::<_, Modulo>(
        r#"
        INSERT INTO modulo (strnombremodulo)
        VALUES ($1)
        RETURNING id, strnombremodulo
        "#
    )
    .bind(&data.strnombremodulo)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(modulo) => HttpResponse::Ok().json(modulo),
        Err(e) => {
            println!("Error creando módulo: {:?}", e);
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

    let result = sqlx::query_as::<_, Modulo>(
        r#"
        UPDATE modulo
        SET strnombremodulo = $1
        WHERE id = $2
        RETURNING id, strnombremodulo
        "#
    )
    .bind(&data.strnombremodulo)
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(modulo)) => HttpResponse::Ok().json(modulo),
        Ok(None) => HttpResponse::NotFound().body("Módulo no encontrado"),
        Err(e) => {
            println!("Error actualizando módulo: {:?}", e);
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
            println!("Error eliminando módulo: {:?}", e);
            HttpResponse::InternalServerError().body("Error al eliminar módulo")
        }
    }
}