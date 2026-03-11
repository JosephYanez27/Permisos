use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use std::collections::HashMap;
use crate::models::permisos::{PermisoPerfil, CrearPermisoPerfil};

//
// 📌 GET PERMISOS (Paginado 5 registros)
//
#[get("/permisosperfil/{idperfil}")]
pub async fn get_permisos_por_perfil(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let idperfil = path.into_inner();

    let permisos = sqlx::query_as::<_, PermisoPerfil>(
        r#"
        SELECT 
            m.id as idmodulo,
            m.strnombremodulo as modulo,
            COALESCE(p.idperfil, $1) as idperfil,
            p.id,
            COALESCE(p.bitagregar,false) as bitagregar,
            COALESCE(p.biteditar,false) as biteditar,
            COALESCE(p.bitconsulta,false) as bitconsulta,
            COALESCE(p.biteliminar,false) as biteliminar,
            COALESCE(p.bitdetalle,false) as bitdetalle
        FROM modulo m
        LEFT JOIN permisosperfil p 
            ON p.idmodulo = m.id 
            AND p.idperfil = $1
        ORDER BY m.id
        "#
    )
    .bind(idperfil)
    .fetch_all(pool.get_ref())
    .await;

    match permisos {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error obteniendo permisos"),
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

    let permiso = sqlx::query_as::<_, PermisoPerfil>(
        r#"
        SELECT id, idperfil, idmodulo,
               bitagregar, biteditar, bitconsulta,
               biteliminar, bitdetalle
        FROM permisosperfil
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await;

    match permiso {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => HttpResponse::NotFound().body("Permiso no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener permiso"),
        Err(e) => {
        println!("ERROR SQL: {:?}", e);
        HttpResponse::InternalServerError().body("Error obteniendo permisos")
    }
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

    let result = sqlx::query(
        r#"
        INSERT INTO permisosperfil (
            idperfil, idmodulo,
            bitagregar, biteditar, bitconsulta,
            biteliminar, bitdetalle
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(data.idperfil)
    .bind(data.idmodulo)
    .bind(data.bitagregar)
    .bind(data.biteditar)
    .bind(data.bitconsulta)
    .bind(data.biteliminar)
    .bind(data.bitdetalle)
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
#[put("/permisosperfil")]
pub async fn guardar_permisos(
    pool: web::Data<PgPool>,
    data: web::Json<Vec<PermisoPerfil>>,
) -> HttpResponse {

    for p in data.iter() {

        let _ = sqlx::query(
            r#"
            INSERT INTO permisosperfil(
                idperfil,idmodulo,
                bitagregar,biteditar,
                bitconsulta,biteliminar,bitdetalle
            )
            VALUES($1,$2,$3,$4,$5,$6,$7)
            ON CONFLICT (idperfil,idmodulo)
            DO UPDATE SET
                bitagregar = EXCLUDED.bitagregar,
                biteditar = EXCLUDED.biteditar,
                bitconsulta = EXCLUDED.bitconsulta,
                biteliminar = EXCLUDED.biteliminar,
                bitdetalle = EXCLUDED.bitdetalle
            "#
        )
        .bind(p.idperfil)
        .bind(p.idmodulo)
        .bind(p.bitagregar)
        .bind(p.biteditar)
        .bind(p.bitconsulta)
        .bind(p.biteliminar)
        .bind(p.bitdetalle)
        .execute(pool.get_ref())
        .await;
    }

    HttpResponse::Ok().body("Permisos guardados")
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

    let result = sqlx::query(
        "DELETE FROM permisosperfil WHERE id = $1"
    )
    .bind(id)
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